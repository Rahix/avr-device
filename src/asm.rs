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

/// Get the stack size in bytes, for debbuging.
#[cfg(feature = "rt")]
#[inline(always)]
pub fn get_stack_size() -> usize {
    extern "C" {
        static __DATA_REGION_ORIGIN__: usize;
        static __DATA_REGION_LENGTH__: usize;
    }
    
    use core::ptr::addr_of;

    cfg_if::cfg_if! {
        if #[cfg(target_arch = "avr")] {
            unsafe {
                let data_region_end = addr_of!(__DATA_REGION_ORIGIN__) as usize + addr_of!(__DATA_REGION_LENGTH__) as usize;
                let sp: usize;
                if data_region_end > u8::MAX as usize {
                    // Here the stack pointer is 2 bytes.

                    let spl: u8;
                    let sph: u8;
                    core::arch::asm!(
                        "in {}, 0x3d",
                        "in {}, 0x3e",
                        out(reg) spl,
                        out(reg) sph,
                        options(nostack, nomem, pure),
                    );
                    sp = usize::from_le_bytes([spl, sph]);
                } else {
                    // Here the stack pointer is 1 byte.

                    let spl: u8;
                    core::arch::asm!(
                        "in {}, 0x3d",
                        out(reg) spl,
                        options(nostack, nomem, pure),
                    );
                    sp = spl as usize;
                }
                data_region_end - sp
            }
        } else {
            unimplemented!()
        }
    }
}

/// Blocks the program for at least `cycles` CPU cycles.
///
/// This is intended for very simple delays in low-level drivers, but it
/// has some caveats:
///
/// - The delay may be significantly longer if an interrupt is serviced at the
///   same time, since the delay loop will not be executing during the interrupt.
///   If you need precise timing, use a hardware timer peripheral instead.
///
/// - The real-time delay depends on the CPU clock frequency. If you want to
///   conveniently specify a delay value in real-time units like microseconds,
///   then use the `delay` module in the HAL crate for your platform.
#[inline(always)]
pub fn delay_cycles(cycles: u32) {
    cfg_if::cfg_if! {
        if #[cfg(target_arch = "avr")] {
            let mut cycles_bytes = cycles.to_le_bytes();
            // Each loop iteration takes 6 cycles when the branch is taken,
            // and 5 cycles when the branch is not taken.
            // So, this loop is guaranteed to run for at least `cycles - 1`
            // cycles, and there will be approximately 4 cycles before the
            // loop to initialize the counting registers.
            unsafe {
                asm!(
                    "1:",
                    "subi {r0}, 6",
                    "sbci {r1}, 0",
                    "sbci {r2}, 0",
                    "sbci {r3}, 0",
                    "brcc 1b",

                    r0 = inout(reg_upper) cycles_bytes[0],
                    r1 = inout(reg_upper) cycles_bytes[1],
                    r2 = inout(reg_upper) cycles_bytes[2],
                    r3 = inout(reg_upper) cycles_bytes[3],
                )
            }
        } else {
            let _ = cycles;
            unimplemented!()
        }
    }
}
