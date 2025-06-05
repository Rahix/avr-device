#[allow(renamed_and_removed_lints)]
#[allow(private_no_mangle_statics)]
#[no_mangle]
pub(crate) static mut DEVICE_PERIPHERALS: bool = false;

/// [AT90CAN128](https://www.microchip.com/wwwproducts/en/AT90CAN128)
#[cfg(feature = "at90can128")]
pub mod at90can128 {
    include!(concat!(env!("OUT_DIR"), "/pac/at90can128.rs"));
}

/// [AT90CAN64](https://www.microchip.com/wwwproducts/en/AT90CAN64)
#[cfg(feature = "at90can64")]
pub mod at90can64 {
    include!(concat!(env!("OUT_DIR"), "/pac/at90can64.rs"));
}

/// [AT90CAN32](https://www.microchip.com/wwwproducts/en/AT90CAN32)
#[cfg(feature = "at90can32")]
pub mod at90can32 {
    include!(concat!(env!("OUT_DIR"), "/pac/at90can32.rs"));
}

/// [AT90USB1286](https://www.microchip.com/wwwproducts/en/AT90USB1286)
#[cfg(feature = "at90usb1286")]
pub mod at90usb1286 {
    include!(concat!(env!("OUT_DIR"), "/pac/at90usb1286.rs"));
}

/// [ATmega1280](https://www.microchip.com/wwwproducts/en/ATmega1280)
#[cfg(feature = "atmega1280")]
pub mod atmega1280 {
    include!(concat!(env!("OUT_DIR"), "/pac/atmega1280.rs"));
}

/// [ATmega1284P](https://www.microchip.com/en-us/product/ATmega1284P)
#[cfg(feature = "atmega1284p")]
pub mod atmega1284p {
    include!(concat!(env!("OUT_DIR"), "/pac/atmega1284p.rs"));
}

/// [ATmega128A](https://www.microchip.com/wwwproducts/en/ATmega128A)
#[cfg(feature = "atmega128a")]
pub mod atmega128a {
    include!(concat!(env!("OUT_DIR"), "/pac/atmega128a.rs"));
}

/// [ATmega128RFA1](https://www.microchip.com/en-us/product/ATmega128RFA1)
#[cfg(feature = "atmega128rfa1")]
pub mod atmega128rfa1 {
    include!(concat!(env!("OUT_DIR"), "/pac/atmega128rfa1.rs"));
}

/// [ATmega16](https://www.microchip.com/wwwproducts/en/ATmega16)
#[cfg(feature = "atmega16")]
pub mod atmega16 {
    include!(concat!(env!("OUT_DIR"), "/pac/atmega16.rs"));
}

/// [ATmega164PA](https://www.microchip.com/en-us/product/ATmega164PA)
#[cfg(feature = "atmega164pa")]
pub mod atmega164pa {
    include!(concat!(env!("OUT_DIR"), "/pac/atmega164pa.rs"));
}

/// [ATmega168](https://www.microchip.com/wwwproducts/en/ATmega168)
#[cfg(feature = "atmega168")]
pub mod atmega168 {
    include!(concat!(env!("OUT_DIR"), "/pac/atmega168.rs"));
}

/// [ATmega16u2](https://www.microchip.com/wwwproducts/en/ATmega16u2)
#[cfg(feature = "atmega16u2")]
pub mod atmega16u2 {
    include!(concat!(env!("OUT_DIR"), "/pac/atmega16u2.rs"));
}

/// [ATmega2560](https://www.microchip.com/wwwproducts/en/ATmega2560)
#[cfg(feature = "atmega2560")]
pub mod atmega2560 {
    include!(concat!(env!("OUT_DIR"), "/pac/atmega2560.rs"));
}

/// [ATmega324PA](https://www.microchip.com/wwwproducts/en/ATmega324PA)
#[cfg(feature = "atmega324pa")]
pub mod atmega324pa {
    include!(concat!(env!("OUT_DIR"), "/pac/atmega324pa.rs"));
}

/// [ATmega328P](https://www.microchip.com/wwwproducts/en/ATmega328P)
#[cfg(feature = "atmega328p")]
pub mod atmega328p {
    include!(concat!(env!("OUT_DIR"), "/pac/atmega328p.rs"));
}

/// [ATmega328PB](https://www.microchip.com/wwwproducts/en/ATmega328PB)
#[cfg(feature = "atmega328pb")]
pub mod atmega328pb {
    include!(concat!(env!("OUT_DIR"), "/pac/atmega328pb.rs"));
}

/// [ATmega32A](https://www.microchip.com/wwwproducts/en/ATmega32A)
#[cfg(feature = "atmega32a")]
pub mod atmega32a {
    include!(concat!(env!("OUT_DIR"), "/pac/atmega32a.rs"));
}

/// [ATmega32u2](https://www.microchip.com/wwwproducts/en/ATmega32u2)
#[cfg(feature = "atmega32u2")]
pub mod atmega32u2 {
    include!(concat!(env!("OUT_DIR"), "/pac/atmega32u2.rs"));
}

/// [ATmega32U4](https://www.microchip.com/wwwproducts/en/ATmega32U4)
#[cfg(feature = "atmega32u4")]
pub mod atmega32u4 {
    include!(concat!(env!("OUT_DIR"), "/pac/atmega32u4.rs"));
}

/// [ATmega3208](https://www.microchip.com/wwwproducts/en/ATmega3208)
#[cfg(feature = "atmega3208")]
pub mod atmega3208 {
    include!(concat!(env!("OUT_DIR"), "/pac/atmega3208.rs"));
}

/// [ATmega3209](https://www.microchip.com/wwwproducts/en/ATmega3209)
#[cfg(feature = "atmega3209")]
pub mod atmega3209 {
    include!(concat!(env!("OUT_DIR"), "/pac/atmega3209.rs"));
}

/// [ATmega4808](https://www.microchip.com/wwwproducts/en/ATmega4808)
#[cfg(feature = "atmega4808")]
pub mod atmega4808 {
    include!(concat!(env!("OUT_DIR"), "/pac/atmega4808.rs"));
}

/// [ATmega4809](https://www.microchip.com/wwwproducts/en/ATmega4809)
#[cfg(feature = "atmega4809")]
pub mod atmega4809 {
    include!(concat!(env!("OUT_DIR"), "/pac/atmega4809.rs"));
}

/// [ATmega48P](https://www.microchip.com/wwwproducts/en/ATmega48P)
#[cfg(feature = "atmega48p")]
pub mod atmega48p {
    include!(concat!(env!("OUT_DIR"), "/pac/atmega48p.rs"));
}

/// [ATmega8](https://www.microchip.com/wwwproducts/en/ATmega8)
#[cfg(feature = "atmega8")]
pub mod atmega8 {
    include!(concat!(env!("OUT_DIR"), "/pac/atmega8.rs"));
}

/// [ATmega8u2](https://www.microchip.com/wwwproducts/en/ATmega8u2)
#[cfg(feature = "atmega8u2")]
pub mod atmega8u2 {
    include!(concat!(env!("OUT_DIR"), "/pac/atmega8u2.rs"));
}

/// [ATmega64](https://www.microchip.com/wwwproducts/en/ATmega64)
#[cfg(feature = "atmega64")]
pub mod atmega64 {
    include!(concat!(env!("OUT_DIR"), "/pac/atmega64.rs"));
}

/// [ATmega644](https://www.microchip.com/wwwproducts/en/ATmega644)
#[cfg(feature = "atmega644")]
pub mod atmega644 {
    include!(concat!(env!("OUT_DIR"), "/pac/atmega644.rs"));
}

/// [ATmega88P](https://www.microchip.com/wwwproducts/en/ATmega88P)
#[cfg(feature = "atmega88p")]
pub mod atmega88p {
    include!(concat!(env!("OUT_DIR"), "/pac/atmega88p.rs"));
}

/// [ATtiny13A](https://www.microchip.com/wwwproducts/en/ATtiny13A)
#[cfg(feature = "attiny13a")]
pub mod attiny13a {
    include!(concat!(env!("OUT_DIR"), "/pac/attiny13a.rs"));
}

/// [ATtiny167](https://www.microchip.com/wwwproducts/en/ATtiny167)
#[cfg(feature = "attiny167")]
pub mod attiny167 {
    include!(concat!(env!("OUT_DIR"), "/pac/attiny167.rs"));
}

/// [ATtiny1606](https://www.microchip.com/wwwproducts/en/ATtiny1606)
#[cfg(feature = "attiny1606")]
pub mod attiny1606 {
    include!(concat!(env!("OUT_DIR"), "/pac/attiny1606.rs"));
}

/// [ATtiny1604](https://www.microchip.com/wwwproducts/en/ATtiny1604)
#[cfg(feature = "attiny1604")]
pub mod attiny1604 {
    include!(concat!(env!("OUT_DIR"), "/pac/attiny1604.rs"));
}

/// [ATtiny1614](https://www.microchip.com/wwwproducts/en/ATtiny1614)
#[cfg(feature = "attiny1614")]
pub mod attiny1614 {
    include!(concat!(env!("OUT_DIR"), "/pac/attiny1614.rs"));
}

/// [ATtiny1626](https://www.microchip.com/wwwproducts/en/ATtiny1626)
#[cfg(feature = "attiny1626")]
pub mod attiny1626 {
    include!(concat!(env!("OUT_DIR"), "/pac/attiny1626.rs"));
}

/// [ATtiny202](https://www.microchip.com/wwwproducts/en/ATtiny202)
#[cfg(feature = "attiny202")]
pub mod attiny202 {
    include!(concat!(env!("OUT_DIR"), "/pac/attiny202.rs"));
}

/// [ATtiny204](https://www.microchip.com/wwwproducts/en/ATtiny204)
#[cfg(feature = "attiny204")]
pub mod attiny204 {
    include!(concat!(env!("OUT_DIR"), "/pac/attiny204.rs"));
}

/// [ATtiny212](https://www.microchip.com/wwwproducts/en/ATtiny212)
#[cfg(feature = "attiny212")]
pub mod attiny212 {
    include!(concat!(env!("OUT_DIR"), "/pac/attiny212.rs"));
}

/// [ATtiny214](https://www.microchip.com/wwwproducts/en/ATtiny214)
#[cfg(feature = "attiny214")]
pub mod attiny214 {
    include!(concat!(env!("OUT_DIR"), "/pac/attiny214.rs"));
}

/// [ATtiny2313](https://www.microchip.com/wwwproducts/en/ATtiny2313)
#[cfg(feature = "attiny2313")]
pub mod attiny2313 {
    include!(concat!(env!("OUT_DIR"), "/pac/attiny2313.rs"));
}

/// [ATtiny2313A](https://www.microchip.com/wwwproducts/en/ATtiny2313A)
#[cfg(feature = "attiny2313a")]
pub mod attiny2313a {
    include!(concat!(env!("OUT_DIR"), "/pac/attiny2313a.rs"));
}

/// [ATtiny26](https://www.microchip.com/wwwproducts/en/ATtiny26)
#[cfg(feature = "attiny26")]
pub mod attiny26 {
    include!(concat!(env!("OUT_DIR"), "/pac/attiny26.rs"));
}

/// [ATtiny261a](https://www.microchip.com/wwwproducts/en/ATtiny261a)
#[cfg(feature = "attiny261a")]
pub mod attiny261a {
    include!(concat!(env!("OUT_DIR"), "/pac/attiny261a.rs"));
}

/// [ATtiny402](https://www.microchip.com/en-us/product/ATTINY402)
#[cfg(feature = "attiny402")]
pub mod attiny402 {
    include!(concat!(env!("OUT_DIR"), "/pac/attiny402.rs"));
}

/// [ATtiny404](https://www.microchip.com/en-us/product/ATTINY404)
#[cfg(feature = "attiny404")]
pub mod attiny404 {
    include!(concat!(env!("OUT_DIR"), "/pac/attiny404.rs"));
}

/// [ATtiny412](https://www.microchip.com/wwwproducts/en/ATtiny412)
#[cfg(feature = "attiny412")]
pub mod attiny412 {
    include!(concat!(env!("OUT_DIR"), "/pac/attiny412.rs"));
}

/// [ATtiny414](https://www.microchip.com/wwwproducts/en/ATtiny414)
#[cfg(feature = "attiny414")]
pub mod attiny414 {
    include!(concat!(env!("OUT_DIR"), "/pac/attiny414.rs"));
}

/// [ATtiny416](https://www.microchip.com/wwwproducts/en/ATtiny416)
#[cfg(feature = "attiny416")]
pub mod attiny416 {
    include!(concat!(env!("OUT_DIR"), "/pac/attiny416.rs"));
}

/// [ATtiny44a](https://www.microchip.com/en-us/product/ATtiny44a)
#[cfg(feature = "attiny44a")]
pub mod attiny44a {
    include!(concat!(env!("OUT_DIR"), "/pac/attiny44a.rs"));
}

/// [ATtiny461a](https://www.microchip.com/en-us/product/ATtiny461a)
#[cfg(feature = "attiny461a")]
pub mod attiny461a {
    include!(concat!(env!("OUT_DIR"), "/pac/attiny461a.rs"));
}

/// [ATtiny804](https://www.microchip.com/wwwproducts/en/ATtiny804)
#[cfg(feature = "attiny804")]
pub mod attiny804 {
    include!(concat!(env!("OUT_DIR"), "/pac/attiny804.rs"));
}

/// [ATtiny816](https://www.microchip.com/wwwproducts/en/ATtiny816)
#[cfg(feature = "attiny816")]
pub mod attiny816 {
    include!(concat!(env!("OUT_DIR"), "/pac/attiny816.rs"));
}

/// [ATtiny828](https://www.microchip.com/wwwproducts/en/ATtiny828)
#[cfg(feature = "attiny828")]
pub mod attiny828 {
    include!(concat!(env!("OUT_DIR"), "/pac/attiny828.rs"));
}

/// [ATtiny84](https://www.microchip.com/wwwproducts/en/ATtiny84)
#[cfg(feature = "attiny84")]
pub mod attiny84 {
    include!(concat!(env!("OUT_DIR"), "/pac/attiny84.rs"));
}

/// [ATtiny841](https://www.microchip.com/wwwproducts/en/ATtiny841)
#[cfg(feature = "attiny841")]
pub mod attiny841 {
    include!(concat!(env!("OUT_DIR"), "/pac/attiny841.rs"));
}

/// [ATtiny84a](https://www.microchip.com/en-us/product/ATtiny84a)
#[cfg(feature = "attiny84a")]
pub mod attiny84a {
    include!(concat!(env!("OUT_DIR"), "/pac/attiny84a.rs"));
}

/// [ATtiny85](https://www.microchip.com/wwwproducts/en/ATtiny85)
#[cfg(feature = "attiny85")]
pub mod attiny85 {
    include!(concat!(env!("OUT_DIR"), "/pac/attiny85.rs"));
}

/// [ATtiny861](https://www.microchip.com/wwwproducts/en/ATtiny861)
#[cfg(feature = "attiny861")]
pub mod attiny861 {
    include!(concat!(env!("OUT_DIR"), "/pac/attiny861.rs"));
}

/// [ATtiny861a](https://www.microchip.com/wwwproducts/en/ATtiny861a)
#[cfg(feature = "attiny861a")]
pub mod attiny861a {
    include!(concat!(env!("OUT_DIR"), "/pac/attiny861a.rs"));
}

/// [ATtiny88](https://www.microchip.com/wwwproducts/en/ATtiny88)
#[cfg(feature = "attiny88")]
pub mod attiny88 {
    include!(concat!(env!("OUT_DIR"), "/pac/attiny88.rs"));
}

/// [AVR64DU32](https://www.microchip.com/wwwproducts/en/AVR64DU32)
#[cfg(feature = "avr64du32")]
pub mod avr64du32 {
    include!(concat!(env!("OUT_DIR"), "/pac/avr64du32.rs"));
}

/// [AVR64DU28](https://www.microchip.com/wwwproducts/en/AVR64DU28)
#[cfg(feature = "avr64du28")]
pub mod avr64du28 {
    include!(concat!(env!("OUT_DIR"), "/pac/avr64du28.rs"));
}

/// [AVR128DB28](https://www.microchip.com/wwwproducts/en/AVR128DB28)
#[cfg(feature = "avr128db28")]
pub mod avr128db28 {
    include!(concat!(env!("OUT_DIR"), "/pac/avr128db28.rs"));
}
