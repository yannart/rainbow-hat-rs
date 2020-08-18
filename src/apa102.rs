use std::fmt;
use std::thread;
use std::time::Duration;
use core::fmt::Debug;
use rppal::gpio::{Gpio, OutputPin, Level};

/// GPIO BCM pin number for DAT.
pub const GPIO_DAT: u8 = 10;

/// GPIO BCM pin number for CLK.
pub const GPIO_CLK: u8 = 11;

/// GPIO BCM pin number for CS.
pub const GPIO_CS: u8 = 8;

/// Number of pixels.
pub const NUM_PIXELS: usize = 7;

/// Brightness.
pub const BRIGHTNESS: u8 = 7;

/// Sleep time between pin commands.
pub const SLEEP_TIME : u64 = 0;

/// Rainbow HAT APA102 Driver.
#[derive(Debug)]
pub struct APA102 {

    /// Output pin to write to GPIO. Optional as not used in simulated mode.
    pin_dat: Option<Box<OutputPin>>,

    /// Output pin to write to GPIO. Optional as not used in simulated mode.
    pin_clk: Option<Box<OutputPin>>,

    /// Output pin to write to GPIO. Optional as not used in simulated mode.
    pin_cs: Option<Box<OutputPin>>,

    /// pixels to be printed
    pub pixels: [[u8;4] ; NUM_PIXELS],

    /// brightness between 0 and 15
    brightness: u8,

    /// In simulation mode, no interaction with the hardware is done to simplify testability.
    simulation: bool, 

    /// is the setup completed
    is_setup: bool,
}

impl APA102 {
    
    /// Creates a APA102.
    pub fn new() -> Result<APA102, Error>  {     

        Ok(Self {
            pin_dat: None,
            pin_clk: None,
            pin_cs: None,
            pixels:[[0; 4]; NUM_PIXELS],
            brightness: BRIGHTNESS,
            simulation: false,
            is_setup: false,
        })
    }

    /// Initialize driver.
    pub fn setup(&mut self) -> Result <(), Error> {
        if !self.is_setup {

            // Ignore Gpio initialization if in simulation mode
            if !self.simulation {
                let gpio_dat = Gpio::new()?;
                let output_dat = gpio_dat.get(GPIO_DAT)?.into_output(); 
                self.pin_dat = Some(Box::new(output_dat));

                let gpio_clk = Gpio::new()?;
                let output_clk = gpio_clk.get(GPIO_CLK)?.into_output(); 
                self.pin_clk = Some(Box::new(output_clk));

                let gpio_cs = Gpio::new()?;
                let output_cs = gpio_cs.get(GPIO_CS)?.into_output(); 
                self.pin_cs = Some(Box::new(output_cs));
            }

            self.is_setup = true;
        }
        Ok(())
    }

    /// Exit.
    pub fn exit(&mut self) -> Result <(), Error> {
        self.clear();
        self.show()?;

        Ok(())
    }

    /// Set the brightness of all pixels.
    /// # Arguments
    ///
    /// * `brightness` - Brightness: 0.0 to 1.0.
    pub fn set_brightness(&mut self, brightness : f32) {

        assert!(brightness >= 0.0);
        assert!(brightness <= 1.0);

        for i in 0..self.pixels.len() {
            self.pixels[i][3] = (31.0 * brightness.round()) as u8;
        }
    }

    /// Clear the pixel buffer.
    pub fn clear(&mut self) {
        for i in 0..self.pixels.len() {
            self.pixels[i][0] = 0 as u8; // R
            self.pixels[i][1] = 0 as u8; // G
            self.pixels[i][2] = 0 as u8; // B
        }
    }

    /// Write a single byte to the DAT and CLK pins.
    /// # Arguments
    ///
    /// * `byte` - Bite to write.
    fn write_byte (&mut self, byte : u8) {

        if !self.simulation {
            let output_dat = self.pin_dat.as_deref_mut().unwrap();
            let output_clk = self.pin_clk.as_deref_mut().unwrap();

            // Scan from most significative to least
            for i in 0..8 {
                if APA102::get_bit_at(byte, 7 - i) {
                    output_dat.write(Level::High);
                } else {
                    output_dat.write(Level::Low);
                }
                output_clk.write(Level::High);
                thread::sleep(Duration::from_millis(SLEEP_TIME));
                output_clk.write(Level::Low);
                thread::sleep(Duration::from_millis(SLEEP_TIME));
            }
        }
    }

    /// Ends writing data.
    fn eof(&mut self) {

            if !self.simulation {
            let output_dat = self.pin_dat.as_deref_mut().unwrap();
            let output_clk = self.pin_clk.as_deref_mut().unwrap();

            output_dat.write(Level::Low);

            for _x in 0..36 {
                output_clk.write(Level::High);
                thread::sleep(Duration::from_millis(SLEEP_TIME));
                output_clk.write(Level::Low);
                thread::sleep(Duration::from_millis(SLEEP_TIME));
            }
        }
    }

    /// Starts writing data.
    fn sof(&mut self) {

        if !self.simulation {
            let output_dat = self.pin_dat.as_deref_mut().unwrap();
            let output_clk = self.pin_clk.as_deref_mut().unwrap();

            output_dat.write(Level::Low);

            for _x in 0..32 {
                output_clk.write(Level::High);
                thread::sleep(Duration::from_millis(SLEEP_TIME));
                output_clk.write(Level::Low);
                thread::sleep(Duration::from_millis(SLEEP_TIME));
            }
        }
    }

    /// Output the buffer.
    pub fn show(&mut self) -> Result <(), Error>{

        // Initialize if not done yet
        if !self.is_setup {
            let _result = self.setup();
        }

        if !self.simulation {
            let output_cs = self.pin_cs.as_deref_mut().unwrap();
            output_cs.write(Level::Low);

            self.sof();

            for i in 0..self.pixels.len() {
                self.write_byte(0b11100000 | self.pixels[i][3]); // brightness
                self.write_byte(self.pixels[i][2]); // b
                self.write_byte(self.pixels[i][1]); // g
                self.write_byte(self.pixels[i][0]); // r
            }

            self.eof();

            let output_cs = self.pin_cs.as_deref_mut().unwrap();
            output_cs.write(Level::High);
        }

        Ok(())
    }

    /// Set the RGB value and optionally brightness of all pixels.
    /// # Arguments
    ///
    /// * `r` - Amount of red: 0 to 255
    /// * `g` - Amount of green: 0 to 255
    /// * `b` - Amount of blue: 0 to 255
    /// * `brightness` - Brightness: 0.0 to 1.0
    pub fn set_all(&mut self, r : u8, g: u8, b: u8, brightness: f32) {
        for i in 0..self.pixels.len() {
            self.set_pixel(i, r, g, b, brightness);
        }
    }

    /// Set the RGB value, and optionally brightness, of a single pixel.
    /// # Arguments
    ///
    /// * `x` - The horizontal position of the pixel: 0 to 7
    /// * `r` - Amount of red: 0 to 255
    /// * `g` - Amount of green: 0 to 255
    /// * `b` - Amount of blue: 0 to 255
    /// * `brightness` - Brightness: 0.0 to 1.0
    pub fn set_pixel(&mut self, x: usize, r : u8, g: u8, b: u8, brightness: f32) {
        assert!(brightness >= 0.0);
        assert!(brightness <= 1.0);
        
        self.pixels[x][0] = r as u8; // R
        self.pixels[x][1] = g as u8; // G
        self.pixels[x][2] = b as u8; // B
        self.pixels[x][3] = (31.0 * brightness.round()) as u8; // Brightness
    }

    /// gets the bit at position `n`. Bits are numbered from 0 (least significant) to 31 (most significant).
    /// # Arguments
    ///
    /// * `byte` - The byte to get the bit from.
    /// * `n` - Bit position.
    fn get_bit_at(byte: u8, n: u8) -> bool {
        assert!(n < 8);

        byte & (1 << n) != 0
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
    fn test_apa102_setup() -> Result<(), Error> {
        
        let mut apa102 = APA102::new()?;
        apa102.simulation = true;
        
        // Not setup
        assert!(apa102.is_setup == false);

        // Force setup
        let _result = apa102.setup();

        assert!(apa102.is_setup == true);

        Ok(())
    }

    /// Tests the setup of the light.
    #[test]
    fn test_apa102_set_brightness() -> Result<(), Error> {
        
        let mut apa102 = APA102::new()?;
        apa102.simulation = true;
        let _result = apa102.setup();

        apa102.set_brightness(0.0);
        for i in 0..apa102.pixels.len() {
            assert!(apa102.pixels[i][3] == 0);
        }

        apa102.set_brightness(1.0);
        for i in 0..apa102.pixels.len() {
            assert!(apa102.pixels[i][3] == 31);
        }

        Ok(())
    }

    /// Test clearing the buffer.
    #[test]
    fn test_apa102_clear() -> Result<(), Error> {
        
        let mut apa102 = APA102::new()?;
        apa102.simulation = true;
        let _result = apa102.setup();

        let brightness : u8 = 31;

        for i in 0..apa102.pixels.len() {
            apa102.pixels[i][0] = 250 as u8; // R
            apa102.pixels[i][1] = 250 as u8; // G
            apa102.pixels[i][2] = 250 as u8; // B
            apa102.pixels[i][3] = brightness;
        }

        apa102.clear();

        // The RGB pixels are clear but the brightness is unchanged
        for i in 0..apa102.pixels.len() {
            assert!(apa102.pixels[i][0] == 0);
            assert!(apa102.pixels[i][1] == 0);
            assert!(apa102.pixels[i][2] == 0);
            assert!(apa102.pixels[i][3] == 31);
        }

        Ok(())
    }

    /// Tests to set pixel colors.
    #[test]
    fn test_apa102_set_pixel() -> Result<(), Error> {
        
        let mut apa102 = APA102::new()?;
        apa102.simulation = true;
        let _result = apa102.setup();

        apa102.set_pixel(0, 123, 234, 012, 1.0);
        apa102.set_pixel(6, 12, 58, 123, 0.0);

        assert!(apa102.pixels[0][0] == 123);
        assert!(apa102.pixels[0][1] == 234);
        assert!(apa102.pixels[0][2] == 12);
        assert!(apa102.pixels[0][3] == 31);

        assert!(apa102.pixels[6][0] == 12);
        assert!(apa102.pixels[6][1] == 58);
        assert!(apa102.pixels[6][2] == 123);
        assert!(apa102.pixels[6][3] == 0);
        Ok(())
    }

    /// Tests to set all 
    #[test]
    fn test_apa102_set_all() -> Result<(), Error> {
        
        let mut apa102 = APA102::new()?;
        apa102.simulation = true;
        let _result = apa102.setup();

        apa102.set_all(123, 234, 012, 1.0);

        for i in 0..apa102.pixels.len() {
            assert!(apa102.pixels[i][0] == 123);
            assert!(apa102.pixels[i][1] == 234);
            assert!(apa102.pixels[i][2] == 12);
            assert!(apa102.pixels[i][3] == 31);
        }

        Ok(())
    }

    /// Tests obtaining a bit from a byte.
    #[test]
    fn test_apa102_get_bit_at() -> Result<(), Error> {
        
        let value = 0b00010101 as u8;

        assert!(APA102::get_bit_at(value, 0) == true);
        assert!(APA102::get_bit_at(value, 1) == false);
        assert!(APA102::get_bit_at(value, 2) == true);
        assert!(APA102::get_bit_at(value, 3) == false);
        assert!(APA102::get_bit_at(value, 4) == true);
        assert!(APA102::get_bit_at(value, 5) == false);
        assert!(APA102::get_bit_at(value, 6) == false);
        assert!(APA102::get_bit_at(value, 7) == false);

        Ok(())
    }
}
