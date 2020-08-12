use std::fmt;
use core::fmt::Debug;
use rppal::gpio::{Gpio, OutputPin, Level};

/// GPIO BCM pin number for the red light.
pub const GPIO_LIGHT_RED: u8 = 6;

/// GPIO BCM pin number for the green light.
pub const GPIO_LIGHT_GREEN: u8 = 19;

/// GPIO BCM pin number for the blue light.
pub const GPIO_LIGHT_BLUE: u8 = 26;

/// Light on the board.
#[derive(Debug)]
pub struct Light {

    /// GPIO pin number using the BCM pin numbering.
    pub bcm_pin: u8,

    /// Output pin to write to GPIO. Optional as not used in simulated mode.
    pin: Option<Box<OutputPin>>,

    /// State of the light: true for on, false for Off
    pub state: bool,

    /// In simulation mode, no interaction with the hardware is done to simplify testability.
    simulation: bool, 

    /// is the setup completed
    is_setup: bool,
}

impl Light {
    
    /// Creates a light for the GPIO number.
    /// # Arguments
    ///
    /// * `bcm_pin` - GPIO pin number using the BCM pin numbering.
    pub fn new(bcm_pin: u8) -> Result<Light, Error>  {     

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
                let output = gpio.get(self.bcm_pin)?.into_output(); 
                self.pin = Some(Box::new(output));
            }

            self.is_setup = true;
        }
        Ok(())
    }

    /// Sets the light state to on.
    pub fn on(&mut self) {
        self.write(true);
    }

    /// Sets the light state to off.
    pub fn off(&mut self) {
        self.write(false);
    }

    /// Toggles the light state between on and off.
    pub fn toggle(&mut self) {
        self.write(!self.state);
    }

    /// Set the light state.
    /// # Arguments
    ///
    /// * `state` - State of the light: true for on, false for Off.
    pub fn write(&mut self, state: bool) {
        self.state = state;

        if !self.is_setup {
            let _result = self.setup();
        }

        // Only perform actual pin write if not in simulation mode
        if !self.simulation {

            let pin = self.pin.as_deref_mut().unwrap();

            if state {
                pin.write(Level::High);
            } else {
                pin.write(Level::Low);
            }

        }
    }
}

/// Set of lights on the board.
pub struct Lights {

    /// Red light.
    pub red : Light,

    /// Green light.
    pub green: Light,

    /// Blue light.
    pub blue: Light,
}

impl Lights {

    /// Creates a the set of Lights.
    pub fn new() -> Result<Lights, Error> {
        Ok(Self {
            red: Light::new(GPIO_LIGHT_RED)?,
            green: Light::new(GPIO_LIGHT_GREEN)?,
            blue: Light::new(GPIO_LIGHT_BLUE)?,
        })
    }

    /// Set the state for all the lights.
    /// # Arguments
    ///
    /// * `state` - State of the lights: true for on, false for Off.
    pub fn all(&mut self, state: bool) {
        self.red.write(state);
        self.green.write(state);
        self.blue.write(state);
    }

    /// Set the state for each light.
    /// # Arguments
    ///
    /// * `r` - State of the red light: true for on, false for Off.
    /// * `g` - State of the green light: true for on, false for Off.
    /// * `b` - State of the blue light: true for on, false for Off.
    pub fn rgb(&mut self, r: bool, g: bool, b: bool) {
        self.red.write(r);
        self.green.write(g);
        self.blue.write(b);
    }

    /// Enbles simulation mode.
    pub fn enable_simulation(&mut self) {
        self.red.simulation = true;
        self.green.simulation = true;
        self.blue.simulation = true;
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

    /// Tests the setup of the light.
    #[test]
    fn test_light_setup() -> Result<(), Error> {
        let mut light = Light::new(GPIO_LIGHT_RED)?;
        // enable simulation
        light.simulation = true;

        // Not setup
        assert!(light.is_setup == false);

        // Force setup
        let _result = light.setup();

        assert!(light.is_setup == true);

        Ok(())
    }

    /// Tests turning on a light.
    #[test]
    fn test_light_on() -> Result<(), Error> {
        let mut light = Light::new(GPIO_LIGHT_RED)?;

        // enable simulation
        light.simulation = true;

        // Off by default
        assert!(light.state == false);

        // Turn on
        light.on();
        assert!(light.state == true);

        Ok(())
    }

    /// Tests turning off a light.
    #[test]
    fn test_light_off() -> Result<(), Error> {
        let mut light = Light::new(GPIO_LIGHT_RED)?;

        // enable simulation
        light.simulation = true;

        // Turn on
        light.state = true;

        // Turn off
        light.off();
        assert!(light.state == false);

        Ok(())
    }

    /// Tests toggling a light.
    #[test]
    fn test_light_toggle() -> Result<(), Error> {
        let mut light = Light::new(GPIO_LIGHT_RED)?;

        // enable simulation
        light.simulation = true;

        light.toggle();
        assert!(light.state == true);

        light.toggle();
        assert!(light.state == false);

        Ok(())
    }
    
    /// Tests defining the state of each of the lights.
    #[test]
    fn test_lights_rgb() -> Result<(), Error> {
        let mut lights = Lights::new()?;

        // enable simulation
        lights.enable_simulation();

        // Test all combinations of rgb light states.
        let bool_array: [bool; 2] = [true, false];
        for red_state in &bool_array {
            for green_state in &bool_array {
                for blue_state in &bool_array {

                    // Turn on red and blue and green off and assert the state is correct.
                    lights.rgb(*red_state, *green_state, *blue_state);
                    assert!(lights.red.state == *red_state);
                    assert!(lights.green.state == *green_state);
                    assert!(lights.blue.state == *blue_state);
                }
            }
        }

        Ok(())
    }

    /// Tests turning on or off all the lights.
    #[test]
    fn test_lights_all() -> Result<(), Error> {
        let mut lights = Lights::new()?;

        // Turn on simulation
        lights.enable_simulation();

        // Test turning all lights on and off.
        lights.all(true);
        assert!(lights.red.state == true);
        assert!(lights.green.state == true);
        assert!(lights.blue.state == true);

        lights.all(false);
        assert!(lights.red.state == false);
        assert!(lights.green.state == false);
        assert!(lights.blue.state == false);

        Ok(())
    }

    /// Tests to enable the simulation.
    #[test]
    fn test_lights_enable_simulation() -> Result<(), Error> {
        let mut lights = Lights::new()?;

        // Simulation off by default
        assert!(!lights.red.simulation);
        assert!(!lights.green.simulation);
        assert!(!lights.blue.simulation);

        // Turn on simulation
        lights.enable_simulation();

        assert!(lights.red.simulation);
        assert!(lights.green.simulation);
        assert!(lights.blue.simulation);

        Ok(())
    }
}
