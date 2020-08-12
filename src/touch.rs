use std::fmt;
use rppal::gpio::{Gpio, InputPin};

/// GPIO BCM pin number for the touch button A.
pub const GPIO_TOUCH_A: u8 = 21;

/// GPIO BCM pin number for the touch button B.
pub const GPIO_TOUCH_B: u8 = 20;

/// GPIO BCM pin number for the touch button C.
pub const GPIO_TOUCH_C: u8 = 16;

/// Touch button on the board.
#[derive(Debug)]
pub struct Button {
    bcm_pin: u8,
    pin: InputPin,
}

impl Button {

    /// Creates a touch for the GPIO number.
    /// # Arguments
    ///
    /// * `bcm_pin` - GPIO pin number using the BCM pin numbering.
    pub fn new(bcm_pin: u8) -> Result<Button, Error> {
        let gpio = Gpio::new()?;
        let pin = gpio.get(bcm_pin)?.into_input();

        Ok(Self {
            bcm_pin,
            pin,
        })
    }

    /// Get the state of the touch button.
    /// returns true if the touch button is pressed or false if it is not.
    pub fn is_pressed(&mut self) -> bool {
        // Touched if the pin is low
        !self.pin.is_high()
    }
}

/// Set of buttons on the board.
pub struct Buttons {
    pub a : Button,
    pub b: Button,
    pub c: Button,
}

impl Buttons {

    /// Creates a the set of buttons.
    pub fn new() -> Result<Buttons, Error> {
        Ok(Self {
            a: Button::new(GPIO_TOUCH_A)?,
            b: Button::new(GPIO_TOUCH_B)?,
            c: Button::new(GPIO_TOUCH_C)?,
        })
    }
}

/// Errors that can occur.
#[derive(Debug)]
pub enum Error {

    /// Gpio error.
    Gpio(rppal::gpio::Error),
}

impl std::error::Error for Error {}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &*self {
            Error::Gpio(err) => write!(f, "Gpio error: {}", &err),
        }
    }
}

/// Converts Gpio error
impl From<rppal::gpio::Error> for Error {
    fn from(err: rppal::gpio::Error) -> Error {
        Error::Gpio(err)
    }
}
