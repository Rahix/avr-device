use anyhow::Context;
use std::{
    collections::{BTreeMap, HashSet},
    env,
    ffi::OsString,
    fs::{self, File},
    io::{self, Read},
    path::{Path, PathBuf},
};
use svd2rust::config::IdentFormats;
use svd_rs::Device;

fn main() -> anyhow::Result<()> {
    let mut cargo_directives = vec![];

    let crate_root = PathBuf::from(env::var_os("CARGO_MANIFEST_DIR").ok_or(anyhow::anyhow!(
        "env variable CARGO_MANIFEST_DIR is missing"
    ))?);
    let out_dir = PathBuf::from(
        env::var_os("OUT_DIR").ok_or(anyhow::anyhow!("env variable OUT_DIR is missing"))?,
    );

    let svd_generator =
        SvdGenerator::from_out_dir(&out_dir).context("failed to create SVD output directory")?;
    let code_generator = CodeGenerator::from_out_dir(&out_dir)
        .context("failed to create codegen output directory")?;

    let mcu_inputs = InputFinder::find(&crate_root, &mut cargo_directives)
        .context("failed finding and selecting MCU inputs")?;
    let mut svds = BTreeMap::new();
    for (mcu, inputs) in mcu_inputs.get_map() {
        let _ = svds.insert(
            mcu.as_str(),
            svd_generator
                .generate(mcu, inputs)
                .context(format!("failed to generate SVD for MCU {}", mcu))?,
        );
    }
    for (mcu, svd) in &svds {
        code_generator
            .generate_module(mcu, svd)
            .context(format!("failed to generate PAC code for MCU {}", mcu))?;
    }
    code_generator
        .generate_vector(&svds)
        .context("failed to generate interrupt vector definitions")?;

    for directive in cargo_directives {
        println!("{}", directive);
    }

    Ok(())
}

#[derive(Debug)]
struct McuInputs {
    atdf_path: PathBuf,
    atdf: atdf2svd::chip::Chip,
    patch: Option<yaml_rust2::Yaml>,
}

#[derive(Debug)]
struct InputFinder {
    inputs: BTreeMap<String, McuInputs>,
}

impl InputFinder {
    pub fn get_map(&self) -> &BTreeMap<String, McuInputs> {
        &self.inputs
    }

    pub fn find(crate_root: &Path, cargo_directives: &mut Vec<String>) -> anyhow::Result<Self> {
        let packs_dir = crate_root.join("vendor");
        let patches_dir = crate_root.join("patch");

        Self::track_path(&packs_dir, cargo_directives)?;
        Self::track_path(&patches_dir, cargo_directives)?;

        let mut inputs = BTreeMap::new();
        let mut all_mcus = Vec::new();
        for result in fs::read_dir(&packs_dir)
            .map_err(io_error_in_path(&packs_dir))
            .context("could not scan vendor/ directory")?
        {
            let entry = result
                .map_err(io_error_in_path(&packs_dir))
                .context("could not read entry from vendor/ directory")?;
            let atdf_path = entry.path();
            if atdf_path.extension() != Some(&OsString::from("atdf")) {
                continue;
            }
            let mcu_name = atdf_path
                .file_stem()
                .ok_or(anyhow::anyhow!(
                    "file with no stem in `{}`",
                    atdf_path.display()
                ))?
                .to_str()
                .ok_or(anyhow::anyhow!(
                    "file with non UTF-8 name in `{}`",
                    atdf_path.display()
                ))?
                .to_owned();
            all_mcus.push(mcu_name.clone());
            if env::var_os(format!("CARGO_FEATURE_{}", mcu_name.to_uppercase())).is_none() {
                continue;
            }

            let atdf_file = File::open(&atdf_path)
                .map_err(io_error_in_path(&atdf_path))
                .context("could not open ATDF file")?;
            let atdf = atdf2svd::atdf::parse(atdf_file, &mut HashSet::new())
                .map_err(atdf_error(&atdf_path))?;
            let patch_path = patches_dir.join(&mcu_name).with_extension("yaml");
            let patch = if patch_path
                .try_exists()
                .map_err(io_error_in_path(&patch_path))
                .context("could not test for patch path existence")?
            {
                Some(
                    svdtools::patch::load_patch(&patch_path)
                        .context("failed to parse patch YAML")?,
                )
            } else {
                None
            };
            inputs.insert(
                mcu_name,
                McuInputs {
                    atdf_path,
                    atdf,
                    patch,
                },
            );
        }
        if inputs.is_empty() {
            return Err(anyhow::anyhow!(
                "at least one MCU feature must be selected; choose from {}",
                all_mcus.join(", ")
            ))
            .context("no crate-features for MCUs were selected");
        }

        Ok(Self { inputs })
    }

    fn track_path(path: &Path, cargo_directives: &mut Vec<String>) -> anyhow::Result<()> {
        if !path
            .try_exists()
            .map_err(io_error_in_path(path))
            .context("could not test for path existence")?
        {
            return Err(anyhow::anyhow!(
                "required path `{}` is missing",
                path.display()
            ));
        } else if path.is_symlink() {
            return Err(anyhow::anyhow!(
                "required path `{}` being a symlink is invalid",
                path.display()
            ));
        }

        if path.is_dir() {
            for result in fs::read_dir(path)
                .map_err(io_error_in_path(path))
                .context("failed to scan directory")?
            {
                let subpath = result
                    .map_err(io_error_in_path(path))
                    .context("failed to read scanned directory entry")?
                    .path();
                Self::track_path(&subpath, cargo_directives)?;
            }
        }

        cargo_directives.push(format!(
            "cargo::rerun-if-changed={}",
            path.to_str().ok_or(anyhow::anyhow!(
                "file with non UTF-8 name in `{}`",
                path.display()
            ))?
        ));

        Ok(())
    }
}

#[derive(Debug)]
struct SvdGenerator {
    unpatched_svd: PathBuf,
    patched_svd: PathBuf,
}

impl SvdGenerator {
    pub fn from_out_dir(out_dir: &Path) -> anyhow::Result<Self> {
        let svd_dir = out_dir.join("svd");
        let unpatched_svd = svd_dir.join("unpatched");
        let patched_svd = svd_dir.join("patched");

        ensure_out_dir(&unpatched_svd)?;
        ensure_out_dir(&patched_svd)?;

        Ok(Self {
            unpatched_svd,
            patched_svd,
        })
    }

    pub fn generate(&self, mcu: &str, inputs: &McuInputs) -> anyhow::Result<Device> {
        let unpatched_svd_path = self.unpatched_svd.join(mcu).with_extension("svd");
        let unpatched_writer = File::create(&unpatched_svd_path)
            .map_err(io_error_in_path(&unpatched_svd_path))
            .context("failed to create file for unpatched SVD")?;
        atdf2svd::svd::generate(&inputs.atdf, &unpatched_writer)
            .map_err(atdf_error(&inputs.atdf_path))
            .context("failed to generate SVD from ATDF")?;
        let unpatched_reader = File::open(&unpatched_svd_path)
            .map_err(io_error_in_path(&unpatched_svd_path))
            .context("failed to open unpatched SVD for reading")?;
        let patched_svd_path = self.patched_svd.join(mcu).with_extension("svd");
        let svd_path = if let Some(patch) = &inputs.patch {
            let mut reader = svdtools::patch::process_reader(
                unpatched_reader,
                &patch,
                &Default::default(),
                &Default::default(),
            )
            .context("failed to start SVD patcher")?;
            let mut b = Vec::new();
            reader
                .read_to_end(&mut b)
                .map_err(io_error_in_path(&patched_svd_path))
                .context("failed to read patched SVD from patcher process")?;

            fs::write(&patched_svd_path, b)
                .map_err(io_error_in_path(&patched_svd_path))
                .context("failed to write patched SVD")?;
            patched_svd_path
        } else {
            unpatched_svd_path
        };

        let svd_str = fs::read_to_string(&svd_path)
            .map_err(io_error_in_path(&svd_path))
            .context("failed to read final SVD")?;

        svd2rust::load_from(&svd_str, &svd2rust::Config::default())
            .context("failed to parse SVD with svd2rust")
    }
}

#[derive(Debug)]
struct CodeGenerator {
    module: PathBuf,
}

impl CodeGenerator {
    pub fn generate_module(&self, mcu: &str, svd: &Device) -> anyhow::Result<()> {
        let mut svd2rust_config = svd2rust::Config::default();
        svd2rust_config.target = svd2rust::Target::None;
        svd2rust_config.generic_mod = true;
        svd2rust_config.make_mod = true;
        svd2rust_config.strict = true;
        svd2rust_config.output_dir = Some(self.module.clone());
        svd2rust_config.skip_crate_attributes = true;
        svd2rust_config.reexport_core_peripherals = false;
        svd2rust_config.reexport_interrupt = false;
        svd2rust_config.ident_formats = IdentFormats::default_theme();

        let generated_stream =
            svd2rust::generate::device::render(&svd, &svd2rust_config, &mut String::new())
                .context("failed to generate svd2rust module")?;

        let mut syntax_tree: syn::File = syn::parse2(generated_stream)
            .map_err(|e| anyhow::anyhow!("{}", e))
            .context("failed to parse svd2rust module code")?;

        self.patch_device_peripherals_singleton(&mut syntax_tree)
            .context("failed to patch svd2rust module code")?;

        let formatted = prettyplease::unparse(&syntax_tree);

        let module_path = self.module.join(mcu).with_extension("rs");
        fs::write(&module_path, &formatted)
            .map_err(io_error_in_path(&module_path))
            .context("failed to write svd2rust module code to file")
    }

    fn patch_device_peripherals_singleton(
        &self,
        syntax_tree: &mut syn::File,
    ) -> anyhow::Result<()> {
        for item in syntax_tree.items.iter_mut() {
            if let syn::Item::Static(statik) = item {
                if statik.ident.to_string() == "DEVICE_PERIPHERALS" {
                    *item = syn::parse_quote! {use super::DEVICE_PERIPHERALS;};
                    return Ok(());
                }
            }
        }
        Err(anyhow::anyhow!(
            "Could not find `DEVICE_PERIPHERALS` static"
        ))
    }

    fn generate_vector(&self, devices: &BTreeMap<&str, Device>) -> anyhow::Result<()> {
        let mut specific_matchers = Vec::new();
        for (mcu, device) in devices {
            for p in &device.peripherals {
                for i in &p.interrupt {
                    specific_matchers.push(format!(
                        r#"(@{0}, {1}, $it:item) => {{
        #[export_name = "__vector_{2}"]
        $it
    }};"#,
                        mcu, i.name, i.value,
                    ));
                }
            }
        }

        let content = format!(
            r#"#[doc(hidden)]
#[macro_export]
macro_rules! __avr_device_trampoline {{
    {}
    (@$mcu:ident, $name:ident, $it:item) => {{
        compile_error!(concat!("Couldn't find interrupt ", stringify!($name), ", for MCU ", stringify!($mcu), "."));
    }}
}}
"#,
            specific_matchers.join("\n    "),
        );
        let vector_path = self.module.join("vector.rs");
        fs::write(&vector_path, content)
            .map_err(io_error_in_path(&vector_path))
            .context("failed to write vector module to file")
    }

    pub fn from_out_dir(out_dir: &Path) -> anyhow::Result<Self> {
        let module = out_dir.join("pac");
        ensure_out_dir(&module).context("failed preparing PAC directory")?;
        Ok(Self { module })
    }
}

fn ensure_out_dir(dir: &Path) -> anyhow::Result<()> {
    if dir.is_file() || dir.is_symlink() {
        return Err(anyhow::anyhow!(
            "path `{}` not being a directory is invalid",
            dir.display()
        ));
    }
    if !dir.is_dir() {
        fs::create_dir_all(dir)
            .map_err(io_error_in_path(dir))
            .context("failed creating directory")?;
    }
    Ok(())
}

fn atdf_error(path: &Path) -> impl Fn(atdf2svd::Error) -> anyhow::Error {
    let path = path.to_owned();
    move |e: atdf2svd::Error| {
        let mut v = Vec::new();
        if let Err(e) = e.format(&mut v).map_err(io_error_in_path(&path)) {
            return e;
        }
        let s = v.utf8_chunks().fold(String::new(), |mut s, chunk| {
            s.push_str(chunk.valid());
            s
        });
        anyhow::anyhow!("{}", s)
    }
}

fn io_error_in_path(p: &Path) -> impl Fn(io::Error) -> anyhow::Error {
    let p = p.to_owned();
    move |e| anyhow::anyhow!("{}: {}", p.display(), e)
}
