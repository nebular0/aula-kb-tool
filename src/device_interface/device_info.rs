use std::fmt::Display;

#[derive(PartialEq, Eq, Clone)]
pub struct DeviceInfo {
    pub product_id: u16,
    pub vendor_id: u16,
    pub interface: i32,
    pub path: String,
}

impl From<&hidapi::DeviceInfo> for DeviceInfo {
    fn from(value: &hidapi::DeviceInfo) -> Self {
        Self {
            product_id: value.product_id(),
            vendor_id: value.vendor_id(),
            interface: value.interface_number(),
            path: value.path().to_str().unwrap().to_string(),
        }
    }
}

impl Display for DeviceInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{},{} (interface {}, path {})",
            self.product_id, self.vendor_id, self.interface, self.path
        )
    }
}
