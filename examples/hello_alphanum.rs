use std::error::Error;
use std::thread;
use std::time::Duration;
use rainbow_hat_rs::alphanum4::Alphanum4;

fn main() -> Result<(), Box<dyn Error>> {

    let sleep_time = 500;
    let msg = "HELLO WORLD  ";

    let mut alphanum = Alphanum4::new()?;

    let mut start_index = 0;
    let mut msg2 = msg.to_string();
    msg2.push_str(&msg);

    loop {

        let substring = &msg2[start_index..=(start_index + 3)];
        alphanum.print_str(substring, false);
        alphanum.show()?;
        thread::sleep(Duration::from_millis(sleep_time));

        start_index += 1;

        if start_index == msg.len() {
            start_index = 0;
        }
    }
}
