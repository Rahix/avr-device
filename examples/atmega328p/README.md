# ATMEGA328p example


## Overview

This example showcases a minimal Rust program that solely utilizes 
bare register writes without relying on a Hardware Abstraction Layer 
abstraction. If you want to use a HAL, you can use 
[avr-hal](https://github.com/Rahix/avr-hal), which uses `avr-device` 
under the hood.


## Demonstrated features

The primary purpose of this example is to illustrate how to interact directly with AVR 
microcontroller registers without using a higher-level HAL.

This is of course more painful, but can be interesting in order to gain insights into 
the low-level details of AVR programming and understand the intricacies of register manipulation.

## Example setup

The following exmple uses an ATmega328p with two LEDs attached (through an adequately sized 
resistor) to ground. The LED pins are:

- `PD2` for the green led
- `PD3` for the red led


Basically, this example program has three main features:

### Panic handler

A custom panic handler stops the main loop and interrupions. Afterwards, it toggles the `PD3` 
output indefinitely.

This panic handler is called on the main loop by the `panic!()` macro once the counter reaches 10.

### Global variable

A global variable named `LED_STATE` is shared between the timer interrupt function and the main loop.
The variable is a `Mutex<Cell<bool>>`. Being a mutex, the variable can only being accessed through a 
critical section. In our case, it's done through the `interrupt::free` function which ensures that the 
closure is run in an interrupt free context.

### Timer interrupt

The `TIMER0_OVF` function is called as an interrupt handler when the `timer0` timer overflows.
Every 16 overflows (as stored into the  `OVF_COUNTER` static variable), it toggles the state of
the `LED_STATE` variable.

### Main loop

The main loop just watches the state of the `LED_STATE` variable and sets the green led (PD3) 
according to the state of the boolean.

Every time this variable changes, a counter is incremented. When it reaches 10, a panic is raised
and our custom panic handler is called.


## Trying it out

First of all, read the README of the 
[avr-hal](https://github.com/Rahix/avr-hal#quickstart)
crate, as it contains a nice summary.

Change into the example directory:

```bash
cd examples/atmega328p/
```


Build the program:
```bash
cargo build
```

If your atmega328p is on an arduino board, you can use [ravedude](https://github.com/Rahix/avr-hal/tree/main/ravedude)
to flash the microcontroller. Simply type:
```bash
cargo run
```

You can configure the board by editting the `.cargo/config.toml` file.

