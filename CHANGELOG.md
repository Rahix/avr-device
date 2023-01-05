# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]
### Added
- Support for `ATtiny404` ([#111]).
- Support for the [`critical-section`] crate via a `critical-section-impl`
  feature flag ([#116]).

### Changed
- Patched registers for `ATmega8` ([#112]).

[#111]: https://github.com/Rahix/avr-device/pull/111
[#112]: https://github.com/Rahix/avr-device/pull/112
[#116]: https://github.com/Rahix/avr-device/pull/116
[`critical-section`]: https://crates.io/crates/critical-section


## [0.4.0] - 2022-09-08
### Added
- Support for `ATmega164PA` ([#101]).
- Added a new, safer, API for manually managing interrupts ([#104]).

### Changed
- Fixed timer registers for `ATtiny167` ([#102]).
- Improved codegen of the interrupt management functions ([#104]).
- Updated to `bare-metal` version 1.0.0.  This changes the
  `interrupt::free()` function slightly.  Please check the docs for
  details ([#108]).

### Removed
- Support for very old `rustc` versions which need `llvm_asm!()` ([#106], [#109]).

[#101]: https://github.com/Rahix/avr-device/pull/101
[#102]: https://github.com/Rahix/avr-device/pull/102
[#104]: https://github.com/Rahix/avr-device/pull/104
[#106]: https://github.com/Rahix/avr-device/pull/106
[#108]: https://github.com/Rahix/avr-device/pull/108
[#109]: https://github.com/Rahix/avr-device/pull/109


## [0.3.4] - 2022-06-23
### Added
- Support for `ATtiny13A` ([#100]).

### Changed
- Fixed timer registers for `ATmega1280` ([#99]).

[#99]: https://github.com/Rahix/avr-device/pull/99
[#100]: https://github.com/Rahix/avr-device/pull/100


## [0.3.3] - 2022-05-10
### Added
- Support for `ATtiny1614` ([#90]).
- Support for `ATmega128RFA1` ([#93]).
- Support for `ATmega1284P` ([#94]).

### Fixed
- `avr-device` now compiles with newer rustc versions (> 1.59) where the
  `llvm_asm!()` macro was removed in favor of `asm!()` ([#97]).

[#90]: https://github.com/Rahix/avr-device/pull/90
[#93]: https://github.com/Rahix/avr-device/pull/93
[#94]: https://github.com/Rahix/avr-device/pull/94
[#97]: https://github.com/Rahix/avr-device/pull/97


## [0.3.2] - 2021-10-15
### Added
- Support for `ATtiny202` ([#82]).
- Support for `ATtiny167` ([#84]).
- Support for `ATtiny2313` and `ATtiny2313A` ([#85]).

#### Changed
- Upgraded to `svd2rust` version 0.19 ([#86]).  Please check the upstream
  changelog for details about what changes this has lead to.
- `interrupt::disable()` now returns a boolean, indicating whether interrupts
  were previously enabled ([#89]).

### Fixed
- Fixed enumerated values for `TWAA` field in `ATtiny841` ([`c0db0422b9ca`]).

[#82]: https://github.com/Rahix/avr-device/pull/82
[#84]: https://github.com/Rahix/avr-device/pull/84
[#85]: https://github.com/Rahix/avr-device/pull/85
[#86]: https://github.com/Rahix/avr-device/pull/86
[#89]: https://github.com/Rahix/avr-device/pull/89
[`c0db0422b9ca`]: https://github.com/Rahix/avr-device/commit/c0db0422b9ca8c7ff4cef39807b05f1cfca26028


## [0.3.1] - 2021-06-29
### Added
- Support for `ATmega8U2` ([#77]).
- Support for `AT90USB1286` ([#80]).
- Convert `static mut`s into `&mut` references inside `#[entry]` and interrupt
  handlers.  This behavior mirrors what `cortex-m-rt` does ([#79]).

[#77]: https://github.com/Rahix/avr-device/pull/77
[#79]: https://github.com/Rahix/avr-device/pull/79
[#80]: https://github.com/Rahix/avr-device/pull/80


## [0.3.0] - 2021-02-07
### Added
- Support for `ATmega4809` ([#63]).
- Support for `ATtiny841` and `ATtiny861` ([#67]).

### Changed
- **BREAKING**: Updated the enumerated values for the SPI clock
  prescaler field ([#64]).
- **BREAKING**: Split the `EIMSK` register fields into per-bit fields on
  `ATmega328P` and `ATmega328PB` ([#74]).
- Switched to using the `svd interrupts` command for extracting
  a list of interrupts instead of parsing generated rust source
  files for this ([#69]).

### Fixed
- `ATmega64`: Fixed wrong field size for `UMSELn` fields in `USART` peripherals ([#64]).
- (via [`atdf2svd` v0.2.0]): Removed invalid enumerated values from all fields.
- Fixed an error in the 16-bit timer documentation ([#67]).
- Fixed the `PRR`, `PRR0`, and `PRR1` registers for `ATmega328P` and
  `ATmega328PB` wrongly being read-only ([#73], [#74])

[#63]: https://github.com/Rahix/avr-device/pull/63
[#64]: https://github.com/Rahix/avr-device/pull/64
[#67]: https://github.com/Rahix/avr-device/pull/67
[#69]: https://github.com/Rahix/avr-device/pull/69
[#73]: https://github.com/Rahix/avr-device/pull/73
[#74]: https://github.com/Rahix/avr-device/pull/74
[`atdf2svd` v0.2.0]: https://github.com/Rahix/atdf2svd/blob/master/CHANGELOG.md#020---2020-11-25


## [0.2.3] - 2020-10-19
### Added
- Support for `ATmega644`
- Support for `ATmega328PB`

### Changed
- Rewired svd-patching infrastructure to use upstream
  [`svdtools`][svdtools] instead of the locally vendored script.

### Fixed
- Made `ADCSRA` for ATtiny84 read-write.
- Made `TIFR` registers for ATmega timers read-write.

[svdtools]: https://github.com/stm32-rs/svdtools


## [0.2.2] - 2020-08-20
### Added
- Support for `ATmega168`
- Support for `ATmega48P`
- Support for `ATtiny84`

### Changed
- `#[entry]` causes a (readable) compile-error when attempting to build
  for non-AVR targets.
- `PINx` registers are now writeable, which enables efficient toggling
  of bits in the corresponding `PORTx` register.

### Fixed
- inline-assembly is now only emitted when building for AVR targets to
  prevent weird compiler errors where AVR instructions are emitted on e.g.
  x86_64.
- Fixed an issue in the build-system in preparation for the next svd2rust
  release.


## [0.2.1] - 2020-08-07
### Fixed
- Fixed `interrupt::free()` not working as advertised, because a wrong
  address was read.


## [0.2.0] - 2020-08-07
### Added
- Support for `ATtiny88`.
- An `asm` module with wrapper functions for various assembly
  instructions.

### Changed
- Various patches for `ATtiny85`, fixing up a lot of the remaining
  inconsistencies.
- `interrupt::enable()` is now unsafe, as it would otherwise be possible
  to enable interrupts safely inside a critical section.


## [0.1.1] - 2020-07-31
### Added
- Support for `ATmega2560`.

### Changed
- Patches for AVR timer/counter peripherals in `ATmega2560`, `ATmega328P`,
  `ATmega32U4`, `ATtiny85`.


## [0.1.0] - 2020-07-26
Initial release with support for `ATmega1280`, `ATmega328P`, `ATmega32U4`,
`ATmega64`, `ATmega8`, `ATtiny85`.

[Unreleased]: https://github.com/Rahix/avr-device/compare/v0.4.0...HEAD
[0.4.0]: https://github.com/Rahix/avr-device/compare/v0.3.4...v0.4.0
[0.3.4]: https://github.com/Rahix/avr-device/compare/v0.3.3...v0.3.4
[0.3.3]: https://github.com/Rahix/avr-device/compare/v0.3.2...v0.3.3
[0.3.2]: https://github.com/Rahix/avr-device/compare/v0.3.1...v0.3.2
[0.3.1]: https://github.com/Rahix/avr-device/compare/v0.3.0...v0.3.1
[0.3.0]: https://github.com/Rahix/avr-device/compare/v0.2.3...v0.3.0
[0.2.3]: https://github.com/Rahix/avr-device/compare/v0.2.2...v0.2.3
[0.2.2]: https://github.com/Rahix/avr-device/compare/v0.2.1...v0.2.2
[0.2.1]: https://github.com/Rahix/avr-device/compare/v0.2.0...v0.2.1
[0.2.0]: https://github.com/Rahix/avr-device/compare/v0.1.1...v0.2.0
[0.1.1]: https://github.com/Rahix/avr-device/compare/v0.1.0...v0.1.1
[0.1.0]: https://github.com/Rahix/avr-device/releases/tag/v0.1.0
