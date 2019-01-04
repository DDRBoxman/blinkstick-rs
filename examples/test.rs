extern crate blinkstick;

use std::{thread, time};

use blinkstick::BlinkStickDevice;

fn main() {
    let ten_millis = time::Duration::from_millis(1000);

    match BlinkStickDevice::get_serials() {
        Ok(serials) => {
            for serial in serials {
                println!("{}", serial)
            }
        }
        Err(e) => println!("{}", e)
    }

    let device = BlinkStickDevice::open_first().expect("Failed to open device.");
    device
        .set_color(0, 1, 255, 0, 0)
        .expect("Failed to set color.");

    thread::sleep(ten_millis);

    device
        .off(0, 1)
        .expect("Failed to set color.");
}