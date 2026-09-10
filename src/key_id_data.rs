use crate::{key_data::KeyData, key_id::KeyId};

#[derive(Clone)]
pub struct KeyIdData {
    pub id: KeyId,

    pub name: String,
    pub width: u32,

    // LED's
    pub led_idx_r: u8,
    pub led_idx_g: u8,
    pub led_idx_b: u8,
}

impl KeyIdData {
    pub fn from_key_data(id: KeyId, key_data: &KeyData) -> Self {
        Self {
            id,
            name: key_data.name.clone(),
            width: key_data.width,
            led_idx_r: key_data.led_idx_r,
            led_idx_g: key_data.led_idx_g,
            led_idx_b: key_data.led_idx_b,
        }
    }
}
