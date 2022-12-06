use serde::{Deserialize, Serialize};

use crate::database::portal;

use super::PortalPlatform;

#[derive(Debug, Serialize, Deserialize)]
pub struct Channel {
    pub id: String,
    pub name: String,
    pub portal_paltform: PortalPlatform,
}

impl From<portal::Channel> for Channel {
    fn from(value: portal::Channel) -> Self {
        Channel {
            id: value.id_in_platform.clone(),
            name: value.name,
            portal_paltform: value.portal_platform.name,
        }
    }
}
