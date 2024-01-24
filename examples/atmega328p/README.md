# ATmega328P Example
## Overview
This example showcases a minimal Rust program that solely utilizes bare
register writes without relying on a Hardware Abstraction Layer.  If you want
to use a HAL, you can use [`avr-hal`], which uses `avr-device` under the hood.

[`avr-hal`]: https://github.com/Rahix/avr-hal

## Demonstrated Features
The primary purpose of this example is to illustrate how to interact directly
with AVR microcontroller registers without using a higher-level HAL.

This is of course more painful, but can be interesting in order to gain
insights into the low-level details of AVR programming and understand the
intricacies of register manipulation.

## Example Setup
This example uses an ATmega328P with two LEDs attached (through an adequately
sized resistor) to ground. The LED pins are:

- `PD2` for the green led
- `PD3` for the red led

This example demonstrates three things:

### Panic Handler
A custom panic handler which blinks the red LED connected to `PD3`
indefinitely.

This panic handler is called on the main loop by the `panic!()` macro once the
counter reaches 10.

### Global Variables
A global variable named `LED_STATE` is shared between the timer interrupt
function and the main loop. The variable is a `Mutex<Cell<bool>>`. Being a
mutex, the variable can only being accessed through a critical section. A
critical section is created using the `interrupt::free` function which ensures
that the closure is run in an interrupt free context.

### Timer Interrupt
The `TIMER0_OVF` function is called as an interrupt handler when the `timer0`
timer overflows. Every 16 overflows (as stored into the  `OVF_COUNTER` static
variable), it toggles the state of the `LED_STATE` variable.

### Main Loop
The main loop just watches the state of the `LED_STATE` variable and sets the
green led (`PD3`) according to the state of the boolean.

Every time this variable changes, a counter is incremented. When it reaches 10,
a panic is raised and our custom panic handler is called.

## Trying it out
1. First of all, check the README of the [`avr-hal`][avr-hal-readme] crate for
   an overview of required build dependencies.
2. Change into the example directory:
   ```bash
   cd examples/atmega328p/
   ```
3. Build the program:
   ```bash
   cargo build
   ```
4. If you want to run the example on an Arduino Uno board, you can use
   [`ravedude`] to flash the microcontroller.  The project is preconfigured for
   this, so all you need to do is:
   ```bash
   cargo run
   ```
   You can configure a different ATmega328P board by editing the
   `.cargo/config.toml` file.

[avr-hal-readme]: https://github.com/Rahix/avr-hal#quickstart
[`ravedude`]: https://github.com/Rahix/avr-hal/tree/main/ravedude
