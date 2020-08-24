use std::error::Error;
use std::thread;
use std::time::Duration;
use rainbow_hat_rs::buzzer::Buzzer;

const NOTES: [u32; 36] = [
    71, 71, 71, 71, 71, 71, 71, 64, 67, 71,
    69, 69, 69, 69, 69, 69, 69, 62, 66, 69,
    71, 71, 71, 71, 71, 71, 71, 73, 74, 77,
    74, 71, 69, 66, 64, 64
];

const TIMES: [u32; 36] = [
    300, 50, 50, 300, 50, 50, 300, 300, 300, 200,
    300, 50, 50, 300, 50, 50, 300, 300, 300, 200,
    300, 50, 50, 300, 50, 50, 300, 300, 300, 200,
    300, 300, 300, 300, 600, 600
];

/// Play a melody with the buzzer.
fn main() -> Result<(), Box<dyn Error>> {
    
    let mut buzzer = Buzzer::new()?;

    for i in 0..36 {
        buzzer.midi_note(NOTES[i], TIMES[i] as f64 / 1000.0)?;
        thread::sleep(Duration::from_millis(50));
    }

    Ok(())
}
