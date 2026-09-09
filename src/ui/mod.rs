use iced::{
    Background, Border, Color, Element, Shadow,
    widget::{Button, Column, Row, button, center, text},
};

use crate::{device_template::DeviceTemplate, key_id_data::KeyIdData};

pub struct UiShell {
    device_template: DeviceTemplate,
}

#[derive(Clone, Copy)]
enum Message {
    None,
}

fn key_button<'a>(key: &'a KeyIdData) -> Button<'a, Message> {
    button(center(text(&key.name).size(14)))
        .style(|_, _| button::Style {
            background: Some(Background::Color(Color::WHITE)),
            text_color: Color::BLACK,
            border: Border::default().rounded(8).width(0),
            shadow: Shadow::default(),
            snap: false,
        })
        .height(32)
        .width(key.width)
        .on_press(Message::None)
}

impl Default for UiShell {
    fn default() -> Self {
        Self::new()
    }
}

impl UiShell {
    pub fn new() -> Self {
        Self {
            device_template: DeviceTemplate::from_file("f75.json"),
        }
    }

    fn view(&self) -> Element<'_, Message> {
        let keys = &self.device_template.keys;

        let column = Column::with_children(keys.iter().map(|row| {
            let row = Row::with_children(row.iter().map(|key| key_button(&key).into()));
            row.spacing(8).into()
        }));

        column.spacing(8).padding(8).into()
    }

    fn update(&mut self, message: Message) {
        ()
    }

    pub fn bootstrap(&mut self) -> iced::Result {
        iced::run(Self::update, Self::view)
    }
}
