use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct KeyData {
    pub name: String,
    pub row: usize,
    pub column: usize,
    pub width: u32,

    // LED's
    pub led_idx_r: u8,
    pub led_idx_g: u8,
    pub led_idx_b: u8,
}
