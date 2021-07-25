use std::error::Error;
use std::thread;
use std::time::{Duration,SystemTime};
use rainbow_hat_rs::apa102::APA102;
use palette::{Hsv, Srgb, FromColor};

/// Displays changing colors on the rainbow lights.
// The crate `palette` is used to convert from HSV to RGB colors.
fn main() -> Result<(), Box<dyn Error>> {

    loop {
        let mut apa102 = APA102::new()?;

        // For each rainbow light
        for x in 0..7 {

            // Current time in millis
            let time_since_epoch = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_millis();
            
            // Computes a hue for the light
            let delta = time_since_epoch as f64 * 20.0 / 1000.0;
            let hue = ((delta + x as f64 * 10.0) % 360.0) as f32;

            // Obtains RGB from hue
            let hsv = Hsv::new(hue, 1.0, 1.0);
            let rgb: Srgb = Srgb::from_color(hsv);
            let r = (rgb.red * 255.0).round() as u8;
            let g = (rgb.green * 255.0).round() as u8;
            let b = (rgb.blue * 255.0).round() as u8;

            // Sets pixel color.
            apa102.set_pixel((6 - x) as usize, r, g, b, 0.5);
        }

        // Shows on the device.
        apa102.show()?;
        thread::sleep(Duration::from_millis(5));
    }

}
