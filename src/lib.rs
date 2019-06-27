//! This crate contains register definitions for
#![cfg_attr(feature = "atmega1280", doc = "**atmega1280**.")]
#![cfg_attr(feature = "atmega8", doc = "**atmega8**.")]
#![cfg_attr(feature = "atmega328p", doc = "**atmega328p**.")]
#![cfg_attr(feature = "atmega32u4", doc = "**atmega32u4**.")]
#![cfg_attr(feature = "attiny85", doc = "**attiny85**.")]
//! Which chip the crate is built for depends on the feature flag used.
//!
//! The following chips are available (using feature flags of the same name):
//! * `atmega1280`
//! * `atmega8`
//! * `atmega328p`
//! * `atmega32u4`
//! * `attiny85`
#![no_std]
#![feature(asm)]

#[allow(non_camel_case_types, unused_attributes)]
mod devices;

pub mod interrupt;

cfg_if::cfg_if! {
    if #[cfg(feature= "atmega1280")] {
        pub use crate::devices::atmega1280::*;
    } else if #[cfg(feature = "atmega8")] {
        pub use crate::devices::atmega8::*;
    } else if #[cfg(feature = "atmega328p")] {
        pub use crate::devices::atmega328p::*;
    } else if #[cfg(feature = "atmega32u4")] {
        pub use crate::devices::atmega32u4::*;
    } else if #[cfg(feature = "attiny85")] {
        pub use crate::devices::attiny85::*;
    } else {
        compile_error!("You need to select a chip as a feature!");
    }
}

// TODO: Find a better way to do this
#[cfg(any(
    all(feature = "atmega1280", any(
        feature = "atmega8",
        feature = "atmega328p",
        feature = "atmega32u4",
        feature = "attiny85",
    )),
    all(feature = "atmega8", any(
        feature = "atmega1280",
        feature = "atmega328p",
        feature = "atmega32u4",
        feature = "attiny85",
    )),
    all(feature = "atmega328p", any(
        feature = "atmega1280",
        feature = "atmega8",
        feature = "atmega32u4",
        feature = "attiny85",
    )),
    all(feature = "atmega32u4", any(
        feature = "atmega1280",
        feature = "atmega8",
        feature = "atmega329p",
        feature = "attiny85",
    )),
))]
compile_error!("You cannot select multiple chips at once!");

#[cfg(any(
    feature = "atmega1280",
    feature = "atmega8",
    feature = "atmega328p",
    feature = "atmega32u4",
    feature = "attiny85",
))]
impl Peripherals {
    /// Returns all the peripherals *once*
    #[inline]
    pub fn take() -> Option<Self> {
        interrupt::free(|_| {
            if unsafe { DEVICE_PERIPHERALS } {
                None
            } else {
                Some(unsafe { Peripherals::steal() })
            }
        })
    }
}
