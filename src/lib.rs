#![no_std]

#[allow(non_camel_case_types)]
mod devices;

#[cfg(feature = "attiny85")]
pub use crate::devices::attiny85::*;
#[cfg(feature = "atmega32u4")]
pub use crate::devices::atmega32u4::*;

impl Peripherals {
    /// Returns all the peripherals *once*
    #[inline]
    pub fn take() -> Option<Self> {
        if unsafe { DEVICE_PERIPHERALS } {
            None
        } else {
            Some(unsafe { Peripherals::steal() })
        }
    }
}
