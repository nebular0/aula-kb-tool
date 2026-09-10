use hex_color::HexColor;
use iced::{
    Background, Border, Color, Element, Font, Length, Shadow,
    widget::{
        Button, Column, Row, Stack, button, center, column, container, row, text, text_input,
    },
};

use crate::{
    device_storage::DeviceStorage, device_template::DeviceTemplate, key_id_data::KeyIdData,
};

pub struct UiShell {
    device_storage: DeviceStorage,
    device_template: DeviceTemplate,
    key_modal: Option<KeyIdData>,
    key_led_rgb_hex: String,
}

#[derive(Clone)]
enum Message {
    OnPressOpenKeyModal(KeyIdData),
    OnPressCloseKeyModal,
    OnInputKeyLedRgbHex(String),
    OnPressSaveKey,
    OnPressSaveChanges,
}

const CONTRAST_THRESHOLD: f64 = 0.2;

fn key_button<'a>(key: &'a KeyIdData, device_storage: &'a DeviceStorage) -> Button<'a, Message> {
    let maybe_key_led = device_storage.get_key_led(key.id);

    button(center(text(&key.name).size(14)))
        .style(move |_, _| button::Style {
            background: if let Some(key_led) = maybe_key_led {
                Some(Background::Color(Color::from_rgb8(
                    key_led.r, key_led.g, key_led.b,
                )))
            } else {
                Some(Background::Color(Color::WHITE))
            },
            text_color: match maybe_key_led {
                Some(key_led)
                    if relative_luminance(key_led.r, key_led.g, key_led.b) > CONTRAST_THRESHOLD =>
                {
                    Color::BLACK
                }

                None => Color::BLACK,
                _ => Color::WHITE,
            },
            border: Border::default().rounded(4).width(0),
            ..Default::default()
        })
        .height(32)
        .width(key.width)
        .on_press(Message::OnPressOpenKeyModal(key.clone()))
}

impl Default for UiShell {
    fn default() -> Self {
        Self::new()
    }
}

fn relative_luminance(r: u8, g: u8, b: u8) -> f64 {
    // https://en.wikipedia.org/wiki/Relative_luminance
    0.2126 * (r as f64 / 255.0) + 0.7152 * (g as f64 / 255.0) + 0.0722 * (b as f64 / 255.0)
}

impl UiShell {
    pub fn new() -> Self {
        Self {
            device_storage: DeviceStorage::from_file(),
            device_template: DeviceTemplate::from_file("f75.json"),
            key_modal: None,
            key_led_rgb_hex: String::new(),
        }
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

    fn view(&self) -> Element<'_, Message> {
        let mut stack = Stack::new();

        let keys = &self.device_template.keys;

        let column: Element<_> = Column::with_children(keys.iter().map(|row| {
            let row = Row::with_children(
                row.iter()
                    .map(|key| key_button(key, &self.device_storage).into()),
            );
            row.spacing(12).into()
        }))
        .spacing(12)
        .padding(12)
        .into();

        stack = stack.push(column);

        if let Some(key) = &self.key_modal {
            stack = stack.push(UiShell::key_modal(key, &self.key_led_rgb_hex));
        }

        stack.width(Length::Fill).height(Length::Fill).into()
    }

    fn update(&mut self, message: Message) {
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

                // todo: handle invalid input

                if let Ok(hex) = maybe_hex {
                    self.device_storage.set_key_led(key.id, hex.r, hex.g, hex.b);
                    self.device_storage.flush().unwrap();
                    // todo: handle error
                }

                self.key_modal = None;
            }

            Message::OnPressSaveChanges => {
                self.device_storage.flush().unwrap();
            }
        }
    }

    pub fn bootstrap(&mut self) -> iced::Result {
        iced::run(Self::update, Self::view)
    }
}
