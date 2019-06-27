#[allow(renamed_and_removed_lints)]
#[allow(private_no_mangle_statics)]
#[no_mangle]
pub(crate) static mut DEVICE_PERIPHERALS: bool = false;

/// [ATmega1280](https://www.microchip.com/wwwproducts/en/ATmega1280)
#[cfg(feature = "atmega1280")]
pub mod atmega1280;

#[cfg(feature = "atmega1280")]
impl atmega1280::Peripherals {
    /// Returns all the peripherals *once*
    #[inline]
    pub fn take() -> Option<Self> {
        crate::interrupt::free(|_| {
            if unsafe { DEVICE_PERIPHERALS } {
                None
            } else {
                Some(unsafe { atmega1280::Peripherals::steal() })
            }
        })
    }
}

/// [ATmega328P](https://www.microchip.com/wwwproducts/en/ATmega328P)
#[cfg(feature = "atmega328p")]
pub mod atmega328p;

#[cfg(feature = "atmega328p")]
impl atmega328p::Peripherals {
    /// Returns all the peripherals *once*
    #[inline]
    pub fn take() -> Option<Self> {
        crate::interrupt::free(|_| {
            if unsafe { DEVICE_PERIPHERALS } {
                None
            } else {
                Some(unsafe { atmega328p::Peripherals::steal() })
            }
        })
    }
}

/// [ATmega32U4](https://www.microchip.com/wwwproducts/en/ATmega32U4)
#[cfg(feature = "atmega32u4")]
pub mod atmega32u4;

#[cfg(feature = "atmega32u4")]
impl atmega32u4::Peripherals {
    /// Returns all the peripherals *once*
    #[inline]
    pub fn take() -> Option<Self> {
        crate::interrupt::free(|_| {
            if unsafe { DEVICE_PERIPHERALS } {
                None
            } else {
                Some(unsafe { atmega32u4::Peripherals::steal() })
            }
        })
    }
}

/// [ATmega8](https://www.microchip.com/wwwproducts/en/ATmega8)
#[cfg(feature = "atmega8")]
pub mod atmega8;

#[cfg(feature = "atmega8")]
impl atmega8::Peripherals {
    /// Returns all the peripherals *once*
    #[inline]
    pub fn take() -> Option<Self> {
        crate::interrupt::free(|_| {
            if unsafe { DEVICE_PERIPHERALS } {
                None
            } else {
                Some(unsafe { atmega8::Peripherals::steal() })
            }
        })
    }
}

/// [ATtiny85](https://www.microchip.com/wwwproducts/en/ATtiny85)
#[cfg(feature = "attiny85")]
pub mod attiny85;

#[cfg(feature = "attiny85")]
impl attiny85::Peripherals {
    /// Returns all the peripherals *once*
    #[inline]
    pub fn take() -> Option<Self> {
        crate::interrupt::free(|_| {
            if unsafe { DEVICE_PERIPHERALS } {
                None
            } else {
                Some(unsafe { attiny85::Peripherals::steal() })
            }
        })
    }
}
