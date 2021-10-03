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
///
/// Returns a bool, reflecting whether interrupts were enabled prior to calling this method.
pub fn disable() -> bool {
    cfg_if::cfg_if! {
        if #[cfg(target_arch = "avr")] {
            // Store current state
            let sreg: u8;
            unsafe { llvm_asm!("in $0,0x3F" :"=r"(sreg) ::: "volatile") };

            // Disable interrupts
            unsafe { llvm_asm!("cli" :::: "volatile") };

            sreg & 0x80 == 0x80
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
            // Disable interrupts
            let interrupts_enabled = disable();

            let r = f(unsafe { &CriticalSection::new() });

            // Restore interrupt state
            if interrupts_enabled {
                unsafe { enable(); }
            }

            r
        } else {
            let _ = f;
            unimplemented!()
        }
    }
}
