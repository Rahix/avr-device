//! Chip-Generic Interrupt Utilities
//!
//! For the most part, [crate::interrupt::free] is what you want:
//!
//! ```
//! atmega32u4::interrupt::free(|cs| {
//!     // Interrupts are disabled here
//! })
//! ```

pub use bare_metal::{CriticalSection, Mutex, Nr};

#[inline]
/// Disables all interrupts
pub fn disable() {
    cfg_if::cfg_if! {
        if #[cfg(target_arch = "avr")] {
            unsafe { llvm_asm!("cli" :::: "volatile") };
        } else {
            unimplemented!()
        }
    }
}

#[inline]
/// Enables all the interrupts
///
/// # Safety
///
/// - Do not call this function inside an [crate::interrupt::free] critical section
pub unsafe fn enable() {
    cfg_if::cfg_if! {
        if #[cfg(target_arch = "avr")] {
            llvm_asm!("sei" :::: "volatile");
        } else {
            unimplemented!()
        }
    }
}

/// Execute closure `f` in an interrupt-free context.
///
/// This as also known as a "critical section".
pub fn free<F, R>(f: F) -> R
where
    F: FnOnce(&CriticalSection) -> R,
{
    cfg_if::cfg_if! {
        if #[cfg(target_arch = "avr")] {
            let sreg: u8;

            // Store current state
            unsafe {
                llvm_asm!("in $0,0x3F" :"=r"(sreg) ::: "volatile");
            }

            // Disable interrupts
            disable();

            let r = f(unsafe { &CriticalSection::new() });

            // Restore interrupt state
            if sreg & 0x80 != 0x00 {
                unsafe { enable(); }
            }

            r
        } else {
            let _ = f;
            unimplemented!()
        }
    }
}
