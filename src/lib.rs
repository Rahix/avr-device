//! This crate contains register definitions for
#![cfg_attr(feature = "at90usb1286", doc = "**at90usb1286**,")]
#![cfg_attr(feature = "atmega1280", doc = "**atmega1280**,")]
#![cfg_attr(feature = "atmega1284p", doc = "**atmega1284p**,")]
#![cfg_attr(feature = "atmega128rfa1", doc = "**atmega128rfa1**,")]
#![cfg_attr(feature = "atmega168", doc = "**atmega168**,")]
#![cfg_attr(feature = "atmega2560", doc = "**atmega2560**,")]
#![cfg_attr(feature = "atmega8", doc = "**atmega8**,")]
#![cfg_attr(feature = "atmega8u2", doc = "**atmega8u2**,")]
#![cfg_attr(feature = "atmega328p", doc = "**atmega328p**,")]
#![cfg_attr(feature = "atmega328pb", doc = "**atmega328pb**,")]
#![cfg_attr(feature = "atmega32u4", doc = "**atmega32u4**,")]
#![cfg_attr(feature = "atmega4809", doc = "**atmega4809**,")]
#![cfg_attr(feature = "atmega48p", doc = "**atmega48p**,")]
#![cfg_attr(feature = "atmega64", doc = "**atmega64**,")]
#![cfg_attr(feature = "atmega644", doc = "**atmega644**,")]
#![cfg_attr(feature = "attiny167", doc = "**attiny167**,")]
#![cfg_attr(feature = "attiny1614", doc = "**attiny1614**,")]
#![cfg_attr(feature = "attiny202", doc = "**attiny202**,")]
#![cfg_attr(feature = "attiny2313", doc = "**attiny2313**,")]
#![cfg_attr(feature = "attiny2313a", doc = "**attiny2313a**,")]
#![cfg_attr(feature = "attiny816", doc = "**attiny816**,")]
#![cfg_attr(feature = "attiny84", doc = "**attiny84**,")]
#![cfg_attr(feature = "attiny841", doc = "**attiny841**,")]
#![cfg_attr(feature = "attiny85", doc = "**attiny85**,")]
#![cfg_attr(feature = "attiny861", doc = "**attiny861**,")]
#![cfg_attr(feature = "attiny88", doc = "**attiny88**,")]
//! and a few things which apply to AVR microcontrollers generally.
//!
//! Which chips the crate is built for depends on the feature flag used.
//! The following chips are available (using feature flags of the same name):
//! * `at90usb1286`
//! * `atmega1280`
//! * `atmega1284p`
//! * `atmega128rfa1`
//! * `atmega168`
//! * `atmega2560`
//! * `atmega8`
//! * `atmega8u2`
//! * `atmega328p`
//! * `atmega328pb`
//! * `atmega32u4`
//! * `atmega4809`
//! * `atmega48p`
//! * `atmega64`
//! * `atmega644`
//! * `attiny167`
//! * `attiny1614`
//! * `attiny202`
//! * `attiny2313`
//! * `attiny2313a`
//! * `attiny816`
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

#[cfg(not(feature = "device-selected"))]
compile_error!(
    "This crate requires you to specify your target chip as a feature.

    Please select one of the following:

    * atmega1280
    * atmega1284p
    * atmega128rfa1
    * atmega168
    * atmega2560
    * atmega328p
    * atmega328pb
    * atmega32u4
    * atmega4809
    * atmega48p
    * atmega64
    * atmega644
    * atmega8
    * atmega8u2
    * attiny167
    * attiny1614
    * attiny202
    * attiny2313
    * attiny2313a
    * attiny816
    * attiny84
    * attiny841
    * attiny85
    * attiny861
    * attiny88
    "
);

#[allow(non_camel_case_types, unused_attributes, unreachable_patterns)]
mod devices;

#[cfg(feature = "at90usb1286")]
pub use crate::devices::at90usb1286;
#[cfg(feature = "atmega1280")]
pub use crate::devices::atmega1280;
#[cfg(feature = "atmega1284p")]
pub use crate::devices::atmega1284p;
#[cfg(feature = "atmega128rfa1")]
pub use crate::devices::atmega128rfa1;
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
#[cfg(feature = "atmega8u2")]
pub use crate::devices::atmega8u2;
#[cfg(feature = "attiny167")]
pub use crate::devices::attiny167;
#[cfg(feature = "attiny1614")]
pub use crate::devices::attiny1614;
#[cfg(feature = "attiny202")]
pub use crate::devices::attiny202;
#[cfg(feature = "attiny2313")]
pub use crate::devices::attiny2313;
#[cfg(feature = "attiny2313a")]
pub use crate::devices::attiny2313a;
#[cfg(feature = "attiny816")]
pub use crate::devices::attiny816;
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
