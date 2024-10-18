#[allow(renamed_and_removed_lints)]
#[allow(private_no_mangle_statics)]
#[no_mangle]
pub(crate) static mut DEVICE_PERIPHERALS: bool = false;

/// [AT90USB1286](https://www.microchip.com/wwwproducts/en/AT90USB1286)
#[cfg(feature = "at90usb1286")]
pub mod at90usb1286;

/// [ATmega1280](https://www.microchip.com/wwwproducts/en/ATmega1280)
#[cfg(feature = "atmega1280")]
pub mod atmega1280;

/// [ATmega1284P](https://www.microchip.com/en-us/product/ATmega1284P)
#[cfg(feature = "atmega1284p")]
pub mod atmega1284p;

/// [ATmega128A](https://www.microchip.com/wwwproducts/en/ATmega128A)
#[cfg(feature = "atmega128a")]
pub mod atmega128a;

/// [ATmega128RFA1](https://www.microchip.com/en-us/product/ATmega128RFA1)
#[cfg(feature = "atmega128rfa1")]
pub mod atmega128rfa1;

/// [ATmega16](https://www.microchip.com/wwwproducts/en/ATmega16)
#[cfg(feature = "atmega16")]
pub mod atmega16;

/// [ATmega164PA](https://www.microchip.com/en-us/product/ATmega164PA)
#[cfg(feature = "atmega164pa")]
pub mod atmega164pa;

/// [ATmega168](https://www.microchip.com/wwwproducts/en/ATmega168)
#[cfg(feature = "atmega168")]
pub mod atmega168;

/// [ATmega16u2](https://www.microchip.com/wwwproducts/en/ATmega16u2)
#[cfg(feature = "atmega16u2")]
pub mod atmega16u2;

/// [ATmega2560](https://www.microchip.com/wwwproducts/en/ATmega2560)
#[cfg(feature = "atmega2560")]
pub mod atmega2560;

/// [ATmega324PA](https://www.microchip.com/wwwproducts/en/ATmega324PA)
#[cfg(feature = "atmega324pa")]
pub mod atmega324pa;

/// [ATmega328P](https://www.microchip.com/wwwproducts/en/ATmega328P)
#[cfg(feature = "atmega328p")]
pub mod atmega328p;

/// [ATmega328PB](https://www.microchip.com/wwwproducts/en/ATmega328PB)
#[cfg(feature = "atmega328pb")]
pub mod atmega328pb;

/// [ATmega32A](https://www.microchip.com/wwwproducts/en/ATmega32A)
#[cfg(feature = "atmega32a")]
pub mod atmega32a;

/// [ATmega32u2](https://www.microchip.com/wwwproducts/en/ATmega32u2)
#[cfg(feature = "atmega32u2")]
pub mod atmega32u2;

/// [ATmega32U4](https://www.microchip.com/wwwproducts/en/ATmega32U4)
#[cfg(feature = "atmega32u4")]
pub mod atmega32u4;

/// [ATmega3208](https://www.microchip.com/wwwproducts/en/ATmega3208)
#[cfg(feature = "atmega3208")]
pub mod atmega3208;

/// [ATmega3209](https://www.microchip.com/wwwproducts/en/ATmega3209)
#[cfg(feature = "atmega3209")]
pub mod atmega3209;

/// [ATmega4808](https://www.microchip.com/wwwproducts/en/ATmega4808)
#[cfg(feature = "atmega4808")]
pub mod atmega4808;

/// [ATmega4809](https://www.microchip.com/wwwproducts/en/ATmega4809)
#[cfg(feature = "atmega4809")]
pub mod atmega4809;

/// [ATmega48P](https://www.microchip.com/wwwproducts/en/ATmega48P)
#[cfg(feature = "atmega48p")]
pub mod atmega48p;

/// [ATmega8](https://www.microchip.com/wwwproducts/en/ATmega8)
#[cfg(feature = "atmega8")]
pub mod atmega8;

/// [ATmega8u2](https://www.microchip.com/wwwproducts/en/ATmega8u2)
#[cfg(feature = "atmega8u2")]
pub mod atmega8u2;

/// [ATmega64](https://www.microchip.com/wwwproducts/en/ATmega64)
#[cfg(feature = "atmega64")]
pub mod atmega64;

/// [ATmega644](https://www.microchip.com/wwwproducts/en/ATmega644)
#[cfg(feature = "atmega644")]
pub mod atmega644;

/// [ATtiny13A](https://www.microchip.com/wwwproducts/en/ATtiny13A)
#[cfg(feature = "attiny13a")]
pub mod attiny13a;

/// [ATtiny167](https://www.microchip.com/wwwproducts/en/ATtiny167)
#[cfg(feature = "attiny167")]
pub mod attiny167;

/// [ATtiny1614](https://www.microchip.com/wwwproducts/en/ATtiny1614)
#[cfg(feature = "attiny1614")]
pub mod attiny1614;

/// [ATtiny202](https://www.microchip.com/wwwproducts/en/ATtiny202)
#[cfg(feature = "attiny202")]
pub mod attiny202;

/// [ATtiny2313](https://www.microchip.com/wwwproducts/en/ATtiny2313)
#[cfg(feature = "attiny2313")]
pub mod attiny2313;

/// [ATtiny2313A](https://www.microchip.com/wwwproducts/en/ATtiny2313A)
#[cfg(feature = "attiny2313a")]
pub mod attiny2313a;

/// [ATtiny26](https://www.microchip.com/wwwproducts/en/ATtiny26)
#[cfg(feature = "attiny26")]
pub mod attiny26;

/// [ATtiny402](https://www.microchip.com/en-us/product/ATTINY402)
#[cfg(feature = "attiny402")]
pub mod attiny402;

/// [ATtiny404](https://www.microchip.com/en-us/product/ATTINY404)
#[cfg(feature = "attiny404")]
pub mod attiny404;

/// [ATtiny44a](https://www.microchip.com/en-us/product/ATtiny44a)
#[cfg(feature = "attiny44a")]
pub mod attiny44a;

/// [ATtiny816](https://www.microchip.com/wwwproducts/en/ATtiny816)
#[cfg(feature = "attiny816")]
pub mod attiny816;

/// [ATtiny828](https://www.microchip.com/wwwproducts/en/ATtiny828)
#[cfg(feature = "attiny828")]
pub mod attiny828;

/// [ATtiny84](https://www.microchip.com/wwwproducts/en/ATtiny84)
#[cfg(feature = "attiny84")]
pub mod attiny84;

/// [ATtiny841](https://www.microchip.com/wwwproducts/en/ATtiny841)
#[cfg(feature = "attiny841")]
pub mod attiny841;

/// [ATtiny84a](https://www.microchip.com/en-us/product/ATtiny84a)
#[cfg(feature = "attiny84a")]
pub mod attiny84a;

/// [ATtiny85](https://www.microchip.com/wwwproducts/en/ATtiny85)
#[cfg(feature = "attiny85")]
pub mod attiny85;

/// [ATtiny861](https://www.microchip.com/wwwproducts/en/ATtiny861)
#[cfg(feature = "attiny861")]
pub mod attiny861;

/// [ATtiny88](https://www.microchip.com/wwwproducts/en/ATtiny88)
#[cfg(feature = "attiny88")]
pub mod attiny88;

/// [AVR64DU32](https://www.microchip.com/wwwproducts/en/AVR64DU32)
#[cfg(feature = "avr64du32")]
pub mod avr64du32;

/// [AVR64DU28](https://www.microchip.com/wwwproducts/en/AVR64DU28)
#[cfg(feature = "avr64du28")]
pub mod avr64du28;
