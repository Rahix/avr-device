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

/// [ATmega168](https://www.microchip.com/wwwproducts/en/ATmega168)
#[cfg(feature = "atmega168")]
pub mod atmega168;

#[cfg(feature = "atmega168")]
impl atmega168::Peripherals {
    /// Returns all the peripherals *once*
    #[inline]
    pub fn take() -> Option<Self> {
        crate::interrupt::free(|_| {
            if unsafe { DEVICE_PERIPHERALS } {
                None
            } else {
                Some(unsafe { atmega168::Peripherals::steal() })
            }
        })
    }
}

/// [ATmega2560](https://www.microchip.com/wwwproducts/en/ATmega2560)
#[cfg(feature = "atmega2560")]
pub mod atmega2560;

#[cfg(feature = "atmega2560")]
impl atmega2560::Peripherals {
    /// Returns all the peripherals *once*
    #[inline]
    pub fn take() -> Option<Self> {
        crate::interrupt::free(|_| {
            if unsafe { DEVICE_PERIPHERALS } {
                None
            } else {
                Some(unsafe { atmega2560::Peripherals::steal() })
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

/// [ATmega328PB](https://www.microchip.com/wwwproducts/en/ATmega328PB)
#[cfg(feature = "atmega328pb")]
pub mod atmega328pb;

#[cfg(feature = "atmega328pb")]
impl atmega328pb::Peripherals {
    /// Returns all the peripherals *once*
    #[inline]
    pub fn take() -> Option<Self> {
        crate::interrupt::free(|_| {
            if unsafe { DEVICE_PERIPHERALS } {
                None
            } else {
                Some(unsafe { atmega328pb::Peripherals::steal() })
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

/// [ATmega4809](https://www.microchip.com/wwwproducts/en/ATmega4809)
#[cfg(feature = "atmega4809")]
pub mod atmega4809;

#[cfg(feature = "atmega4809")]
impl atmega4809::Peripherals {
    /// Returns all the peripherals *once*
    #[inline]
    pub fn take() -> Option<Self> {
        crate::interrupt::free(|_| {
            if unsafe { DEVICE_PERIPHERALS } {
                None
            } else {
                Some(unsafe { atmega4809::Peripherals::steal() })
            }
        })
    }
}

/// [ATmega48P](https://www.microchip.com/wwwproducts/en/ATmega48P)
#[cfg(feature = "atmega48p")]
pub mod atmega48p;

#[cfg(feature = "atmega48p")]
impl atmega48p::Peripherals {
    /// Returns all the peripherals *once*
    #[inline]
    pub fn take() -> Option<Self> {
        crate::interrupt::free(|_| {
            if unsafe { DEVICE_PERIPHERALS } {
                None
            } else {
                Some(unsafe { atmega48p::Peripherals::steal() })
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

/// [ATmega8u2](https://www.microchip.com/wwwproducts/en/ATmega8u2)
#[cfg(feature = "atmega8u2")]
pub mod atmega8u2;

#[cfg(feature = "atmega8u2")]
impl atmega8u2::Peripherals {
    /// Returns all the peripherals *once*
    #[inline]
    pub fn take() -> Option<Self> {
        crate::interrupt::free(|_| {
            if unsafe { DEVICE_PERIPHERALS } {
                None
            } else {
                Some(unsafe { atmega8u2::Peripherals::steal() })
            }
        })
    }
}

/// [ATmega64](https://www.microchip.com/wwwproducts/en/ATmega64)
#[cfg(feature = "atmega64")]
pub mod atmega64;

#[cfg(feature = "atmega64")]
impl atmega64::Peripherals {
    /// Returns all the peripherals *once*
    #[inline]
    pub fn take() -> Option<Self> {
        crate::interrupt::free(|_| {
            if unsafe { DEVICE_PERIPHERALS } {
                None
            } else {
                Some(unsafe { atmega64::Peripherals::steal() })
            }
        })
    }
}

/// [ATmega644](https://www.microchip.com/wwwproducts/en/ATmega644)
#[cfg(feature = "atmega644")]
pub mod atmega644;

#[cfg(feature = "atmega644")]
impl atmega644::Peripherals {
    /// Returns all the peripherals *once*
    #[inline]
    pub fn take() -> Option<Self> {
        crate::interrupt::free(|_| {
            if unsafe { DEVICE_PERIPHERALS } {
                None
            } else {
                Some(unsafe { atmega644::Peripherals::steal() })
            }
        })
    }
}

/// [ATtiny84](https://www.microchip.com/wwwproducts/en/ATtiny84)
#[cfg(feature = "attiny84")]
pub mod attiny84;

#[cfg(feature = "attiny84")]
impl attiny84::Peripherals {
    /// Returns all the peripherals *once*
    #[inline]
    pub fn take() -> Option<Self> {
        crate::interrupt::free(|_| {
            if unsafe { DEVICE_PERIPHERALS } {
                None
            } else {
                Some(unsafe { attiny84::Peripherals::steal() })
            }
        })
    }
}

/// [ATtiny841](https://www.microchip.com/wwwproducts/en/ATtiny841)
#[cfg(feature = "attiny841")]
pub mod attiny841;

#[cfg(feature = "attiny841")]
impl attiny841::Peripherals {
    /// Returns all the peripherals *once*
    #[inline]
    pub fn take() -> Option<Self> {
        crate::interrupt::free(|_| {
            if unsafe { DEVICE_PERIPHERALS } {
                None
            } else {
                Some(unsafe { attiny841::Peripherals::steal() })
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

/// [ATtiny861](https://www.microchip.com/wwwproducts/en/ATtiny861)
#[cfg(feature = "attiny861")]
pub mod attiny861;

#[cfg(feature = "attiny861")]
impl attiny861::Peripherals {
    /// Returns all the peripherals *once*
    #[inline]
    pub fn take() -> Option<Self> {
        crate::interrupt::free(|_| {
            if unsafe { DEVICE_PERIPHERALS } {
                None
            } else {
                Some(unsafe { attiny861::Peripherals::steal() })
            }
        })
    }
}

/// [ATtiny88](https://www.microchip.com/wwwproducts/en/ATtiny88)
#[cfg(feature = "attiny88")]
pub mod attiny88;

#[cfg(feature = "attiny88")]
impl attiny88::Peripherals {
    /// Returns all the peripherals *once*
    #[inline]
    pub fn take() -> Option<Self> {
        crate::interrupt::free(|_| {
            if unsafe { DEVICE_PERIPHERALS } {
                None
            } else {
                Some(unsafe { attiny88::Peripherals::steal() })
            }
        })
    }
}
