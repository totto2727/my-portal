use serde::{Deserialize, Serialize};

use super::Channel;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Tag {
    pub id: String,
    pub name: String,
    pub channels: Vec<Channel>,
}
