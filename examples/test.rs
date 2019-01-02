extern crate blinkstick;

use blinkstick::BlinkStickDevice;

fn main() {
    let device = BlinkStickDevice::find_first().expect("Failed to open device.");
    device
        .set_color(0, 1, 255, 0, 0)
        .expect("Failed to set color.")
}