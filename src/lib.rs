extern crate hidapi;

use hidapi::HidApi;
use hidapi::HidError;
use hidapi::HidResult;

const BLINKSTICK_VENDOR_ID: u16 = 0x20A0;
const BLINKSTICK_PRODUCT_ID: u16 = 0x41E5;

const BLINKSTICK_INDEXED_LED_MSG_PACKET_SIZE: usize = 6;

pub struct BlinkStickDevice {
    device: hidapi::HidDevice,
}

impl BlinkStickDevice {
    pub fn find_first() -> Result<BlinkStickDevice, HidError> {
        match HidApi::new() {
            Ok(api) => {
                // Connect to device using its VID and PID
                match api.open(BLINKSTICK_VENDOR_ID, BLINKSTICK_PRODUCT_ID) {
                    Ok(device) => Ok(BlinkStickDevice { device: device }),
                    Err(e) => return Err(e),
                }
            }
            Err(e) => return Err(e),
        }
    }

    pub fn off(&self, channel: u8, index: u8) -> HidResult<()> {
        return self.set_color(channel, index, 0, 0, 0);
    }

    pub fn set_color(&self, channel: u8, index: u8, r: u8, g: u8, b: u8) -> HidResult<()> {
        let mut buf: [u8; BLINKSTICK_INDEXED_LED_MSG_PACKET_SIZE] =
            [0; BLINKSTICK_INDEXED_LED_MSG_PACKET_SIZE];

        buf[0] = 0x05;
        buf[1] = channel;
        buf[2] = index;
        buf[3] = r;
        buf[4] = g;
        buf[5] = b;

        return self.device.send_feature_report(&buf);
    }
}
