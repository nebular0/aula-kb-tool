mod aula_id;
pub mod device_info;

use crate::{device_storage::DeviceStorage, device_template::DeviceTemplate};
use aula_id::AulaId;
use device_info::DeviceInfo;

use hidapi::{HidApi, HidDevice, HidError};
use std::{ffi::CStr, fs};

pub struct DeviceInterface {
    device: Option<HidDevice>,
    hid_api: HidApi,
    aula_ids: Vec<AulaId>,
}

const AULA_IDS_FILENAME: &str = "aula_ids.json";

impl DeviceInterface {
    pub fn new() -> Self {
        let aula_ids_content =
            fs::read_to_string(AULA_IDS_FILENAME).expect("couldn't open device identifiers file");
        let aula_ids = serde_json::from_str::<Vec<AulaId>>(&aula_ids_content)
            .expect("couldn't parse device identifiers file");

        Self {
            device: None,
            hid_api: HidApi::new().unwrap(),
            aula_ids,
        }
    }

    pub fn open(&mut self, path: String) -> Result<(), HidError> {
        self.hid_api
            .open_path(unsafe { CStr::from_ptr(path.as_ptr() as *const i8) })
            .map(|device| {
                self.device = Some(device);
                ()
            })
    }

    pub fn list_devices(&self) -> impl Iterator<Item = DeviceInfo> {
        self.hid_api
            .device_list()
            .map(|d| DeviceInfo::from(d))
            .filter(|d| {
                self.aula_ids
                    .iter()
                    .any(|id| (d.product_id, d.vendor_id) == (id.product_id, id.vendor_id))
            })
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

        let report_start = device_template.report_start;
        let report_offset = device_template.report_offset;

        let mut report: [u8; 512] = [0; 512];
        report[..report_start].copy_from_slice(&[0x06, 0x06, 0x00, 0x00, 0x01, 0x00, 0x80, 0x01]);

        for key in device_template.keys.iter().flatten() {
            if let Some(key_led) = device_storage.get_key_led(key.id) {
                report[report_start + key.led_idx] = key_led.r;
                report[report_start + key.led_idx + report_offset] = key_led.g;
                report[report_start + key.led_idx + report_offset + report_offset] = key_led.b;
            }
        }

        device.send_feature_report(&report)
    }
}
