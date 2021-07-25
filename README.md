
# Rust driver for Rainbow HAT

[![Build Status](https://travis-ci.com/yannart/rainbow-hat-rs.svg?branch=master)](https://travis-ci.com/yannart/rainbow-hat-rs)
[![](http://meritbadge.herokuapp.com/rainbow-hat-rs)](https://crates.io/crates/rainbow-hat-rs)
[![MIT licensed](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)
[![Minimum rustc version](https://img.shields.io/badge/rustc-v1.45.0-lightgray.svg)](https://blog.rust-lang.org/2019/02/28/Rust-1.45.0.html)
[![Documentation](https://docs.rs/rainbow-hat-rs/badge.svg)](https://docs.rs/rainbow-hat-rs)

This repository contains an unofficial Rust driver library for [Rainbow HAT](https://shop.pimoroni.com/products/rainbow-hat-for-android-things), for use with Raspberry Pi OS on your Raspberry Pi.  
For the official Python driver see: https://github.com/pimoroni/rainbow-hat.  
For the official AndroidThings driver see: https://github.com/androidthings/contrib-drivers/tree/master/rainbowhat.  

This library depends on https://github.com/golemparts/rppal for access to the Raspberry Pi peripherals.  

Current periferials supported:
| Periferial                             | Supported | Structs                                |
|----------------------------------------|-----------|----------------------------------------|
| Multicolour LEDs                       | X         | rainbow_hat_rs::apa102::APA102         |
| Four 14-segment alphanumeric displays  | X         | rainbow_hat_rs::alphanum4::Alphanum4   |
| Three capacitive touch buttons         | X         | rainbow_hat_rs::touch::Buttons         |
| Temperature and pressure sensor        |           |                                        |
| Blue, green and red LEDs               | X         | rainbow_hat_rs::lights::Lights         |
| Piezo buzzer                           | X         | rainbow_hat_rs::buzzer::Buzzer         |

## References
* https://pinout.xyz/pinout/rainbow_hat

## Usage
Add a dependency for `rainbow-hat-rs` to your `Cargo.toml`.

```toml
[dependencies]
rainbow-hat-rs = "0.2.0"
```

Call `new()` on any of the peripherals to construct a new instance.

```rust
use rainbow_hat_rs::lights::Lights;
use rainbow_hat_rs::alphanum4::Alphanum4;
use rainbow_hat_rs::touch::Buttons;
use rainbow_hat_rs::apa102::APA102;
use rainbow_hat_rs::buzzer::Buzzer;

let mut apa102 = APA102::new()?;
let mut lights = Lights::new()?;
let mut buttons = Buttons::new()?;
let mut alphanum = Alphanum4::new()?;
let mut buzzer = Buzzer::new()?;
```

## Examples
See folder [examples](examples/README.md).

## Multicolour LEDs
```rust
let mut apa102 = APA102::new()?;

 // Sets color for all LED.
 apa102.set_all(255, 0, 0, 0.5);

 // Sets color for first LED.
 apa102.set_pixel(0, 0, 255, 0, 0.5);

// Shows on the device.
apa102.show()?;
```

### Lights

```rust
let mut lights = Lights::new()?;

// Turn on red and green lights
lights.rgb(true, true, false);

// Turn off red light
lights.red.off()

// Turn on blue light
lights.blue.on()

// Toggle green light
lights.green.toggle()
```

### Buttons
```rust
let mut buttons = Buttons::new()?;

// Identify if button A is pressed
if buttons.a.is_pressed() {
    println!("Button A touched!");
}
```

### Display
```rust
let mut alphanum = Alphanum4::new()?;

// Print a message on the display
alphanum.print_str("1234", false);
alphanum.show()?;
```

### Buzzer
```rust
let mut buzzer = Buzzer::new()?;

// Play a note
buzzer.midi_note(69, 0.3)?;
```

## Caution

Always be careful when working with the Raspberry Pi's peripherals, especially if you attach any external components to the GPIO pins. Improper use can lead to permanent damage.

## Copyright and license
Copyright (c) 2020 Yann Nicolas. Released under the [MIT license](LICENSE).
