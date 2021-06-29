avr-device [![crates.io page](http://meritbadge.herokuapp.com/avr-device)](https://crates.io/crates/avr-device) [![docs.rs](https://docs.rs/avr-device/badge.svg)](https://docs.rs/avr-device) ![Continuous Integration](https://github.com/Rahix/avr-device/workflows/Continuous%20Integration/badge.svg)
==========
Auto-generated wrappers around registers for AVR microcontrollers.

## Usage
Add the following to `Cargo.toml`:
```toml
[dependencies.avr-device]
version = "0.3.1"
features = ["atmega32u4"]
```

Via the feature you can select which chip you want the register specifications for.  The following list is what is currently supported:
* `at90usb1286`
* `atmega1280`
* `atmega168`
* `atmega2560`
* `atmega8`
* `atmega8u2`
* `atmega328p`
* `atmega32u4`
* `atmega4809`
* `atmega48p`
* `atmega64`
* `atmega644`
* `attiny84`
* `attiny841`
* `attiny85`
* `attiny861`
* `attiny88`

## Build Instructions
The version on `crates.io` is pre-built.  The following is only necessary when trying to build this crate from source.

You need to have [atdf2svd][], [svd2rust][], [form][], [rustfmt][](for the *nightly* toolchain) and [svdtools][] (>= 0.1.9) installed:
```bash
cargo install atdf2svd
cargo install svd2rust
cargo install form
rustup component add --toolchain nightly rustfmt
pip3 install --user svdtools
```

[atdf2svd]: https://github.com/Rahix/atdf2svd
[svd2rust]: https://github.com/rust-embedded/svd2rust
[form]: https://github.com/djmcgill/form
[rustfmt]: https://github.com/rust-lang/rustfmt
[svdtools]: https://github.com/stm32-rs/svdtools

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
To add a new chip, download the *atdf* from <http://packs.download.atmel.com/> (or [avr-mcu/packs/](https://github.com/avr-rust/avr-mcu/tree/master/packs)) and place it in `vendor/`.  Be sure to name it like the Rust module that should be generated.  Next, you need to integrate it into the base crate and build system.  Follow what was done in commit [290613454fbd ("Add basic support for ATmega64")](https://github.com/Rahix/avr-device/commit/290613454fbdc5e4ac98e53deccaf74dafc88963).  Please adhere to the alphabetical sorting that is present so far.

Next, you **must** create a `<chipname>.yaml` in `patch/` which has at least the following content:
```yaml
_svd: ../svd/<chipname>.svd
```

If more patches need to be applied (most likely!), they should be added into this file as well.  The patching format is documented in the [`svdtools` README](https://github.com/stm32-rs/svdtools#device-and-peripheral-yaml-format).  Ideally, try to reuse the exisiting patches in `patch/common/` or `patch/timer/`.

Finally, try building the crate for your MCU with `make <chipname>`.

## License
*avr-device* is licensed under either of

 * Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

The vendored *atdf* files are licensed under the Apache License, Version 2.0 ([LICENSE-VENDOR](vendor/LICENSE)).
