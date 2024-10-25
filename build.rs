#![feature(io_error_more)]
#![feature(exit_status_error)]

use atdf2svd::{atdf, svd};
use std::{
    collections::HashSet,
    env,
    fs::{self, File},
    io,
    path::{Path, PathBuf}, process::{Command, Stdio},
};
use svd2rust::config;
use svdtools::patch;

fn main() {
    let mcus = get_available_mcus();
    let mcu = select_mcu(&mcus);
    build_mcu_module(&mcu);
}

fn get_available_mcus() -> Vec<String> {
    let crate_root = env::var_os("CARGO_MANIFEST_DIR").unwrap();
    let packs_dir = Path::new(&crate_root).join("vendor");
    // Reserve a number greater than there are atdf files in ./vendor, so we
    // don't have to extend between iterations. A bit of overshoot is used so
    // people won't have to remember changing this when adding a chip.
    let mut available_mcus = Vec::with_capacity(50);
    for pack_file in fs::read_dir(&packs_dir).unwrap() {
        if let Some(mcu) = pack_file
            .unwrap()
            .file_name()
            .to_str()
            .unwrap()
            .strip_suffix(".atdf")
        {
            available_mcus.push(mcu.to_owned());
        }
    }
    available_mcus
}

fn select_mcu(mcus: &[String]) -> String {
    let mut pack_name = None;
    for mcu in mcus {
        if let Ok(_) = env::var(format!("CARGO_FEATURE_{}", mcu.to_uppercase())) {
            if pack_name.is_none() {
                pack_name = Some(mcu.to_owned());
            } else {
                panic!("More than one MCU feature selected!");
            }
        }
    }
    pack_name
        .expect(&format!(
            "No MCU feature selected! \
         The avr-device crate requires exactly one to be enabled in order to \
         know your target's peripherals. Currently available are:\n\
         {}",
            mcus.join("\n"),
        ))
        .to_owned()
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
    let atdf_reader = File::open(&atdf_file).unwrap();
    let atdf_parsed = match atdf::parse(atdf_reader, &HashSet::<String>::new()) {
        Ok(chip) => chip,
        Err(what) => {
            let _ = what.format(&mut io::stdout());
            panic!("Failed to parse \"{}\"!", atdf_file.display());
        }
    };
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
    let mut svd2rust_config = config::Config::default();
    svd2rust_config.target = config::Target::None;
    svd2rust_config.generic_mod = true;
    svd2rust_config.make_mod = true;
    svd2rust_config.skip_crate_attributes = true;
    svd2rust_config.strict = true;
    svd2rust_config.output_dir = Some(pac_dir.to_owned());
    svd2rust_config.interrupt_link_section = Some(".text.vector".to_owned());
    let generated = svd2rust::generate(&svd_content, &svd2rust_config).unwrap();
    let module_file = pac_dir.join(mcu).with_extension("rs");
    fs::write(&module_file, generated.lib_rs).unwrap();

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
