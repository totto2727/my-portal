use serde::{Deserialize, Serialize};

use crate::database::portal;

use super::Channel;

#[derive(Debug, Serialize, Deserialize)]
pub struct Tag {
    pub name: String,
    pub channels: Vec<Channel>,
}

impl From<portal::Tag> for Tag {
    fn from(value: portal::Tag) -> Self {
        Tag {
            name: value.name,
            channels: value
                .channels
                .iter()
                .map(|c| -> Channel { c.clone().into() })
                .collect(),
        }
    }
}
