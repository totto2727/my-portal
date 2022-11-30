use serde::{Deserialize, Serialize};

use crate::portal;

#[derive(Debug, Serialize, Deserialize)]
pub struct SourcePlatform {
    pub id: String,
    pub name: portal::SourcePlatform,
}

