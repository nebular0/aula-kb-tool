mod device_interface;
mod device_storage;
mod device_template;
mod key_data;
mod key_id;
mod key_id_data;
mod ui;

use ui::UiShell;

fn main() {
    let mut ui = UiShell::new().unwrap();
    ui.bootstrap().unwrap();
}
