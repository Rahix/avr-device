//! This crate contains register definitions for
#![cfg_attr(feature = "atmega1280", doc = "**atmega1280**,")]
#![cfg_attr(feature = "atmega8", doc = "**atmega8**,")]
#![cfg_attr(feature = "atmega328p", doc = "**atmega328p**,")]
#![cfg_attr(feature = "atmega32u4", doc = "**atmega32u4**,")]
#![cfg_attr(feature = "atmega64", doc = "**atmega64**,")]
#![cfg_attr(feature = "attiny85", doc = "**attiny85**,")]
//! and a few things which apply to AVR microcontrollers generally.
//!
//! Which chips the crate is built for depends on the feature flag used.
//! The following chips are available (using feature flags of the same name):
//! * `atmega1280`
//! * `atmega8`
//! * `atmega328p`
//! * `atmega32u4`
//! * `atmega64`
//! * `attiny85`
#![no_std]
#![feature(llvm_asm)]

pub mod interrupt;

#[allow(unused_imports)]
use generic::*;
#[doc = r"Common register and bit access and modify traits"]
pub mod generic;

#[cfg(feature = "rt")]
pub use avr_device_macros::interrupt;

#[allow(non_camel_case_types, unused_attributes, unreachable_patterns)]
mod devices;

#[cfg(feature = "atmega1280")]
pub use crate::devices::atmega1280;
#[cfg(feature = "atmega328p")]
pub use crate::devices::atmega328p;
#[cfg(feature = "atmega32u4")]
pub use crate::devices::atmega32u4;
#[cfg(feature = "atmega8")]
pub use crate::devices::atmega8;
#[cfg(feature = "atmega64")]
pub use crate::devices::atmega64;
#[cfg(feature = "attiny85")]
pub use crate::devices::attiny85;

#[cfg(not(any(
    feature = "atmega1280",
    feature = "atmega8",
    feature = "atmega328p",
    feature = "atmega32u4",
    feature = "atmega64",
    feature = "attiny85",
)))]
compile_error!("You need to select at least one chip as a feature!");
