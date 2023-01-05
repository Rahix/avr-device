//! Chip-Generic Interrupt Utilities
//!
//! For the most part, [crate::interrupt::free] is what you want:
//!
//! ```
//! avr_device::interrupt::free(|cs| {
//!     // Interrupts are disabled here
//! });
//! ```
//!
//! To access shared state, Mutex can be used:
//!
//! ```
//! use avr_device::interrupt::Mutex;
//! use core::cell::Cell;
//!
//! // Use Cell, if the wrapped type is Copy.
//! // Use RefCell, if the wrapped type is not Copy or if you need a reference to it for other reasons.
//! static MYGLOBAL: Mutex<Cell<u16>> = Mutex::new(Cell::new(0));
//!
//! fn my_fun() {
//!     avr_device::interrupt::free(|cs| {
//!         // Interrupts are disabled here
//!
//!         // Acquire mutex to global variable.
//!         let myglobal_ref = MYGLOBAL.borrow(cs);
//!         // Write to the global variable.
//!         myglobal_ref.set(42);
//!     });
//! }
//! ```

pub use bare_metal::{CriticalSection, Mutex};

#[cfg(target_arch = "avr")]
use core::arch::asm;

/// Opaque structure for storing the global interrupt flag status.
///
/// This structure does not implement `Copy` and `Clone`,
/// because the user shall not duplicate and pass it twice to [crate::interrupt::restore].
#[derive(Debug)]
#[cfg_attr(feature = "ufmt", derive(ufmt::derive::uDebug))]
pub struct IrqFlag {
    // The saved SREG.
    sreg: u8,
}

impl IrqFlag {
    #[inline(always)]
    fn new(sreg: u8) -> IrqFlag {
        IrqFlag { sreg }
    }

    /// Check the status of the saved global interrupt flag.
    ///
    /// Returns true, if the saved global interrupt flag is set (IRQs enabled).
    /// Otherwise returns false.
    ///
    /// This method can be used to check whether interrupts were enabled
    /// before the [crate::interrupt::disable_save] call.
    /// You probably shouldn't make your program behavior dependent on this state.
    /// Consider using a different design.
    #[inline(always)]
    pub fn enabled(&self) -> bool {
        self.sreg & 0x80 != 0
    }
}

/// Disable the global interrupt flag.
///
/// *Hint*: Most of the time you probably don't want to use this function directly.
///         Consider creating a critical section with [crate::interrupt::free] instead.
///
/// This function is an optimization fence.
/// That means memory accesses will not be re-ordered by the compiler across this function call.
#[inline(always)]
pub fn disable() {
    cfg_if::cfg_if! {
        if #[cfg(target_arch = "avr")] {
            // Disable interrupts
            unsafe { asm!("cli") };
        } else {
            unimplemented!()
        }
    }
}

/// Disable the global interrupt flag and return an opaque representation of the previous flag status.
///
/// *Hint*: Most of the time you probably don't want to use this function directly.
///         Consider creating a critical section with [crate::interrupt::free] instead.
///
/// This function is an optimization fence.
/// That means memory accesses will not be re-ordered by the compiler across this function call.
///
/// Returns an object that contains the status of the global interrupt flag from *before* the `disable_save()` call.
/// This object shall later be passed to the [crate::interrupt::restore] function.
#[inline(always)]
#[allow(unreachable_code)]
pub fn disable_save() -> IrqFlag {
    let sreg;
    cfg_if::cfg_if! {
        if #[cfg(target_arch = "avr")] {
            // Store current state
            unsafe {
                asm!(
                    "in {sreg}, 0x3F",
                    sreg = out(reg) sreg,
                )
            };
        } else {
            let _ = sreg;
            unimplemented!()
        }
    }
    // Disable interrupts
    disable();

    IrqFlag::new(sreg)
}

/// Enable the global interrupt flag.
///
/// *Warning*: This function enables interrupts, no matter what the enable-state was before [crate::interrupt::disable].
///            Especially in library code, where the previous interrupt state may be unknown,
///            this function call shall be avoided.
///            Most of the time you probably don't want to use this function directly.
///            Consider creating a critical section with [crate::interrupt::free] instead.
///
/// This function is an optimization fence.
/// That means memory accesses will not be re-ordered by the compiler across this function call.
///
/// # Safety
///
/// - Do not call this function inside an [crate::interrupt::free] critical section
#[inline(always)]
pub unsafe fn enable() {
    cfg_if::cfg_if! {
        if #[cfg(target_arch = "avr")] {
            asm!("sei");
        } else {
            unimplemented!()
        }
    }
}

/// Restore the global interrupt flag to its previous state before [crate::interrupt::disable_save].
///
/// *Hint*: Most of the time you probably don't want to use this function directly.
///         Consider creating a critical section with [crate::interrupt::free] instead.
///
/// This function is an optimization fence.
/// That means memory accesses will not be re-ordered by the compiler across this function call.
///
/// # Safety
///
/// - If you call this function inside of a [crate::interrupt::free] critical section, the
///   corresponding [crate::interrupt::disable_save] must also be in the same critical section.
/// - If you nest multiple [crate::interrupt::disable_save] + [crate::interrupt::restore]
///   sequences, the [crate::interrupt::restore] must be called in the reverse order of the
///   [crate::interrupt::disable_save] call order.
///   That means the first saved IrqFlag must be restored last.
#[inline(always)]
pub unsafe fn restore(irq_flag: IrqFlag) {
    cfg_if::cfg_if! {
        if #[cfg(target_arch = "avr")] {
            // Restore global interrupt flag in SREG.
            // This also clobbers all other bits in SREG.
            asm!(
                "out 0x3F, {sreg}",
                sreg = in(reg) irq_flag.sreg,
            );
        } else {
            let _ = irq_flag;
            unimplemented!()
        }
    }
}

/// Check whether the global interrupt flag is currently enabled (in SREG).
///
/// *Warning*: You shouldn't use this to hand craft your own memory/interrupt safety mechanisms.
///            This function may be used for things such as deciding whether to do
///            expensive calculations in library code, or similar things.
///
/// This function is **not** an optimization fence.
/// That means memory accesses *can* be re-ordered by the compiler across this function call.
#[inline(always)]
#[allow(unreachable_code)]
pub fn is_enabled() -> bool {
    let sreg;
    cfg_if::cfg_if! {
        if #[cfg(target_arch = "avr")] {
            // Store current state
            unsafe {
                asm!(
                    "in {sreg}, 0x3F",
                    sreg = out(reg) sreg,
                    options(readonly, preserves_flags, nostack),
                )
            };
        } else {
            let _ = sreg;
            unimplemented!()
        }
    }

    IrqFlag::new(sreg).enabled()
}

/// Execute closure `f` in an interrupt-free context.
///
/// This as also known as a "critical section".
#[inline(always)]
pub fn free<F, R>(f: F) -> R
where
    F: FnOnce(CriticalSection) -> R,
{
    cfg_if::cfg_if! {
        if #[cfg(target_arch = "avr")] {
            // Disable interrupts. This is an optimization fence.
            let irq_flag = disable_save();

            let r = f(unsafe { CriticalSection::new() });

            // Restore interrupt state. This is an optimization fence.
            unsafe { restore(irq_flag); }

            r
        } else {
            let _ = f;
            unimplemented!()
        }
    }
}

#[cfg(feature = "critical-section-impl")]
mod cs {
    use critical_section::RawRestoreState;

    struct AvrCriticalSection;
    critical_section::set_impl!(AvrCriticalSection);

    unsafe impl critical_section::Impl for AvrCriticalSection {
        unsafe fn acquire() -> RawRestoreState {
            crate::interrupt::disable_save().sreg
        }

        unsafe fn release(restore_state: RawRestoreState) {
            crate::interrupt::restore(crate::interrupt::IrqFlag::new(restore_state))
        }
    }
}
