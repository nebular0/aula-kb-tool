use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct KeyData {
    pub name: String,
    pub row: usize,
    pub column: usize,
    pub width: Option<u32>,

    // LED's
    pub led_idx: usize,
}
