use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct RgbData {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl RgbData {
    pub fn new(r: u8, g: u8, b: u8) -> Self {
        Self { r, g, b }
    }
}
