#![no_std]
#![no_main]
#![feature(abi_avr_interrupt)]

use core::cell::Cell;

use avr_device::interrupt::{self, Mutex};

static LED_STATE: Mutex<Cell<bool>> = Mutex::new(Cell::new(true));

// Our panic handler. It hangs forever while making the PD3 pin blink.
// In a real world use-case, this can be used to put the device into a
// fail-state mode (shutting off motors, power, ...)
#[cfg(not(doc))]
#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    // disable interrupts - firmware has panicked so no ISRs should continue running
    avr_device::interrupt::disable();

    // get the peripherals so we can access serial and the LED.
    //
    // SAFETY: Because main() already has references to the peripherals this is an unsafe
    // operation - but because no other code can run after the panic handler was called,
    // we know it is okay.
    let dp = unsafe { avr_device::atmega328p::Peripherals::steal() };

    loop {
        avr_device::asm::delay_cycles(1_000_000);
        dp.PORTD.portd.write(|w| w.pd3().set_bit());
        avr_device::asm::delay_cycles(1_000_000);
        dp.PORTD.portd.write(|w| w.pd3().clear_bit());
    }
}

#[avr_device::interrupt(atmega328p)]
fn TIMER0_OVF() {
    // This interrupt should raise every (1024*255)/16MHz s ≈ 0.01s
    // We then count 61 times to approximate 1s.
    // XXX: this is a really bad way to count time

    static mut OVF_COUNTER: u16 = 0;
    const ROLLOVER: u16 = 61;

    *OVF_COUNTER = OVF_COUNTER.wrapping_add(1);
    if *OVF_COUNTER > ROLLOVER {
        *OVF_COUNTER = 0;

        interrupt::free(|cs| {
            LED_STATE.borrow(cs).set(!LED_STATE.borrow(cs).get());
        });
    }
}

#[avr_device::entry]
fn main() -> ! {
    let dp = avr_device::atmega328p::Peripherals::take().unwrap();

    // As you can see, we use .write() instead of .modify(), so the register
    // will be written value + the modified bits

    // Divide by 1024 -> 16MHz/1024 = 15.6kHz
    dp.TC0.tccr0b.write(|w| w.cs0().prescale_1024());
    // Enable overflow interrupts
    dp.TC0.timsk0.write(|w| w.toie0().set_bit());

    // Make pd2 and pd3 outputs
    // We use .modify() in order not to change the other bits
    dp.PORTD.ddrd.modify(|_, w| w.pd2().set_bit());
    dp.PORTD.ddrd.modify(|_, w| w.pd3().set_bit());

    // SAFETY: We can enable the interrupts here as we are not inside
    // a critical section.
    unsafe {
        avr_device::interrupt::enable();
    }

    let mut counter = 0;
    let mut previous_state: bool = true;
    loop {
        let mut led_state: bool = true;
        interrupt::free(|cs| {
            led_state = LED_STATE.borrow(cs).get();
        });

        dp.PORTD.portd.modify(|_, w| w.pd2().bit(led_state));

        // We want to make the program crash after 9 blinks
        if previous_state != led_state {
            counter += 1;
        }
        if counter > 9 {
            // The following panics, but it could also be
            // a more "general" bug like dividing by zero, out of bounds,
            // etc..
            panic!();
        }
        previous_state = led_state;
    }
}
