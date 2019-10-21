extern crate texter;
use std::{thread, time};

fn main() {
    //Setup WiringPi with its own pin numbering order
    let pi: wiringpi::WiringPi<wiringpi::pin::WiringPi> = wiringpi::setup();

    let mut k = texter::cyclo_keyboard::KeyboardState::new(&pi);

    let interval = time::Duration::from_millis(10);

    loop {
        k.update();
        thread::sleep(interval);
    }
}
