use std::{
    collections::{BTreeMap, HashSet},
    env,
    ffi::OsString,
    fs::{self, File},
    io::{self, Write},
    path::{Path, PathBuf},
};

#[derive(Clone)]
struct McuInputs {
    pub atdf_path: PathBuf,
    pub patch_path: PathBuf,
}

struct GenerationDirs {
    pub unpatched_svd: PathBuf,
    pub patched_svd: PathBuf,
    pub module: PathBuf,
}

fn main() {
    let gen_dirs = match get_create_gen_dirs() {
        Ok(d) => d,
        Err(e) => {
            println!("cargo::error={}", e);
            return;
        }
    };

    let available_mcus = match setup() {
        Ok(d) => d,
        Err(e) => {
            println!("cargo::error={}", e);
            return;
        }
    };

    let all_mcus = available_mcus
        .keys()
        .map(|s| s.to_owned())
        .collect::<Vec<_>>();
    let selected_mcus = available_mcus
        .into_iter()
        .filter(|(mcu, _)| env::var_os(format!("CARGO_FEATURE_{}", mcu.to_uppercase())).is_some())
        .collect::<BTreeMap<String, McuInputs>>();
    if selected_mcus.is_empty() {
        println!(
            "cargo::error=No MCU feature selected! \
            The avr-device crate requires at least one to be enabled in order \
            to know your target's peripherals. Currently available are:\n\
            {}",
            all_mcus.join("\n"),
        );
        return;
    }

    let Ok(generated_mcus) = build_mcu_modules(&gen_dirs, &selected_mcus) else {
        return;
    };

    let _ = generate_vector_macro(&gen_dirs, &generated_mcus);
}

fn get_create_gen_dirs() -> Result<GenerationDirs, std::io::Error> {
    let out_dir = PathBuf::from(env::var_os("OUT_DIR").unwrap());
    let svd_dir = out_dir.join("svd");
    let unpatched_svd = svd_dir.join("unpatched");
    let patched_svd = svd_dir.join("patched");
    let patch = out_dir.join("patch");
    let module = out_dir.join("pac");

    for dir in [&unpatched_svd, &patched_svd, &patch, &module] {
        if !dir.is_dir() {
            fs::create_dir_all(dir)?;
        }
    }

    Ok(GenerationDirs {
        unpatched_svd,
        patched_svd,
        module,
    })
}

fn track_dir(dir: &Path) -> Result<(), std::io::Error> {
    let contents = fs::read_dir(dir)?;
    for result in contents {
        let entry: std::fs::DirEntry = result?;
        let ty = entry.file_type()?;
        let path = entry.path();
        if ty.is_file() {
            println!("cargo::rerun-if-changed={}", path.display());
        } else if ty.is_dir() {
            track_dir(&entry.path())?;
        }
    }
    Ok(())
}

fn setup() -> Result<BTreeMap<String, McuInputs>, std::io::Error> {
    let crate_root = env::var_os("CARGO_MANIFEST_DIR").unwrap();
    let packs_dir = Path::new(&crate_root).join("vendor");
    let patches_dir = Path::new(&crate_root).join("patch");

    track_dir(&patches_dir)?;

    let pack_files = fs::read_dir(&packs_dir)?;

    let mut map = BTreeMap::new();
    for result in pack_files {
        let entry = result?;
        let atdf_path = entry.path();
        if atdf_path.extension() != Some(&OsString::from("atdf")) {
            continue;
        }
        let mcu_name = atdf_path
            .file_stem()
            .and_then(|f| f.to_str())
            .expect("ATDF file must be named after its MCU and str-compatible!")
            .to_owned();
        let patch_path = patches_dir.join(&mcu_name).with_extension("yaml");
        println!("cargo::rerun-if-changed={}", atdf_path.display());
        map.insert(
            mcu_name,
            McuInputs {
                atdf_path,
                patch_path,
            },
        );
    }
    Ok(map)
}

fn build_mcu_modules(
    gen_dirs: &GenerationDirs,
    mcus: &BTreeMap<String, McuInputs>,
) -> Result<BTreeMap<String, PathBuf>, ()> {
    let mut map = BTreeMap::new();
    for (mcu, inputs) in mcus {
        let unpatched_svd_path = gen_dirs.unpatched_svd.join(mcu).with_extension("svd");
        let patched_svd_path = gen_dirs.patched_svd.join(mcu).with_extension("svd");
        let module_path = gen_dirs.module.join(mcu).with_extension("rs");

        let atdf_reader = File::open(&inputs.atdf_path).unwrap();
        let Ok(atdf_parsed) = atdf2svd::atdf::parse(atdf_reader, &mut HashSet::new()) else {
            println!("cargo::error=Failed to parse ATDF for {}!", mcu);
            return Err(());
        };
        let unpatched_writer = match File::create(&unpatched_svd_path) {
            Ok(w) => w,
            Err(e) => {
                println!(
                    "cargo::error=Failed to open unpatched SVD file for {}!\n{}",
                    mcu, e
                );
                return Err(());
            }
        };
        match atdf2svd::svd::generate(&atdf_parsed, unpatched_writer) {
            Ok(()) => {}
            Err(_) => {
                println!(
                    "cargo::error=Failed to generate unpatched SVD file for {}!",
                    mcu
                );
                return Err(());
            }
        };

        // Apply svdtools' patches.
        let svd_content;
        match inputs.patch_path.try_exists() {
            Ok(true) => {
                // Point the patch file to our generated svd and apply the patch.
                let Ok(unpatched_svd_file) = File::open(unpatched_svd_path) else {
                    println!("cargo::error=Failed to open unpatched svd for {}!", mcu);
                    return Err(());
                };
                let svd_patch = match svdtools::patch::load_patch(&inputs.patch_path) {
                    Ok(patch) => patch,
                    Err(e) => {
                        println!(
                            "cargo::error=Failed to open and parse patch for {}! Error: {}",
                            mcu, e
                        );
                        return Err(());
                    }
                };
                let Ok(patched_svd_reader) = svdtools::patch::process_reader(
                    unpatched_svd_file,
                    &svd_patch,
                    &Default::default(),
                    &Default::default(),
                ) else {
                    println!("cargo::error=Failed to patch svd for {}!", mcu);
                    return Err(());
                };

                let Ok(svd) = io::read_to_string(patched_svd_reader) else {
                    println!("cargo::error=Failed to read svd to string for {}!", mcu);
                    return Err(());
                };

                let Ok(()) = fs::write(&patched_svd_path, &svd) else {
                    println!("cargo::error=Failed to write patched svd for {}!", mcu);
                    return Err(());
                };

                svd_content = Ok(svd);
                map.insert(mcu.to_owned(), patched_svd_path);
            }
            Ok(false) => {
                // No patching needed, just continue with unpatched.
                svd_content = fs::read_to_string(&unpatched_svd_path);
                map.insert(mcu.to_owned(), unpatched_svd_path);
            }
            Err(e) => {
                println!(
                    "cargo::error=Failed to check patch existence for {}!\n{}",
                    mcu, e
                );
                return Err(());
            }
        }
        let svd_content = match svd_content {
            Ok(c) => c,
            Err(e) => {
                println!(
                    "cargo::error=Failed to read patched SVD back for {}!\n{}",
                    mcu, e
                );
                return Err(());
            }
        };
        // Apply svd2rust.
        let mut svd2rust_config = svd2rust::Config::default();
        svd2rust_config.target = svd2rust::Target::None;
        svd2rust_config.generic_mod = true;
        svd2rust_config.make_mod = true;
        svd2rust_config.strict = true;
        svd2rust_config.output_dir = Some(gen_dirs.module.clone());
        svd2rust_config.skip_crate_attributes = true;
        let generated = match svd2rust::generate(&svd_content, &svd2rust_config) {
            Ok(g) => g,
            Err(e) => {
                println!(
                    "cargo::error=Failed to generate rust code for {}!\n{}",
                    mcu, e
                );
                return Err(());
            }
        };

        let mut syntax_tree: syn::File = match syn::parse_str(&generated.lib_rs) {
            Ok(s) => s,
            Err(e) => {
                println!("cargo::error=Failed to parse rust code for {}! {}", mcu, e);
                return Err(());
            }
        };
        syntax_tree.attrs.clear();
        for item in syntax_tree.items.iter_mut() {
            {
                let syn::Item::Static(statik) = item else {
                    continue;
                };
                if &statik.ident.to_string() != "DEVICE_PERIPHERALS" {
                    continue;
                }
            }
            *item = syn::parse_quote! {use super::DEVICE_PERIPHERALS;};
            break;
        }

        let formatted = prettyplease::unparse(&syntax_tree);
        if let Err(e) = fs::write(&module_path, &formatted) {
            println!(
                "cargo::error=Failed to write module file for {}!\n{}",
                mcu, e
            );
            return Err(());
        }
    }

    Ok(map)
}

fn generate_vector_macro(
    gen_dirs: &GenerationDirs,
    mcus: &BTreeMap<String, PathBuf>,
) -> Result<(), ()> {
    let mut devices = BTreeMap::new();
    for (mcu, patched_path) in mcus {
        let d = match svdtools::common::svd_reader::device(&patched_path) {
            Ok(d) => d,
            Err(e) => {
                println!(
                    "cargo::error=Failed to parse SVD device for {}!\n{}",
                    mcu, e
                );
                return Err(());
            }
        };
        devices.insert(mcu.to_owned(), d);
    }
    let mut specific_matchers = Vec::new();
    for (mcu, device) in devices {
        for p in device.peripherals {
            for i in &p.interrupt {
                specific_matchers.push(format!(
                    r#"
        (@{0}, {1}, $it:item) => {{
            #[export_name = "__vector_{2}"]
            $it
        }};"#,
                    mcu, i.name, i.value,
                ));
            }
        }
    }

    let mut file_handle = match File::create(&gen_dirs.module.join("vector.rs")) {
        Ok(h) => h,
        Err(e) => {
            println!("cargo::error=Failed to create vector file!\n{}", e);
            return Err(());
        }
    };
    match writeln!(
        file_handle,
        r#"#[doc(hidden)]
#[macro_export]
macro_rules! __avr_device_trampoline {{
    {}
    (@$mcu:ident, $name:ident, $it:item) => {{
        compile_error!(concat!("Couldn't find interrupt ", stringify!($name), ", for MCU ", stringify!($mcu), "."));
    }}
}}
"#,
        specific_matchers.concat(),
    ) {
        Ok(()) => Ok(()),
        Err(e) => {
            println!("cargo::error=Failed to write vector file!\n{}", e);
            Err(())
        }
    }
}
