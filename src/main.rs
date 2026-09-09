mod device_storage;
mod device_template;
mod key_data;
mod key_id;
mod key_id_data;
mod ui;

use ui::UiShell;

use crate::device_storage::DeviceStorage;

fn main() {
    let mut d = DeviceStorage::from_file();
    d.set_key_led(10, 0, 0, 255);
    d.set_key_led(65, 0, 0, 255);
    d.flush().unwrap();

    let mut ui = UiShell::new();
    ui.bootstrap().unwrap();
}
