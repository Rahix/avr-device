cfg_if::cfg_if! {
    if #[cfg(feature = "atmega32u4")] {
        pub mod atmega32u4;
    } else if #[cfg(feature = "attiny85")] {
        pub mod attiny85;
    }
}
