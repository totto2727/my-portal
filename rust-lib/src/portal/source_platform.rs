use serde::{Deserialize, Serialize};
use strum_macros::EnumString;

use crate::dto;

#[derive(Debug, EnumString, Serialize, Deserialize)]
pub enum SourcePlatform {
    #[strum(serialize = "twitter")]
    #[serde(alias = "twitter")]
    Twitter,
}

impl From<dto::portal::SourcePlatform> for SourcePlatform {
    fn from(dto: dto::portal::SourcePlatform) -> Self {
        dto.name
    }
}
