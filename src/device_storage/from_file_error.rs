use std::{fmt::Display, io};

pub enum FromFileError {
    IoError(io::Error),
    ParseError(serde_json::Error),
}

impl Display for FromFileError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::IoError(err) => write!(f, "input-output error ({err})"),
            Self::ParseError(err) => write!(f, "json parse error ({err})"),
        }
    }
}
