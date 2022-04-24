//! Assembly instructions

#[cfg(all(target_arch = "avr", avr_device_asm_macro))]
use core::arch::asm;

/// No Operation
#[inline(always)]
pub fn nop() {
    cfg_if::cfg_if! {
        if #[cfg(all(target_arch = "avr", avr_device_asm_macro))] {
            unsafe { asm!("nop") }
        } else if #[cfg(target_arch = "avr")] {
            unsafe { llvm_asm!("nop") }
        } else {
            unimplemented!()
        }
    }
}

/// Sleep / Wait For Interrupt
#[inline(always)]
pub fn sleep() {
    cfg_if::cfg_if! {
        if #[cfg(all(target_arch = "avr", avr_device_asm_macro))] {
            unsafe { asm!("sleep") }
        } else if #[cfg(target_arch = "avr")] {
            unsafe { llvm_asm!("sleep") }
        } else {
            unimplemented!()
        }
    }
}

/// Watchdog Reset
#[inline(always)]
pub fn wdr() {
    cfg_if::cfg_if! {
        if #[cfg(all(target_arch = "avr", avr_device_asm_macro))] {
            unsafe { asm!("wdr") }
        } else if #[cfg(target_arch = "avr")] {
            unsafe { llvm_asm!("wdr") }
        } else {
            unimplemented!()
        }
    }
}
