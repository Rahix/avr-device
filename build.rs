#![feature(io_error_more)]
#![feature(exit_status_error)]

use atdf2svd::{atdf, svd};
use std::{
    collections::{BTreeSet, HashSet},
    env,
    fs::{self, File},
    io::{self, Write},
    path::{Path, PathBuf},
    process::{Command, Stdio},
};
use svdtools::patch;

fn main() {
    println!("cargo::rustc-check-cfg=cfg(single_mcu)");

    let available_mcus = get_available_mcus();
    let mcus = select_mcus(available_mcus);
    if mcus.len() == 1 {
        println!("cargo::rustc-cfg=single_mcu");
    }
    for mcu in &mcus {
        build_mcu_module(mcu);
    }
    generate_vector_macro(mcus);
}

fn get_available_mcus() -> BTreeSet<String> {
    let crate_root = env::var_os("CARGO_MANIFEST_DIR").unwrap();
    let packs_dir = Path::new(&crate_root).join("vendor");
    fs::read_dir(&packs_dir)
        .unwrap()
        .filter_map(|f| f.ok())
        .filter_map(|entry| {
            entry
                .file_name()
                .to_str()
                .and_then(|fstr| fstr.strip_suffix(".atdf"))
                .map(|mcu| mcu.to_owned())
        })
        .collect()
}

fn select_mcus(mcus: BTreeSet<String>) -> BTreeSet<String> {
    let pack_names = mcus
        .iter()
        .filter_map(|mcu| {
            env::var(format!("CARGO_FEATURE_{}", mcu.to_uppercase()))
                .and_then(|_| Ok(mcu.clone()))
                .ok()
        })
        .collect::<BTreeSet<_>>();

    if pack_names.is_empty() {
        panic!(
            "No MCU feature selected! \
            The avr-device crate requires one to be enabled in order to \
            know your target's peripherals. Currently available are:\n\
            {}",
            mcus.into_iter().collect::<Vec<_>>().join("\n"),
        )
    }

    pack_names
}

fn build_mcu_module(mcu: &str) {
    // Register our inputs with Cargo.
    let crate_root = PathBuf::from(env::var_os("CARGO_MANIFEST_DIR").unwrap());
    let atdf_file = crate_root.join("vendor").join(format!("{}.atdf", mcu));
    println!("cargo::rerun-if-changed={}", atdf_file.display());
    let patch_dir = crate_root.join("patch");
    let patch_file = patch_dir.join(format!("{}.yaml", mcu));
    println!("cargo::rerun-if-changed={}", patch_file.display());

    // Get a temporary directory to work inside.
    let temp_dir = env::temp_dir().join(format!("avr-device-build-{}", mcu));
    ensure_directory(&temp_dir);

    // Apply atdf2svd.
    let atdf_parsed = get_atdf_parsed(mcu);
    let svd_file = temp_dir.join("unpatched.svd");
    let svd_writer = File::create(&svd_file).unwrap();
    if let Err(what) = svd::generate(&atdf_parsed, svd_writer) {
        let _ = what.format(&mut io::stdout());
        panic!("Failed to generate \"{}\"!", svd_file.display());
    }

    let svd_content;
    if let Ok(true) = patch_file.try_exists() {
        // Point the patch file to our generated svd and apply the patch.
        let includer_content = format!(
            r#"_svd: {}

_include:
- {}
"#,
            svd_file.display(),
            patch_file.display()
        );
        let includer_file = temp_dir.join("patch.yaml");
        fs::write(&includer_file, &includer_content).unwrap();
        let svd_patched_file = temp_dir.join("patched.svd");
        patch::process_file(
            &includer_file,
            Some(&svd_patched_file),
            None,
            &Default::default(),
        )
        .unwrap();

        // Read the contents after patching.
        svd_content = fs::read_to_string(&svd_patched_file).unwrap();
    } else {
        // No patching needed, just read the svd.
        svd_content = fs::read_to_string(&svd_file).unwrap();
    }

    // Apply svd2rust.
    let out_dir = env::var("OUT_DIR").unwrap();
    let pac_dir = Path::new(&out_dir).join("pac");
    ensure_directory(&pac_dir);
    let mut svd2rust_config = svd2rust::Config::default();
    svd2rust_config.target = svd2rust::Target::None;
    svd2rust_config.generic_mod = true;
    svd2rust_config.make_mod = true;
    svd2rust_config.skip_crate_attributes = true;
    svd2rust_config.strict = true;
    svd2rust_config.output_dir = Some(pac_dir.to_owned());
    svd2rust_config.interrupt_link_section = Some(".text.vector".to_owned());
    let generated = svd2rust::generate(&svd_content, &svd2rust_config).unwrap();

    // HACK: Using simple pattern replacement like this is likely to break
    // with any formatting changes in the input. Ideally we'd parse it somehow,
    // but that'd increase compile time. Perhaps reopening the file after
    // rustfmt runs and patching that would be less fickle.
    let pat = "# [no_mangle] static mut DEVICE_PERIPHERALS : bool = false ;";
    let lib_rs = generated
        .lib_rs
        .replace(pat, "use super::DEVICE_PERIPHERALS;");
    let module_file = pac_dir.join(mcu).with_extension("rs");
    fs::write(&module_file, lib_rs).unwrap();

    let status = Command::new("rustfmt")
        .arg(module_file.to_str().unwrap())
        .stdin(Stdio::null())
        .stdout(Stdio::null())
        .status()
        .unwrap();
    if let Err(what) = status.exit_ok() {
        panic!("rustfmt failed to format generated source with: {}", what);
    }
}

fn ensure_directory(dir: &Path) {
    if dir.is_dir() {
        return;
    } else if dir.try_exists().unwrap() {
        println!(
            "cargo::warning=A non-directory exists in \"{0}\", it will be moved to \"{0}.before\" and a directory created in its place.",
            dir.display()
        );
        fs::rename(&dir, dir.with_extension("before")).unwrap();
    }

    fs::create_dir(dir).unwrap();
}

fn get_atdf_parsed(mcu: &str) -> atdf2svd::chip::Chip {
    let crate_root = PathBuf::from(env::var_os("CARGO_MANIFEST_DIR").unwrap());
    let atdf_file = crate_root.join("vendor").join(format!("{}.atdf", mcu));

    let atdf_reader = File::open(&atdf_file).unwrap();
    let atdf_parsed = match atdf::parse(atdf_reader, &HashSet::<String>::new()) {
        Ok(chip) => chip,
        Err(what) => {
            let _ = what.format(&mut io::stdout());
            panic!("Failed to parse \"{}\"!", atdf_file.display());
        }
    };

    atdf_parsed
}

fn generate_vector_macro(mcus: BTreeSet<String>) {
    let specific_matchers = mcus
        .iter()
        .flat_map(|mcu| {
            get_atdf_parsed(mcu)
                .interrupts
                .into_iter()
                .map(move |(_, value)| {
                    format!(
                        r#"
        (@{0}, {1}, $it:item) => {{
            #[export_name = "__vector_{2}"]
            $it
        }};"#,
                        mcu, value.name, value.index
                    )
                })
        })
        .collect::<Vec<_>>();
    let single_expansion = if mcus.len() == 1 {
        &format!(
            "$crate::__avr_device_trampoline!(@{}, $name, $it);",
            mcus.first().unwrap(),
        )
    } else {
        "compile_error!(\"More than one MCU feature selected. \
         To use interrupts like this, you must call #[interrupt(chip)] with the \
         chip argument as your MCU, using default is not allowed. \
         Note this is rarely what you want, you should probably enable only one \
         MCU.\");"
    };

    let mut file_handle = File::create(
        Path::new(&env::var("OUT_DIR").unwrap())
            .join("pac")
            .join("vector.rs"),
    )
    .unwrap();
    writeln!(
        file_handle,
        r#"#[doc(hidden)]
#[macro_export]
macro_rules! __avr_device_trampoline {{
    {}
    (@$mcu:ident, $name:ident, $it:item) => {{
        compile_error!(concat!("Couldn't find interrupt ", stringify!($name), ", for MCU ", stringify!($mcu), "."));
    }}
}}

#[doc(hidden)]
#[macro_export]
macro_rules! __avr_device_trampoline_single {{
    (@$name:ident, $it:item) => {{
        {}
    }};
}}
"#,
        specific_matchers.concat(),
        single_expansion,
    )
    .unwrap();
}
