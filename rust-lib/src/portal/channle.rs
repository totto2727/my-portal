use serde::{Deserialize, Serialize};

use crate::dto;

use super::PortalPlatform;

#[derive(Debug, Serialize, Deserialize)]
pub struct Channel {
    pub id: String,
    pub name: String,
    pub portal_paltform: PortalPlatform,
}

impl From<dto::portal::Channel> for Channel {
    fn from(dto: dto::portal::Channel) -> Self {
        Channel {
            id: dto.id_in_platform.clone(),
            name: dto.name,
            portal_paltform: dto.portal_platform.name,
        }
    }
}
