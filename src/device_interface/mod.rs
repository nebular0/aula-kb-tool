use crate::{device_storage::DeviceStorage, device_template::DeviceTemplate};
use hidapi::{DeviceInfo, HidApi, HidDevice, HidError};

pub struct DeviceInterface {
    device: Option<HidDevice>,
    hid_api: HidApi,
}

impl DeviceInterface {
    pub fn new() -> Self {
        Self {
            device: None,
            hid_api: HidApi::new().unwrap(),
        }
    }

    pub fn open(&mut self, vendor_id: u16, product_id: u16) -> Result<(), HidError> {
        // todo: de-hardcode
        self.hid_api.open_path(c"/dev/hidraw2").map(|device| {
            self.device = Some(device);
            ()
        })
    }

    pub fn list_devices(&self) -> impl Iterator<Item = &DeviceInfo> {
        self.hid_api.device_list()
    }

    pub fn save_key_leds(
        &mut self,
        device_storage: &DeviceStorage,
        device_template: &DeviceTemplate,
    ) -> Result<(), HidError> {
        let device = if let Some(device) = &self.device {
            device
        } else {
            unreachable!()
        };

        let mut report: [u8; 512] = [0; 512];
        report[..8].copy_from_slice(&[0x06, 0x06, 0x00, 0x00, 0x01, 0x00, 0x80, 0x01]);

        for key in device_template.keys.iter().flatten() {
            if let Some(key_led) = device_storage.get_key_led(key.id) {
                report[START + key.led_idx] = key_led.r;
                report[START + key.led_idx + OFFSET] = key_led.g;
                report[START + key.led_idx + OFFSET + OFFSET] = key_led.b;
            }
        }

        device.send_feature_report(&report)
    }
}

const START: usize = 8;
const OFFSET: usize = 126;
