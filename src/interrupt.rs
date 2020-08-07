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
    unsafe {
        llvm_asm!(
            "cli" :::: "volatile"
        );
    }
}

#[inline]
/// Enables all the interrupts
///
/// # Safety
///
/// - Do not call this function inside an [crate::interrupt::free] critical section
pub unsafe fn enable() {
    llvm_asm!(
        "sei" :::: "volatile"
    );
}

/// Execute closure `f` in an interrupt-free context.
///
/// This as also known as a "critical section".
pub fn free<F, R>(f: F) -> R
where
    F: FnOnce(&CriticalSection) -> R,
{
    let sreg: u8;

    // Store current state
    unsafe {
        llvm_asm!(
            "in $0,0x3F"
            : "=r"(sreg)
            :
            :
            : "volatile"
        );
    }

    // Disable interrupts
    disable();

    let r = f(unsafe { &CriticalSection::new() });

    // Restore interrupt state
    if sreg & 0x80 != 0x00 {
        unsafe { enable(); }
    }

    r
}
