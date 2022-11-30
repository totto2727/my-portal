use serde::{Deserialize, Serialize};

use super::PortalPlatform;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Channel {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    pub id_in_platform: String,
    pub name: String,
    pub portal_platform: PortalPlatform,
}
