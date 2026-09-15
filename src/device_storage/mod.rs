use std::{collections::HashMap, fs};

mod from_file_error;
pub mod rgb_data;

use rgb_data::RgbData;
use serde::{Deserialize, Serialize};

use crate::{device_storage::from_file_error::FromFileError, key_id::KeyId};

#[derive(Serialize, Deserialize)]
pub struct DeviceStorage {
    key_leds: HashMap<KeyId, RgbData>,
    pub color_swatches: Vec<RgbData>,
}

impl DeviceStorage {
    pub fn new() -> Self {
        Self {
            key_leds: HashMap::new(),
            color_swatches: Vec::new(),
        }
    }

    pub fn from_file() -> Result<Self, FromFileError> {
        // todo: de-hardcode
        let json =
            fs::read_to_string("device_storage.json").map_err(|err| FromFileError::IoError(err))?;
        let storage = serde_json::from_str::<Self>(json.as_str())
            .map_err(|err| FromFileError::ParseError(err))?;

        Ok(storage)
    }

    pub fn set_key_led(&mut self, key_id: KeyId, r: u8, g: u8, b: u8) {
        self.key_leds.insert(key_id, RgbData::new(r, g, b));
    }

    pub fn get_key_led(&self, key_id: KeyId) -> Option<&RgbData> {
        self.key_leds.get(&key_id)
    }

    pub fn add_color_swatch(&mut self, color: RgbData) {
        self.color_swatches.push(color);
    }

    // todo: instead of a string, make it an enum like 'FromFileError'
    pub fn flush(&self) -> Result<(), String> {
        let json = serde_json::to_string_pretty(self).map_err(|err| err.to_string())?;
        fs::write("device_storage.json", json).map_err(|err| err.to_string())
    }
}
