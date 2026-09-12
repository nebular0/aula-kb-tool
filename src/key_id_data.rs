use crate::{key_data::KeyData, key_id::KeyId};

#[derive(Clone)]
pub struct KeyIdData {
    pub id: KeyId,

    pub name: String,
    pub width: Option<u32>,

    // LED's
    pub led_idx: usize,
}

impl KeyIdData {
    pub fn from_key_data(id: KeyId, key_data: &KeyData) -> Self {
        Self {
            id,
            name: key_data.name.clone(),
            width: key_data.width,
            led_idx: key_data.led_idx,
        }
    }
}
