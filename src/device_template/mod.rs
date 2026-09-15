use crate::{key_data::KeyData, key_id::KeyId, key_id_data::KeyIdData};

use from_file_error::FromFileError;

use itertools::Itertools;
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, fs};

mod from_file_error;

#[derive(Default, Serialize, Deserialize)]
pub struct DeviceTemplateJson {
    pub keys: HashMap<KeyId, KeyData>,
}

pub struct DeviceTemplate {
    pub keys: Vec<Vec<KeyIdData>>,
}

impl DeviceTemplate {
    pub fn new() -> Self {
        Self { keys: Vec::new() }
    }

    pub fn from_file(filename: &'_ str) -> Result<Self, FromFileError> {
        let file_content =
            fs::read_to_string(filename).map_err(|err| FromFileError::IoError(err))?;
        let json = serde_json::from_str::<DeviceTemplateJson>(file_content.as_str())
            .map_err(|err| FromFileError::ParseError(err))?;

        Ok(Self {
            keys: Self::sort_keys(json),
        })
    }

    fn sort_keys(json: DeviceTemplateJson) -> Vec<Vec<KeyIdData>> {
        let keys_vec: Vec<_> = json.keys.iter().collect();

        let column = keys_vec.iter().into_group_map_by(|f| f.1.column);
        let mut keys = Vec::new();

        for row in column.iter().sorted_by_key(|r| r.0) {
            let mut rows = Vec::new();

            for key in row.1.iter().sorted_by_key(|k| k.1.row) {
                rows.push(KeyIdData::from_key_data(*key.0, key.1));
            }

            keys.push(rows);
        }

        keys
    }
}
