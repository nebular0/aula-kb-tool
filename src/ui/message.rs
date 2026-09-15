use iced::Color;

use crate::{key_id::KeyId, key_id_data::KeyIdData};

#[derive(Clone)]
pub enum Message {
    OnPressOpenKeyModal(KeyIdData),
    OnPressCloseKeyModal,
    OnInputKeyLedRgbHex(String),
    OnPressSaveKey,
    OnPressSaveChanges,
    OnPressApplyChanges,
    OnPressFillColor(KeyId),
    OnInputSwatchColorRgbHex(String),
    OnPressAddSwatchColor,
    OnPressSelectSwatchColor(Color),
}

impl Message {
    pub fn should_dismiss_messages(&self) -> bool {
        match self {
            Self::OnInputKeyLedRgbHex(_) => false,
            Self::OnInputSwatchColorRgbHex(_) => false,
            _ => true,
        }
    }
}
