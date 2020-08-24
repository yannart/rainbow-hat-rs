use std::fmt;
use std::thread;
use std::time::Duration;
use core::fmt::Debug;
use rppal::gpio::{Gpio, OutputPin};

/// GPIO BCM pin number for buzzer.
pub const GPIO_BUZZER: u8 = 13;

/// Buzzer on the board.
#[derive(Debug)]
pub struct Buzzer {

    /// Output pin to write to GPIO. Optional as not used in simulated mode.
    pin: Option<Box<OutputPin>>,

    /// In simulation mode, no interaction with the hardware is done to simplify testability.
    simulation: bool, 

    /// is the setup completed
    is_setup: bool,
}

impl Buzzer {
    
    /// Creates a Buzzer.
    pub fn new() -> Result<Buzzer, Error>  {     

        Ok(Self {
            pin: None,
            simulation: false,
            is_setup: false,
        })
    }

    /// Setup piezo buzzer.
    pub fn setup(&mut self) -> Result <(), Error> {
        if !self.is_setup {

            // Ignore Gpio initialization if in sumulation mode
            if !self.simulation {
                let gpio = Gpio::new()?;
                let output = gpio.get(GPIO_BUZZER)?.into_output(); 
                self.pin = Some(Box::new(output));
            }

            self.is_setup = true;
        }
        Ok(())
    }

    /// Play a single note.
    ///
    /// # Arguments
    ///
    /// * `frequency` - Musical frequency in hertz.
    /// * `duration` - Duration in seconds.
    pub fn note(&mut self, frequency : f64, duration: f64) -> Result<(), Error>{
        
        assert!(frequency > 0.0);

        if !self.is_setup {
            let _result = self.setup();
        }

        // Only perform actual pin write if not in simulation mode
        if !self.simulation {

            let pin = self.pin.as_deref_mut().unwrap();

            pin.set_pwm_frequency(frequency, 0.90)?;

            thread::sleep(Duration::from_millis((duration * 1000.0) as u64));

            pin.clear_pwm()?;
        }

        Ok(())
    }

    /// Play a single note by MIDI note number.
    /// Converts a MIDI note number into a frequency and plays it. A5 is 69.
    ///
    /// # Arguments
    ///
    /// * `note_number` - MIDI note number of note.
    /// * `duration` - Duration in seconds.
    pub fn midi_note(&mut self, note_number : u32, duration: f64) -> Result <(), Error>{
        
        assert!(note_number > 0);

        let freq = Buzzer::midi_note_to_frequency(note_number);
        self.note(freq, duration)?;

        Ok(())
    }

    /// Get the frequency in Hz from the midi note.
    ///
    /// # Arguments
    ///
    /// * `note_number` - Midi note number.
    fn midi_note_to_frequency(note_number : u32) -> f64 {
        assert!(note_number > 0);

        let base: f64 = 2.0;
        base.powf((note_number as f64 - 69.0) / 12.0) * 440.0
    }

    /// Stop buzzer.
    /// Immediately silences the buzzer.
    pub fn stop(&mut self) -> Result <(), Error>{

        if !self.is_setup {
            let _result = self.setup();
        }

        // Only perform actual pin write if not in simulation mode
        if !self.simulation {

            let pin = self.pin.as_deref_mut().unwrap();
            pin.clear_pwm()?;
        }

        Ok(())
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

    /// Tests the setup of the buzzer.
    #[test]
    fn test_buzzer_setup() -> Result<(), Error> {
        let mut buzzer = Buzzer::new()?;
        // enable simulation
        buzzer.simulation = true;

        // Not setup
        assert!(buzzer.is_setup == false);

        // Force setup
        let _result = buzzer.setup();

        assert!(buzzer.is_setup == true);

        Ok(())
    }

    /// Tests the conversion from mido to frequency.
    #[test]
    fn test_buzzer_midi_note_to_frequency() -> Result<(), Error> {

        // Test some examples of notes and frequency equivalent.
        let tests: [(u32, f64); 5] = [(11, 15.434),(21, 27.5),(40, 82.407),(57, 220.0),(112, 5274.0)];

        for i in 0..tests.len() {
            let (note, expected_freq) = tests[i];
            let computed_frq = Buzzer::midi_note_to_frequency(note);

            // Compare computed frequency with expected with 0.1 tolerance.
            assert!((expected_freq >= computed_frq - 0.1) && (expected_freq <= computed_frq + 0.1));
            //println!("note: {}, expected freq: {}, freq: {} Hz", note, expected_freq, computed_frq);
        }
    
        Ok(())
    }

    /// Tests note.
    #[test]
    fn test_buzzer_note() -> Result<(), Error> {
        let mut buzzer = Buzzer::new()?;
        // enable simulation
        buzzer.simulation = true;

        buzzer.note(493.0, 0.5)?;

        assert!(buzzer.is_setup == true);

        Ok(())
    }

    /// Tests invalid note.
    #[test]
    #[should_panic]
    fn test_buzzer_note_invalid() {
        let mut buzzer = Buzzer::new().unwrap();
        // enable simulation
        buzzer.simulation = true;

        let _result = buzzer.note(-1.0, 0.5);
    }

    /// Tests midi note.
    #[test]
    fn test_buzzer_midi_note() -> Result<(), Error> {
        let mut buzzer = Buzzer::new()?;
        // enable simulation
        buzzer.simulation = true;

        buzzer.midi_note(71, 0.5)?;

        assert!(buzzer.is_setup == true);

        Ok(())
    }

    /// Tests invalid midi note.
    #[test]
    #[should_panic]
    fn test_buzzer_midi_note_invalid() {
        let mut buzzer = Buzzer::new().unwrap();
        // enable simulation
        buzzer.simulation = true;

        let _result = buzzer.midi_note(0, 0.5);
    }
}
