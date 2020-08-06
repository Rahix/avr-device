//! Assembly instructions

/// No Operation
#[inline(always)]
pub fn nop() {
    unsafe { llvm_asm!("nop") }
}

/// Sleep / Wait For Interrupt
#[inline(always)]
pub fn sleep() {
    unsafe { llvm_asm!("sleep") }
}

/// Watchdog Reset
#[inline(always)]
pub fn wdr() {
    unsafe { llvm_asm!("wdr") }
}
