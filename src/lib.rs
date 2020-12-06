//! This crate contains register definitions for
#![cfg_attr(feature = "atmega1280", doc = "**atmega1280**,")]
#![cfg_attr(feature = "atmega168", doc = "**atmega168**,")]
#![cfg_attr(feature = "atmega2560", doc = "**atmega2560**,")]
#![cfg_attr(feature = "atmega8", doc = "**atmega8**,")]
#![cfg_attr(feature = "atmega328p", doc = "**atmega328p**,")]
#![cfg_attr(feature = "atmega328pb", doc = "**atmega328pb**,")]
#![cfg_attr(feature = "atmega32u4", doc = "**atmega32u4**,")]
#![cfg_attr(feature = "atmega4809", doc = "**atmega4809**,")]
#![cfg_attr(feature = "atmega48p", doc = "**atmega48p**,")]
#![cfg_attr(feature = "atmega64", doc = "**atmega64**,")]
#![cfg_attr(feature = "atmega644", doc = "**atmega644**,")]
#![cfg_attr(feature = "attiny84", doc = "**attiny84**,")]
#![cfg_attr(feature = "attiny841", doc = "**attiny841**,")]
#![cfg_attr(feature = "attiny85", doc = "**attiny85**,")]
#![cfg_attr(feature = "attiny861", doc = "**attiny861**,")]
#![cfg_attr(feature = "attiny88", doc = "**attiny88**,")]
//! and a few things which apply to AVR microcontrollers generally.
//!
//! Which chips the crate is built for depends on the feature flag used.
//! The following chips are available (using feature flags of the same name):
//! * `atmega1280`
//! * `atmega168`
//! * `atmega2560`
//! * `atmega8`
//! * `atmega328p`
//! * `atmega328pb`
//! * `atmega32u4`
//! * `atmega4809`
//! * `atmega48p`
//! * `atmega64`
//! * `atmega644`
//! * `attiny84`
//! * `attiny841`
//! * `attiny85`
//! * `attiny861`
//! * `attiny88`
#![no_std]
#![feature(llvm_asm)]

pub mod asm;
pub mod interrupt;

#[allow(unused_imports)]
use generic::*;
#[doc = r"Common register and bit access and modify traits"]
pub mod generic;

/// Attribute to declare an interrupt service routine
///
/// ```
/// #[avr_device::interrupt(atmega32u4)]
/// fn INT6() {
///     // ...
/// }
/// ```
///
/// # Constraints
/// - The name of the function must be the name of an interrupt.  Each chip's
///   module has a `Interrupt` enum defining the available names.
/// - The attribute needs the chip-name to correctly map the interrupt to its
///   vector.  This is an unfortunate requirement of the current crate
///   architecture and might change in the future.
/// - The function must have a signature of `[unsafe] fn() [-> !]`.
#[cfg(feature = "rt")]
pub use avr_device_macros::interrupt;

/// Attribute to declare the entry point of the program
///
/// Exactly one entry point must be declared in the entire dependency tree.
///
/// ```
/// #[avr_device::entry]
/// fn main() -> ! {
///     // ...
/// }
/// ```
///
/// The entry function must have a signature of `[unsafe] fn() -> !`.
#[cfg(feature = "rt")]
pub use avr_device_macros::entry;

#[allow(non_camel_case_types, unused_attributes, unreachable_patterns)]
mod devices;

#[cfg(feature = "atmega1280")]
pub use crate::devices::atmega1280;
#[cfg(feature = "atmega168")]
pub use crate::devices::atmega168;
#[cfg(feature = "atmega2560")]
pub use crate::devices::atmega2560;
#[cfg(feature = "atmega328p")]
pub use crate::devices::atmega328p;
#[cfg(feature = "atmega328pb")]
pub use crate::devices::atmega328pb;
#[cfg(feature = "atmega32u4")]
pub use crate::devices::atmega32u4;
#[cfg(feature = "atmega4809")]
pub use crate::devices::atmega4809;
#[cfg(feature = "atmega48p")]
pub use crate::devices::atmega48p;
#[cfg(feature = "atmega64")]
pub use crate::devices::atmega64;
#[cfg(feature = "atmega644")]
pub use crate::devices::atmega644;
#[cfg(feature = "atmega8")]
pub use crate::devices::atmega8;
#[cfg(feature = "attiny84")]
pub use crate::devices::attiny84;
#[cfg(feature = "attiny841")]
pub use crate::devices::attiny841;
#[cfg(feature = "attiny85")]
pub use crate::devices::attiny85;
#[cfg(feature = "attiny861")]
pub use crate::devices::attiny861;
#[cfg(feature = "attiny88")]
pub use crate::devices::attiny88;

#[cfg(not(any(
    feature = "atmega1280",
    feature = "atmega168",
    feature = "atmega2560",
    feature = "atmega8",
    feature = "atmega328p",
    feature = "atmega328pb",
    feature = "atmega32u4",
    feature = "atmega4809",
    feature = "atmega48p",
    feature = "atmega64",
    feature = "atmega644",
    feature = "attiny84",
    feature = "attiny841",
    feature = "attiny85",
    feature = "attiny861",
    feature = "attiny88",
)))]
compile_error!("You need to select at least one chip as a feature!");
