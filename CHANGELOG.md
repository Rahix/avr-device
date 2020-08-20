# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]
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

[Unreleased]: https://github.com/Rahix/avr-device/compare/v0.2.1...HEAD
[0.2.1]: https://github.com/Rahix/avr-device/compare/v0.2.0...v0.2.1
[0.2.0]: https://github.com/Rahix/avr-device/compare/v0.1.1...v0.2.0
[0.1.1]: https://github.com/Rahix/avr-device/compare/v0.1.0...v0.1.1
[0.1.0]: https://github.com/Rahix/avr-device/releases/tag/v0.1.0
