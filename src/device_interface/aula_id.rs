use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, PartialEq)]
pub struct AulaId {
    pub name: String,
    pub product_id: u16,
    pub vendor_id: u16,
}
