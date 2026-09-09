use std::{collections::HashMap, fs};

mod rgb_data;
use rgb_data::RgbData;
use serde::{Deserialize, Serialize};

use crate::key_id::KeyId;

#[derive(Serialize, Deserialize)]
pub struct DeviceStorage {
    key_leds: HashMap<KeyId, RgbData>,
}

impl DeviceStorage {
    pub fn new() -> Self {
        Self {
            key_leds: HashMap::new(),
        }
    }

    pub fn from_file() -> Self {
        // todo: fix unwrap
        let json = fs::read_to_string("device_storage.json").unwrap();
        let storage = serde_json::from_str::<Self>(json.as_str()).unwrap();
        storage
    }

    pub fn set_key_led(&mut self, key_id: KeyId, r: u8, g: u8, b: u8) {
        self.key_leds.insert(key_id, RgbData::new(r, g, b));
    }

    pub fn flush(&self) -> Result<(), ()> {
        let json = serde_json::to_string_pretty(self).map_err(|_| ())?;
        fs::write("device_storage.json", json).map_err(|_| ())
    }
}
