use serde::{Deserialize, Serialize};
use time::OffsetDateTime;

use crate::portal::User;

use super::source_platform::SourcePlatform;

#[derive(Debug, Serialize, Deserialize)]
pub struct Message {
    pub id: String,
    pub text: String,
    pub source_platform: SourcePlatform,
    #[serde(with = "time::serde::rfc3339")]
    pub created_at: OffsetDateTime,
    pub tag: Vec<String>,
    pub author: User,
}
