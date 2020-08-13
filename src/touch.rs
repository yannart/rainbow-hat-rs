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

    /// Output pin to read from GPIO. Optional as not used in simulated mode.
    pin: Option<Box<InputPin>>,

    /// State of the button: true for pressed, false for released
    state: bool,

    /// In simulation mode, no interaction with the hardware is done to simplify testability.
    simulation: bool, 

    /// is the setup completed
    is_setup: bool,
}

impl Button {

    /// Creates a touch for the GPIO number.
    /// # Arguments
    ///
    /// * `bcm_pin` - GPIO pin number using the BCM pin numbering.
    pub fn new(bcm_pin: u8) -> Result<Button, Error> {

        Ok(Self {
            bcm_pin,
            pin: None,
            state: false,
            simulation: false,
            is_setup: false,
        })
    }

    /// Initialize driver.
    pub fn setup(&mut self) -> Result <(), Error> {
        if !self.is_setup {

            // Ignore Gpio initialization if in sumulation mode
            if !self.simulation {
                let gpio = Gpio::new()?;
                let input = gpio.get(self.bcm_pin)?.into_input();
                self.pin = Some(Box::new(input));
            }

            self.is_setup = true;
        }
        Ok(())
    }

    /// Get the state of the touch button.
    /// returns true if the touch button is pressed or false if it is not.
    pub fn is_pressed(&mut self) -> bool {

        // Initialize the Gpio reading if not done yet
        if !self.is_setup {
            let _result = self.setup();
        }

        // Only perform actual pin write if not in simulation mode
        if !self.simulation {
            let pin = self.pin.as_deref_mut().unwrap();

            // Touched if the pin is low
            self.state =!pin.is_high();
        }

        self.state
    }
}

/// Set of buttons on the board.
pub struct Buttons {

    /// Button A
    pub a : Button,

    /// Button B
    pub b: Button,

    /// Button C
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

    /// Enables simulation mode.
    pub fn enable_simulation(&mut self) {
        self.a.simulation = true;
        self.b.simulation = true;
        self.c.simulation = true;
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

/// Unit tests
#[cfg(test)]
mod tests {
    use super::*;

    /// Tests the setup of the button.
    #[test]
    fn test_button_setup() -> Result<(), Error> {
        let mut button = Button::new(GPIO_TOUCH_A)?;

        // enable simulation
        button.simulation = true;

        // Not setup
        assert!(button.is_setup == false);

        // Force setup
        let _result = button.setup();

        assert!(button.is_setup == true);

        Ok(())
    }

    /// Tests when a button is pressed.
    #[test]
    fn test_button_is_pressed() -> Result<(), Error> {
        let mut button = Button::new(GPIO_TOUCH_A)?;

        // enable simulation
        button.simulation = true;

        // Not setup
        assert!(button.is_setup == false);

        // Lazy setup
        // For simulation the button is not pressed by default
        assert!(button.is_pressed() == false);
        assert!(button.is_setup == true);

        // Force the state
        button.state = true;
        assert!(button.is_pressed() == true);

        Ok(())
    }

    /// Tests the setup of the button.
    #[test]
    fn test_buttons_new() -> Result<(), Error> {
        let buttons = Buttons::new()?;

        // Verify the buttons use the right pin
        assert!(buttons.a.bcm_pin == 21);
        assert!(buttons.b.bcm_pin == 20);
        assert!(buttons.c.bcm_pin == 16);

        Ok(())
    }

    /// Tests to enable the simulation.
    #[test]
    fn test_buttons_enable_simulation() -> Result<(), Error> {
        let mut buttons = Buttons::new()?;

        // Simulation off by default
        assert!(!buttons.a.simulation);
        assert!(!buttons.b.simulation);
        assert!(!buttons.c.simulation);

        // Turn on simulation
        buttons.enable_simulation();

        assert!(buttons.a.simulation);
        assert!(buttons.b.simulation);
        assert!(buttons.c.simulation);

        Ok(())
    }
}
