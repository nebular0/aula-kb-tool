use hex_color::HexColor;
use iced::{
    Background, Border, Color, Element, Font, Length,
    widget::{
        Column, Row, Stack, button, center, column, container, mouse_area, pick_list, row, text,
        text_input,
    },
};

mod message;
use message::Message;

use crate::{
    device_interface::{DeviceInterface, device_info::DeviceInfo},
    device_storage::{DeviceStorage, rgb_data::RgbData},
    device_template::DeviceTemplate,
    key_id_data::KeyIdData,
};

pub struct UiShell {
    error_message: Option<String>,

    device_storage: DeviceStorage,
    device_template: DeviceTemplate,
    device_interface: DeviceInterface,
    opened_device: Option<DeviceInfo>,

    color_swatches: Vec<Color>,
    selected_color: Color,
    swatch_color_rgb_hex: String,

    key_modal: Option<KeyIdData>,
    key_led_rgb_hex: String,
}

const CONTRAST_THRESHOLD: f64 = 0.2;

fn key_button<'a>(key: &'a KeyIdData, device_storage: &'a DeviceStorage) -> Element<'a, Message> {
    let maybe_key_led = device_storage.get_key_led(key.id);

    let button = button(center(text(&key.name).size(14)))
        .style(move |_, _| button::Style {
            background: if let Some(key_led) = maybe_key_led {
                Some(Background::Color(Color::from_rgb8(
                    key_led.r, key_led.g, key_led.b,
                )))
            } else {
                Some(Background::Color(Color::BLACK))
            },
            text_color: if maybe_key_led.is_some_and(|key_led| {
                relative_luminance(key_led.r, key_led.g, key_led.b) > CONTRAST_THRESHOLD
            }) {
                Color::BLACK
            } else {
                Color::WHITE
            },
            border: Border::default().rounded(4).width(0),
            ..Default::default()
        })
        .height(32)
        .width(if let Some(width) = key.width {
            width.into()
        } else {
            Length::Shrink
        });

    mouse_area(button)
        .on_press(Message::OnPressFillColor(key.id))
        .on_right_press(Message::OnPressOpenKeyModal(key.clone()))
        .into()
}

impl Default for UiShell {
    fn default() -> Self {
        Self::new().expect("couldn't initialize UI shell")
    }
}

fn relative_luminance(r: u8, g: u8, b: u8) -> f64 {
    // https://en.wikipedia.org/wiki/Relative_luminance
    0.2126 * (r as f64 / 255.0) + 0.7152 * (g as f64 / 255.0) + 0.0722 * (b as f64 / 255.0)
}

impl UiShell {
    pub fn new() -> Result<Self, String> {
        let device_storage = DeviceStorage::from_file().map_err(|err| err.to_string())?;
        let mut color_swatches = vec![Color::BLACK];

        color_swatches.extend(
            device_storage
                .color_swatches
                .iter()
                .map(|color| Color::from_rgb8(color.r, color.g, color.b)),
        );

        Ok(Self {
            error_message: None,
            device_storage,
            device_template: DeviceTemplate::new(),
            device_interface: DeviceInterface::new(),
            opened_device: None,
            key_modal: None,
            key_led_rgb_hex: String::new(),
            color_swatches,
            selected_color: Color::BLACK,
            swatch_color_rgb_hex: String::new(),
        })
    }

    fn key_modal<'a>(key: &'a KeyIdData, led_rgb_hex: &'a str) -> Element<'a, Message> {
        let content: Element<_> = column![
            row![text("Selected key:"), text(&key.name).font(Font::MONOSPACE)].spacing(12),
            row![
                text("LED Color (hex):"),
                text_input("#000000", led_rgb_hex).on_input(Message::OnInputKeyLedRgbHex),
            ]
            .spacing(12),
            row![
                button("Cancel").on_press(Message::OnPressCloseKeyModal),
                button("Save").on_press(Message::OnPressSaveKey)
            ]
            .spacing(12)
        ]
        .spacing(12)
        .into();

        let modal: Element<_> = container(content)
            .style(container::bordered_box)
            .padding(12)
            .width(300)
            .into();

        let container: Element<_> = container(modal).center(Length::Fill).into();
        container
    }

    fn swatch_color_button(color: &'_ Color, is_selected: bool) -> Element<'_, Message> {
        button("")
            .width(24)
            .height(24)
            .style(move |_, _| button::Style {
                background: Some(Background::Color(color.clone())),
                border: if is_selected {
                    Border {
                        color: Color::WHITE,
                        width: 2.0,
                        ..Default::default()
                    }
                } else {
                    Border::default()
                },
                ..Default::default()
            })
            .on_press(Message::OnPressSelectSwatchColor(color.clone()))
            .into()
    }

    fn view(&self) -> Element<'_, Message> {
        let mut stack = Stack::new();

        let keys = &self.device_template.keys;

        let keys_column: Element<_> = Column::with_children(keys.iter().map(|column| {
            let row = Row::with_children(
                column
                    .iter()
                    .map(|key| key_button(key, &self.device_storage).into()),
            );
            row.spacing(12).into()
        }))
        .spacing(12)
        .into();

        let devices: Vec<_> = self.device_interface.list_devices().collect();

        let config_column: Element<_> = pick_list(devices, self.opened_device.clone(), |device| {
            Message::OnChangeOpenedDevice(device)
        })
        .placeholder("Select your keyboard")
        .into();

        let buttons_column: Element<_> = row![
            button("Save").on_press(Message::OnPressSaveChanges),
            button("Apply").on_press(Message::OnPressApplyChanges),
        ]
        .spacing(12)
        .into();

        let swatches_column = row!(
            text_input("#000000", &self.swatch_color_rgb_hex)
                .on_input(|value| Message::OnInputSwatchColorRgbHex(value))
                .width(128),
            button("+").on_press(Message::OnPressAddSwatchColor)
        )
        .extend(
            self.color_swatches
                .iter()
                .map(|color| Self::swatch_color_button(color, &self.selected_color == color)),
        )
        .spacing(12);

        let main_column = column![config_column, buttons_column, swatches_column, keys_column,]
            .spacing(12)
            .padding(12)
            .push(if let Some(msg) = &self.error_message {
                text(format!("error: {msg}")).color(Color::from_rgb8(255, 0, 0))
            } else {
                text("")
            });

        stack = stack.push(main_column);

        if let Some(key) = &self.key_modal {
            stack = stack.push(UiShell::key_modal(key, &self.key_led_rgb_hex));
        }

        stack.width(Length::Shrink).height(Length::Shrink).into()
    }

    fn update(&mut self, message: Message) {
        if message.should_dismiss_messages() {
            self.error_message = None;
        }

        match message {
            Message::OnPressOpenKeyModal(key) => {
                self.key_led_rgb_hex = String::new();
                self.key_modal = Some(key);
            }

            Message::OnPressCloseKeyModal => {
                self.key_modal = None;
            }

            Message::OnInputKeyLedRgbHex(value) => {
                self.key_led_rgb_hex = value;
            }

            Message::OnPressSaveKey => {
                let maybe_hex = HexColor::parse_rgb(&self.key_led_rgb_hex);

                let key = match &self.key_modal {
                    Some(key_data) => key_data,
                    None => unreachable!(),
                };

                if let Ok(hex) = maybe_hex {
                    self.device_storage.set_key_led(key.id, hex.r, hex.g, hex.b);
                } else {
                    self.error_message =
                        Some(format!("invalid color hex '{}'", self.key_led_rgb_hex).into());
                }

                self.key_modal = None;
            }

            Message::OnPressSaveChanges => {
                if let Err(err) = self.device_storage.flush() {
                    self.error_message = Some(err);
                }
            }

            Message::OnPressApplyChanges => {
                if let Err(err) = self
                    .device_interface
                    .save_key_leds(&self.device_storage, &self.device_template)
                {
                    self.error_message = Some(format!("device interface error: {}", err));
                }
            }

            Message::OnPressFillColor(key_id) => {
                let color = &self.selected_color;
                self.device_storage.set_key_led(
                    key_id,
                    (color.r * 255.0) as u8,
                    (color.g * 255.0) as u8,
                    (color.b * 255.0) as u8,
                );
            }

            Message::OnInputSwatchColorRgbHex(value) => {
                self.swatch_color_rgb_hex = value;
            }

            Message::OnPressAddSwatchColor => {
                let maybe_hex = HexColor::parse_rgb(&self.swatch_color_rgb_hex);

                if let Ok(hex) = maybe_hex {
                    self.color_swatches
                        .push(Color::from_rgb8(hex.r, hex.g, hex.b));
                    self.device_storage
                        .add_color_swatch(RgbData::new(hex.r, hex.g, hex.b));
                    self.swatch_color_rgb_hex = String::new();
                } else {
                    self.error_message =
                        Some(format!("invalid color hex '{}'", self.swatch_color_rgb_hex).into());
                }
            }

            Message::OnPressSelectSwatchColor(color) => {
                self.selected_color = color;
            }

            Message::OnChangeOpenedDevice(device) => {
                let maybe_device_template = DeviceTemplate::from_file(
                    format!("{}_{}.json", device.product_id, device.vendor_id).as_str(),
                );

                if let Err(err) = maybe_device_template {
                    self.error_message = Some(err.to_string());
                    return;
                } else if let Ok(device_template) = maybe_device_template {
                    self.device_template = device_template;
                }

                if let Err(err) = self.device_interface.open(device.path.clone()) {
                    self.error_message = Some(format!("device interface error: {}", err));
                    return;
                }

                self.opened_device = Some(device);
            }
        }
    }

    pub fn bootstrap(&mut self) -> iced::Result {
        iced::run(Self::update, Self::view)
    }
}
