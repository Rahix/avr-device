avr-device [![crates.io page](https://img.shields.io/crates/v/avr-device.svg)](https://crates.io/crates/avr-device) [![docs.rs](https://docs.rs/avr-device/badge.svg)](https://docs.rs/avr-device) ![Continuous Integration](https://github.com/Rahix/avr-device/workflows/Continuous%20Integration/badge.svg)
==========
Auto-generated wrappers around registers for AVR microcontrollers.

## Usage
Add the following to `Cargo.toml`:
```toml
[dependencies.avr-device]
version = "0.8.0"
features = ["atmega32u4"]
```

With the feature you can select which chip you want the register definitions
for.  For a full list of supported AVR chips, please check
[`Cargo.toml`](https://github.com/rahix/avr-device/blob/main/Cargo.toml#L30).

## Build Instructions
The PACs (Peripheral Access Crates, or really modules, in our case) **are not**
checked into git. Rather, we generate them at build time, via an automated
process implemented in [`build.rs`](./build.rs). It takes the ATDF files
Microchip (former Atmel) provides plus some patches of our own making as inputs,
and outputs a module generated from those device descriptions. These inputs
**are** checked-in. The process is similar to what the `*bindgen` crates
provide, just has more steps. So, in short, building should be a matter of
selecting the features and running cargo.

### Adding a new Chip
To add a new chip:

1. Download the ATDF from <http://packs.download.atmel.com/> and place it in
   `vendor/`. Be sure to name it like the Rust module that should be generated.
2. Add a feature of the same name to `Cargo.toml` (it should enable
   `device-selected`);
3. Add any needed patches to a yaml file with the same name under the `patch`
   directory, ideally by including some of the snippets present in
   `patch/common` and `patch/timer`; The format is decribed
   [here](https://github.com/rust-embedded/svdtools#device-and-peripheral-yaml-format),
   but it should not include the top-level `_svd` key, as that's handled by the
   build system; If patching is unneeded (it's almost always needed!), the file
   can be omitted.
4. Include the module into the tree, in [`devices.rs`](./src/devices.rs),
   following the format used by other modules in that file;
5. Update [`lib.rs`](./src/lib.rs) to conditionally `use` the new MCU module,
   and add it to the lists of selected and available MCUs in the doc comment.
6. Finally, try building the crate for your MCU with
   `cargo build --features <mcu>,rt`.
7. Also check the built documentation for inconsistencies, via
   `cargo doc --features <mcu>,rt --open` (it will pop up in your browser).
8. Update this README.md, adding the MCU to the table.
   
## Internals
Since the vendor does not provide SVDs we can pass to [`svd2rust`][], we
generate one via [`atdf2svd`][]. The sequence is as follows:

1. Check which MCUs are known to the crate
   ([build.rs:get_available_mcus](./build.rs));
2. Select which to build for by checking enabled features
   ([build.rs:select_mcu](./build.rs));
3. Generate the Rust module ([build.rs:build_mcu_module](./build.rs));
   
   Substeps are:
   1. Register inputs with cargo;
   2. Get a temporary directory;
   3. Apply `atdf2svd`;
   4. If a yaml patch exists, use it via [`svdtools`][] and read the new content
      / else, read the content of the unpatched file to continue;
   5. Get the output directory;
   6. Apply `svd2rust`;
   7. Run [`prettyplease`][] on the module to make it readable in [`docs.rs`][];
4. It will be included from `$OUT_DIR/pac/<mcu>.rs` into the path
   `avr_device::devices::<mcu>` (private), and re-exported as
   `avr_device::<mcu>` (public).

[`atdf2svd`]: https://github.com/Rahix/atdf2svd
[`svd2rust`]: https://github.com/rust-embedded/svd2rust
[`svdtools`]: https://github.com/rust-embedded/svdtools
[`prettyplease`]: https://github.com/dtolnay/prettyplease
[`docs.rs`]: https://docs.rs/avr-device/latest/avr_device

## License
*avr-device* is licensed under either of

 * Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

The vendored *atdf* files are licensed under the Apache License, Version 2.0 ([LICENSE-VENDOR](vendor/LICENSE)).
