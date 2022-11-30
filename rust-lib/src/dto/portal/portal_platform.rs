use serde::{Deserialize, Serialize};

use crate::portal;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PortalPlatform {
    pub id: String,
    pub name: portal::PortalPlatform,
}

