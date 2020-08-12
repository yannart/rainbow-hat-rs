use std::error::Error;
use std::thread;
use std::time::Duration;
use rainbow_hat_rs::lights::Lights;
use rainbow_hat_rs::touch::Buttons;

fn main() -> Result<(), Box<dyn Error>> {
    
    let mut lights = Lights::new()?;
    let mut buttons = Buttons::new()?;

    // Turn on the light when a touch is pressed.
    loop {
        
        thread::sleep(Duration::from_millis(50));

        if buttons.a.is_pressed() {
            println!("Button A touched!");
            lights.rgb(true, false, false);
        } else if buttons.b.is_pressed() {
            println!("Button B touched!");
            lights.rgb(false, true, false);
        } else if buttons.c.is_pressed(){
            println!("Button C touched!");
            lights.rgb(false, false, true);
        } else {
            println!("Button release!");
            lights.rgb(false, false, false);
        }
     }
}
