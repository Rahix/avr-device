//! Assembly instructions

#[cfg(target_arch = "avr")]
use core::arch::asm;

/// No Operation
#[inline(always)]
pub fn nop() {
    cfg_if::cfg_if! {
        if #[cfg(target_arch = "avr")] {
            unsafe { asm!("nop") }
        } else {
            unimplemented!()
        }
    }
}

/// Sleep / Wait For Interrupt
#[inline(always)]
pub fn sleep() {
    cfg_if::cfg_if! {
        if #[cfg(target_arch = "avr")] {
            unsafe { asm!("sleep") }
        } else {
            unimplemented!()
        }
    }
}

/// Watchdog Reset
#[inline(always)]
pub fn wdr() {
    cfg_if::cfg_if! {
        if #[cfg(target_arch = "avr")] {
            unsafe { asm!("wdr") }
        } else {
            unimplemented!()
        }
    }
}
