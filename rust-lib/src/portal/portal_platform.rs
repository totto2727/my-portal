use serde::{Deserialize, Serialize};
use strum_macros::EnumString;

use crate::dto;

#[derive(Debug, EnumString, Serialize, Deserialize, Clone)]
pub enum PortalPlatform {
    #[strum(serialize = "slack")]
    #[serde(alias = "slack")]
    Slack,
}

impl From<dto::portal::PortalPlatform> for PortalPlatform {
    fn from(dto: dto::portal::PortalPlatform) -> Self {
        dto.name
    }
}
