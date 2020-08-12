
use std::collections::HashMap;
use std::fmt;
use crate::ht16k33::HT16K33;

/// Digit value to bitmask mapping.
const DIGIT_VALUES: [(char, u16); 95] = 
    [
        (' ', 0b0000000000000000),
        ('!', 0b0000000000000110),
        ('"', 0b0000001000100000),
        ('#', 0b0001001011001110),
        ('$', 0b0001001011101101),
        ('%', 0b0000110000100100),
        ('&', 0b0010001101011101),
        ('\'', 0b0000010000000000),
        ('(', 0b0010010000000000),
        (')', 0b0000100100000000),
        ('*', 0b0011111111000000),
        ('+', 0b0001001011000000),
        (',', 0b0000100000000000),
        ('-', 0b0000000011000000),
        ('.', 0b0000000000000000),
        ('/', 0b0000110000000000),
        ('0', 0b0000110000111111),
        ('1', 0b0000000000000110),
        ('2', 0b0000000011011011),
        ('3', 0b0000000010001111),
        ('4', 0b0000000011100110),
        ('5', 0b0010000001101001),
        ('6', 0b0000000011111101),
        ('7', 0b0000000000000111),
        ('8', 0b0000000011111111),
        ('9', 0b0000000011101111),
        (':', 0b0001001000000000),
        (';', 0b0000101000000000),
        ('<', 0b0010010000000000),
        ('=', 0b0000000011001000),
        ('>', 0b0000100100000000),
        ('?', 0b0001000010000011),
        ('@', 0b0000001010111011),
        ('A', 0b0000000011110111),
        ('B', 0b0001001010001111),
        ('C', 0b0000000000111001),
        ('D', 0b0001001000001111),
        ('E', 0b0000000011111001),
        ('F', 0b0000000001110001),
        ('G', 0b0000000010111101),
        ('H', 0b0000000011110110),
        ('I', 0b0001001000000000),
        ('J', 0b0000000000011110),
        ('K', 0b0010010001110000),
        ('L', 0b0000000000111000),
        ('M', 0b0000010100110110),
        ('N', 0b0010000100110110),
        ('O', 0b0000000000111111),
        ('P', 0b0000000011110011),
        ('Q', 0b0010000000111111),
        ('R', 0b0010000011110011),
        ('S', 0b0000000011101101),
        ('T', 0b0001001000000001),
        ('U', 0b0000000000111110),
        ('V', 0b0000110000110000),
        ('W', 0b0010100000110110),
        ('X', 0b0010110100000000),
        ('Y', 0b0001010100000000),
        ('Z', 0b0000110000001001),
        ('[', 0b0000000000111001),
        ('\\', 0b0010000100000000),
        (']', 0b0000000000001111),
        ('^', 0b0000110000000011),
        ('_', 0b0000000000001000),
        ('`', 0b0000000100000000),
        ('a', 0b0001000001011000),
        ('b', 0b0010000001111000),
        ('c', 0b0000000011011000),
        ('d', 0b0000100010001110),
        ('e', 0b0000100001011000),
        ('f', 0b0000000001110001),
        ('g', 0b0000010010001110),
        ('h', 0b0001000001110000),
        ('i', 0b0001000000000000),
        ('j', 0b0000000000001110),
        ('k', 0b0011011000000000),
        ('l', 0b0000000000110000),
        ('m', 0b0001000011010100),
        ('n', 0b0001000001010000),
        ('o', 0b0000000011011100),
        ('p', 0b0000000101110000),
        ('q', 0b0000010010000110),
        ('r', 0b0000000001010000),
        ('s', 0b0010000010001000),
        ('t', 0b0000000001111000),
        ('u', 0b0000000000011100),
        ('v', 0b0010000000000100),
        ('w', 0b0010100000010100),
        ('x', 0b0010100011000000),
        ('y', 0b0010000000001100),
        ('z', 0b0000100001001000),
        ('{', 0b0000100101001001),
        ('|', 0b0001001000000000),
        ('}', 0b0010010010001001),
        ('~', 0b0000010100100000)
    ];

/// 4-digit alphanumeric 7-segment display driver.
#[derive(Debug)]
pub struct Alphanum4 {
    
    /// Driver for HT16K33
    pub ht16k33 : HT16K33,

    /// Map of bitmask for each character to print on the display
    digit_value: HashMap<char, u16>,
}

impl Alphanum4 {

    /// Creates the alphanumeric 7-segment display driver screen.
    pub fn new() -> Result<Alphanum4, Error> {
        let ht16k33 = HT16K33::new()?;
        let digit_value = DIGIT_VALUES.iter().cloned().collect();

        Ok(Self {
            ht16k33,
            digit_value,
        })
    }

    /// Set digit at position to raw bitmask value.
    ///
    /// # Arguments
    ///
    /// * `pos` - Position should be a value 0 to 3 with 0 being the left most digit on the display.
    /// * `bitmask` - bitmask value to set.
    pub fn set_digit_raw(&mut self, pos:usize, bitmask: u16) {
        
        // Ignore out of bounds digits.
        if pos <= 3 {
            let digit = self.u16_to_u8(bitmask);
            self.ht16k33.buffer[pos * 2] = digit.0;
            self.ht16k33.buffer[pos * 2 + 1] = digit.1;
        }
    }

    /// Turn decimal point on or off at provided position.
    ///
    /// # Arguments
    ///
    /// * `pos` - Position should be a value 0 to 3 with 0 being the left most digit on the display.
    /// * `decimal` - Decimal should be True to turn on the decimal point and False to turn it off.
    pub fn set_decimal(&mut self, pos : usize, decimal: bool) {

        // Ignore out of bounds digits.
        if pos <= 3 {
            if decimal {
                self.ht16k33.buffer[pos * 2 + 1] |= 1 << 6;
            } else {
                self.ht16k33.buffer[pos * 2 + 1] &= !(1 << 6);
            }
        }
    }

    /// Set digit at position to provided value.
    ///
    /// # Arguments
    ///
    /// * `pos` - Position should be a value of 0 to 3 with 0 being the left most digit on the display.
    /// * `digit` - Digit should be any ASCII value 32-127 (printable ASCII).
    /// * `decimal` - Decimal should be True to turn on the decimal point and False to turn it off.
    pub fn set_digit(&mut self, pos : usize, digit: char, decimal: bool) {
        self.set_digit_raw(pos, *self.digit_value.get(&digit).unwrap());
        self.set_decimal(pos, decimal);
    }

    /// Print a 4 character long string of values to the display.
    ///
    /// # Arguments
    ///
    /// * `value` - String where characters in the string should be any ASCII value 32 to 127 (printable ASCII).
    /// * `justify_right` - Align to the right.
    pub fn print_str(&mut self, value : &str, justify_right: bool) {

        let char_vec: Vec<char> = value.chars().collect();
        let mut pos = 0;

        // Calculcate starting position of digits based on justification.
        if justify_right {
            pos = 4 - value.len();
        }

        for c in char_vec {
            self.set_digit(pos, c, false);
            pos += 1;
        }
    }

    // TODO:
    // print_number_str
    // print_float
    // print_hex

    /// Display buffer on display.
    pub fn show(&mut self) -> Result <(), Error>{
        self.ht16k33.write_display()?;

        Ok(())
    }

    /// Splits a u16 in a tuple of u8.
    ///
    /// # Arguments
    ///
    /// * `num` - u16 number.
    fn u16_to_u8(&mut self, num : u16) -> (u8, u8) {
        (
            (num & 0xFF) as u8,
            ((num >> 8) & 0xFF) as u8
        )
    }
}

/// Errors that can occur.
#[derive(Debug)]
pub enum Error {

    /// HT16K33 error.
    HT16K33(crate::ht16k33::Error),
}

impl std::error::Error for Error {}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &*self {
            Error::HT16K33(err) => write!(f, "HT16K33 error: {}", &err),
        }
    }
}

/// Converts HT16K33 error
impl From<crate::ht16k33::Error> for Error {
    fn from(err: crate::ht16k33::Error) -> Error {
        Error::HT16K33(err)
    }
}
