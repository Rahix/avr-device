#![no_std]
#![feature(asm)]

#[allow(non_camel_case_types, unused_attributes)]
mod devices;

pub mod interrupt;

cfg_if::cfg_if! {
    if #[cfg(feature = "atmega328p")] {
        pub use crate::devices::atmega328p::*;
    } else if #[cfg(feature = "atmega32u4")] {
        pub use crate::devices::atmega32u4::*;
    } else if #[cfg(feature = "attiny85")] {
        pub use crate::devices::attiny85::*;
    } else {
        compile_error!("You need to select exactly one chip as a feature!");
    }
}

#[cfg(any(feature = "attiny85", feature = "atmega32u4"))]
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
