cfg_if::cfg_if! {
    if #[cfg(feature = "atmega8")] {
        pub mod atmega8;
    } else if #[cfg(feature = "atmega328p")] {
        pub mod atmega328p;
    } else if #[cfg(feature = "atmega32u4")] {
        pub mod atmega32u4;
    } else if #[cfg(feature = "attiny85")] {
        pub mod attiny85;
    }
}
