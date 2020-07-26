avr-device [![crates.io page](http://meritbadge.herokuapp.com/avr-device)](https://crates.io/crates/avr-device) [![docs.rs](https://docs.rs/avr-device/badge.svg)](https://docs.rs/avr-device) [![Build Status](https://travis-ci.org/Rahix/avr-device.svg?branch=master)](https://travis-ci.org/Rahix/avr-device)
==========
Auto-generated wrappers around registers for AVR microcontrollers.

## Usage
Add the following to `Cargo.toml`:
```toml
[dependencies.avr-device]
version = "0.1.0"
features = ["atmega32u4"]
```

Via the feature you can select which chip you want the register specifications for.  The following list is what is currently supported:
* `atmega1280`
* `atmega8`
* `atmega328p`
* `atmega32u4`
* `atmega64`
* `attiny85`

## Build Instructions
The version on `crates.io` is pre-built.  The following is only necessary when trying to build this crate from source.

You need to have [atdf2svd](https://github.com/Rahix/atdf2svd), [svd2rust](https://github.com/rust-embedded/svd2rust), [form](https://github.com/djmcgill/form), [rustfmt](https://github.com/rust-lang/rustfmt)(for the *nightly* toolchain) and [PyYAML](https://github.com/yaml/pyyaml) installed:
```bash
rustup component add --toolchain nightly rustfmt
cargo install form
cargo install svd2rust
cargo install atdf2svd
pip3 install --user pyyaml
```

Next, clone this repo and build the device definitions:
```bash
git clone https://github.com/Rahix/avr-device
cd avr-device
make
# You can build for just one specific chip using
# make atmega32u4
# I suggest building documentation as well
cargo +nightly doc --features <chip> --open
```

## Internals
*avr-device* is generated using [`atdf2svd`](https://github.com/Rahix/atdf2svd) and [`svd2rust`](https://github.com/rust-embedded/svd2rust).  The vendor-provided *atdf* files can be found in `vendor/`.  The intermediate svd files are patched by `svdpatch.py` (Adapted from [`svdpatch.py`](https://github.com/stm32-rs/stm32-rs/blob/master/scripts/svdpatch.py) in [stm32-rs](https://github.com/stm32-rs/stm32-rs)) with device-dependent patches in `patch/`, mainly to improve undescriptive names and missing descriptions.

### Adding a new Chip
To add a new chip, download the *atdf* from <http://packs.download.atmel.com/> and place it in `vendor/`.  Be sure to name it like the Rust module that should be generated.  Next, you need to integrate it into the base crate and build system.  Follow what was done in commit [290613454fbd ("Add basic support for ATmega64")](https://github.com/Rahix/avr-device/commit/290613454fbdc5e4ac98e53deccaf74dafc88963).  Please adhere to the alphabetical sorting that is present so far.

If patches need to be applied, create a `<chipname>.yaml` in `patch/`. The patching format is compatible to the [YAML patching format of stm32](https://github.com/stm32-rs/stm32-rs/#device-and-peripheral-yaml-format), except that you do not need to provide the path to the svd file (`_svd: ...`) since it is generated in the build process. Additionally, the following patching was added:
```yaml
PERIPHERIAL:
    REGISTER:
        _modify:
          FIELD:
            # If a field already contains a write-constraint, you can
            # change it using the _write_constraint modifier.  The
            # following values are allowed:

            # "enum": Only allow enumerated values (<useEnumeratedValues>)
            _write_constraint: "enum"

            # Remove any existing writeConstraint tag
            _write_constraint: "none"

            # Constrain to a range of values (<range>)
            _write_constraint: [min, max]
        FIELD:
            # Replaces the enumeratedValues definition for this field.
            # If it does not exist yet, it is created.
            _replace_enum:
                NAME: [VALUE, DESCRIPTION]
```

## License
*avr-device* is licensed under either of

 * Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

The vendored *atdf* files are licensed under the Apache License, Version 2.0 ([LICENSE-VENDOR](vendor/LICENSE)).
