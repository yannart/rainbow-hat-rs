use std::error;
use std::fmt;
use rppal::i2c::I2c;

pub const DEFAULT_ADDRESS: u16 = 0x70;
pub const HT16K33_BLINK_CMD: u8 = 0x80;
pub const HT16K33_BLINK_DISPLAYON: u8 = 0x01;
pub const HT16K33_BLINK_OFF: u8 = 0x00;
pub const HT16K33_BLINK_2HZ: u8 = 0x02;
pub const HT16K33_BLINK_1HZ: u8 = 0x04;
pub const HT16K33_BLINK_HALFHZ: u8 = 0x06;
pub const HT16K33_SYSTEM_SETUP: u8 = 0x20;
pub const HT16K33_OSCILLATOR: u8 = 0x01;
pub const HT16K33_CMD_BRIGHTNESS: u8 = 0xE0;

/// Driver for interfacing with a Holtek HT16K33 16x8 LED driver.
#[derive(Debug)]
pub struct HT16K33 {

    /// Address of i2c
    i2c_address: u16,

    /// I2C. Optional as not used in simulated mode.
    i2c: Option<Box<I2c>>,

    /// buffer with data to be printed
    pub buffer: [u8; 8],

    /// frequency for blink: one of HT16K33_BLINK_OFF, HT16K33_BLINK_2HZ, HT16K33_BLINK_1HZ, HT16K33_BLINK_HALFHZ
    blink_frequency: u8,

    /// brightness between 0 and 15
    brightness: u8,

    /// In simulation mode, no interaction with the hardware is done to simplify testability.
    simulation: bool, 

    /// is the setup completed
    is_setup: bool
}

impl HT16K33 {

    /// Create an HT16K33 driver for device.
    /// Uses the specified I2C address (defaults to 0x70) and I2C device.
    pub fn new() -> Result<HT16K33, Error> {

        Ok(Self {
            i2c_address: DEFAULT_ADDRESS,
            i2c: None,
            buffer:[0; 8],
            blink_frequency: HT16K33_BLINK_OFF,
            brightness: 15 as u8,
            simulation: false,
            is_setup: false,
         })
    }

    /// Encapsulates block write to I2C bus.
    ///
    /// # Arguments
    ///
    /// * `command` - Command to write.
    /// * `buffer` - Buffer to write.
    fn i2c_block_write(&mut self, command: u8, buffer: &[u8]) -> Result <(), Error> {

        if !self.simulation {

            let i2c = self.i2c.as_deref_mut().unwrap();
            i2c.block_write(command, buffer)?;
        }

        Ok(())
    }

    /// Initialize driver with LEDs enabled and all turned off.
    fn setup(&mut self) -> Result <(), Error> {

        if !self.is_setup {
            if !self.simulation {

                let mut i2c = I2c::new()?;

                // Set the I2C slave address to the device we're communicating with.
                i2c.set_slave_address(self.i2c_address)?;

                i2c.block_write(
                    (HT16K33_SYSTEM_SETUP | HT16K33_OSCILLATOR) as u8, &[]
                )?;

                self.i2c = Some(Box::new(i2c));
            }

            self.set_blink(self.blink_frequency)?;

            self.set_brightness(self.brightness)?;

            self.is_setup = true;
        }

        Ok(())
    }

    /// Blink display at specified frequency
    ///
    /// # Arguments
    ///
    /// * `frequency` - frequency must be a value allowed by the HT16K33, specifically one of: HT16K33_BLINK_OFF, HT16K33_BLINK_2HZ, HT16K33_BLINK_1HZ, or HT16K33_BLINK_HALFHZ.
    pub fn set_blink(&mut self, frequency: u8) -> Result <(), Error> {
        self.blink_frequency = frequency;
        self.i2c_block_write(
            (HT16K33_BLINK_CMD | HT16K33_BLINK_DISPLAYON | frequency) as u8, &[]
        )?;

        Ok(())
    }

    // Set brightness of entire display to specified value.
    // Supports 16 levels, from 0 to 15.
    ///
    /// # Arguments
    ///
    /// * `brightness` - level of brightness, from 0 to 15.
    pub fn set_brightness(&mut self, brightness: u8) -> Result <(), Error> {
        
        assert!(brightness <= 15);

        self.brightness = brightness;

        self.i2c_block_write(
            (HT16K33_CMD_BRIGHTNESS | brightness) as u8, &[]
        )?;

        Ok(())
    }

    /// Write display buffer to display hardware.
    pub fn write_display(&mut self) -> Result <(), Error> {

        if !self.is_setup {
            let _result = self.setup();
        }
        
        let buffer = self.buffer;

        self.i2c_block_write(
            0x00 as u8, &buffer
        )?;

        Ok(())
    }

    /// Clear contents of display buffer.
    pub fn clear(&mut self) {

        for i in 0..self.buffer.len() {
            self.buffer[i] = 0;
        }
    }

    // TODO: set_led
}

/// Errors that can occur.
#[derive(Debug)]
pub enum Error {

    /// I2C error.
    I2c(rppal::i2c::Error),
}

impl error::Error for Error {}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &*self {
            Error::I2c(err) => write!(f, "I2C error: {}", &err),
        }
    }
}

/// Converts I2C error
impl From<rppal::i2c::Error> for Error {
    fn from(err: rppal::i2c::Error) -> Error {
        Error::I2c(err)
    }
}

/// Unit tests
#[cfg(test)]
mod tests {
    use super::*;

    /// Tests the setup of the ht16k33.
    #[test]
    fn test_ht16k33_setup() -> Result<(), Error> {
        let mut ht16k33 = HT16K33::new()?;
        // enable simulation
        ht16k33.simulation = true;

        // Not setup
        assert!(ht16k33.is_setup == false);

        // Force setup
        let _result = ht16k33.setup();

        assert!(ht16k33.is_setup == true);

        // Default values
        assert!(ht16k33.blink_frequency == HT16K33_BLINK_OFF);
        assert!(ht16k33.brightness == 15);

        Ok(())
    }

    /// Tests the setup of the ht16k33.
    #[test]
    fn test_ht16k33_set_blink() -> Result<(), Error> {
        let mut ht16k33 = HT16K33::new()?;
        // enable simulation
        ht16k33.simulation = true;

        // Default values
        assert!(ht16k33.blink_frequency == HT16K33_BLINK_OFF);

        // Change value
        let _result = ht16k33.set_blink(HT16K33_BLINK_2HZ);
        assert!(ht16k33.blink_frequency == HT16K33_BLINK_2HZ);

        Ok(())
    }

    /// Tests the setup of the ht16k33.
    #[test]
    fn test_ht16k33_set_brightness() -> Result<(), Error> {
        let mut ht16k33 = HT16K33::new()?;
        // enable simulation
        ht16k33.simulation = true;

        // Default values
        assert!(ht16k33.brightness == 15);

        // Change value
        let _result = ht16k33.set_brightness(14);
        assert!(ht16k33.brightness == 14);

        Ok(())
    }

}
