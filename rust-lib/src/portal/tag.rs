use serde::{Deserialize, Serialize};

use crate::dto;

use super::Channel;

#[derive(Debug, Serialize, Deserialize)]
pub struct Tag {
    pub name: String,
    pub channels: Vec<Channel>,
}

impl From<dto::portal::Tag> for Tag {
    fn from(dto: dto::portal::Tag) -> Self {
        Tag {
            name: dto.name,
            channels: dto
                .channels
                .iter()
                .map(|c| -> Channel { c.clone().into() })
                .collect(),
        }
    }
}
