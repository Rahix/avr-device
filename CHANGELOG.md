# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]
### Changed
- **BREAKING**: Changed the architecture of `avr-device` such that code is now
  generated at build time ([#157]).  This allows us to scale better and also makes the
  code-generation much simpler in general.  No external dependencies need to be
  installed anymore when working on this crate.
- **BREAKING**: Upgraded to svd2rust version 0.36.1 ([#157]).  This leads to
  mostly one major change in the register API:
  - Registers are now accessed through functions instead of struct members.

  In practice, this means you will have to do the following changes throughout
  your codebase:
  ```diff
  -dp.PORTD.portd.write(|w| w.pd3().clear_bit());
  +dp.PORTD.portd().write(|w| w.pd3().clear_bit());

  -dp.TC0.tccr0b.write(|w| w.cs0().prescale_1024());
  +dp.TC0.tccr0b().write(|w| w.cs0().prescale_1024());
  ```
- **BREAKING**: Renamed the `critical-section-impl` to just `critical-section`
  and enabled it by default.  This was done because the `Periperals::take()`
  method is now gated on `critical-section` ([#195]).
- Switched to the rust version of svdtools ([#174]).
- Better register definitions for peripherals of the ATmega128RFA1 ([#173]).

### Added
- Support for `AT90CAN128`, `AT90CAN64`, and `AT90CAN32`.
- Support for `ATtiny204`, `ATtiny804`, and `ATtiny1604` ([#182]).
- Support for `ATtiny1606` ([#183]).
- Support for `ATtiny1626` ([#185]). The `VPORTA.DIR` register is currently
  unavailable due to a compiler limitation.  See pull request for details.
- Added `asm::get_stack_size()` function which returns the current size of the
  stack ([#189]).  This function depends on the `rt` feature.

[#157]: https://github.com/Rahix/avr-device/pull/157
[#173]: https://github.com/Rahix/avr-device/pull/173
[#174]: https://github.com/Rahix/avr-device/pull/174
[#182]: https://github.com/Rahix/avr-device/pull/182
[#183]: https://github.com/Rahix/avr-device/pull/183
[#185]: https://github.com/Rahix/avr-device/pull/185
[#189]: https://github.com/Rahix/avr-device/pull/189
[#195]: https://github.com/Rahix/avr-device/pull/195


## [0.7.0] - 2025-01-05
### Added
- Added support for `ATtiny212`, `ATtiny214`. `ATtiny412`, `ATtiny414`, `ATtiny416` ([#167]).
- Added support for `ATmega16U2` and `ATmega32U2` ([#164]).
- Added support for `ATmega3208` and `ATmega3209` ([#164]).

### Changed
- **BREAKING** Upgraded to `atdf2svd` version 0.5.0 ([#170]).  This is a
  breaking change because some registers now have fields instead of safe
  `.bits()` access.  Chech the pull-request for details.
- **BREAKING** Renamed `in` to `input` in ATtiny-xmega PORT registers ([#171]).

### Fixed
- Fixed `ATmega88P` not being available ([#166]).

[#164]: https://github.com/Rahix/avr-device/pull/164
[#166]: https://github.com/Rahix/avr-device/pull/166
[#167]: https://github.com/Rahix/avr-device/pull/167
[#170]: https://github.com/Rahix/avr-device/pull/170
[#171]: https://github.com/Rahix/avr-device/pull/171


## [0.6.0] - 2024-09-20
### Added
- Added support for `AVR64DU32` and `AVR64DU28` ([#152]).
- Added support for `ATmega16` ([#153]).
- Added support for `ATtiny26` ([#158]).

### Changed
- Upgraded to `atdf2svd` version 0.4.0 ([#154]).
- The split peripherals of `ATmega4808`, `ATtiny402`, `ATtiny404` are now represented ([#154] and [atdf2svd#48]).

### Fixed
- Patched the ADC registers of ATtiny84A ([#151]).

[#151]: https://github.com/Rahix/avr-device/pull/151
[#152]: https://github.com/Rahix/avr-device/pull/152
[#153]: https://github.com/Rahix/avr-device/pull/153
[#154]: https://github.com/Rahix/avr-device/pull/154
[#158]: https://github.com/Rahix/avr-device/pull/158
[atdf2svd#48]: https://github.com/Rahix/atdf2svd/pull/48


## [0.5.4] - 2024-01-28
### Added
- Support for `ATtiny44A` ([#141]).
- Support for `ATtiny84A` ([#143]).
- Added an example of bare `avr-device` usage (without `avr-hal`) ([#146]).

### Fixed
- Added missing pin fields in `DDR`,`PIN`,`PORT` registers for `ATtiny167`,
  `ATtiny84`, `ATtiny2313`, and `ATtiny2313A` ([#148], [#149]).
- Added missing fields for `PF7` pin on `ATmega32U4` ([#150]).

[#141]: https://github.com/Rahix/avr-device/pull/141
[#143]: https://github.com/Rahix/avr-device/pull/143
[#146]: https://github.com/Rahix/avr-device/pull/146
[#148]: https://github.com/Rahix/avr-device/pull/148
[#149]: https://github.com/Rahix/avr-device/pull/149
[#150]: https://github.com/Rahix/avr-device/pull/150


## [0.5.3] - 2023-11-17
### Added
- Support for `ATmega4808` ([#137]).
- Support for `ATtiny402` ([#140]).

### Changed
- Patched EEPROM registers for `ATmega164PA` ([#139]).
- Patched added fields to the `PCMSK` register on `ATtiny85` ([#131]).

[#131]: https://github.com/Rahix/avr-device/pull/131
[#137]: https://github.com/Rahix/avr-device/pull/137
[#139]: https://github.com/Rahix/avr-device/pull/139
[#140]: https://github.com/Rahix/avr-device/pull/140


## [0.5.2] - 2023-08-24
### Added
- `asm::delay_cycles()` for delaying at least a certain number of cpu cycles ([#127]).
- Support for `ATtiny828` ([#126]).

[#127]: https://github.com/Rahix/avr-device/pull/127
[#126]: https://github.com/Rahix/avr-device/pull/126


## [0.5.1] - 2023-04-09
### Added
- Support for `ATmega128A` ([#121]).
- Support for `ATmega324PA` ([#119]).
- Support for `ATmega88P` ([#120]).
- Support for `ATmega32A` ([#123]).

[#119]: https://github.com/Rahix/avr-device/pull/119
[#120]: https://github.com/Rahix/avr-device/pull/120
[#121]: https://github.com/Rahix/avr-device/pull/121
[#123]: https://github.com/Rahix/avr-device/pull/123


## [0.5.0] - 2023-01-05
### Added
- Support for `ATtiny404` ([#111]).
- Support for the [`critical-section`] crate via a `critical-section-impl`
  feature flag ([#116]).

### Changed
- Patched registers for `ATmega8` ([#112]).
- Upgraded to svd2rust 0.28.  This changes the register API slightly, please
  check upstream docs for details ([#118]).

[#111]: https://github.com/Rahix/avr-device/pull/111
[#112]: https://github.com/Rahix/avr-device/pull/112
[#116]: https://github.com/Rahix/avr-device/pull/116
[#118]: https://github.com/Rahix/avr-device/pull/118
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

[Unreleased]: https://github.com/Rahix/avr-device/compare/v0.7.0...HEAD
[0.7.0]: https://github.com/Rahix/avr-device/compare/v0.6.0...v0.7.0
[0.6.0]: https://github.com/Rahix/avr-device/compare/v0.5.4...v0.6.0
[0.5.4]: https://github.com/Rahix/avr-device/compare/v0.5.3...v0.5.4
[0.5.3]: https://github.com/Rahix/avr-device/compare/v0.5.2...v0.5.3
[0.5.2]: https://github.com/Rahix/avr-device/compare/v0.5.1...v0.5.2
[0.5.1]: https://github.com/Rahix/avr-device/compare/v0.5.0...v0.5.1
[0.5.0]: https://github.com/Rahix/avr-device/compare/v0.4.0...v0.5.0
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
