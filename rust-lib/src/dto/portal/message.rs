use serde::{Deserialize, Serialize};
use time::OffsetDateTime;

use super::{User, SourcePlatform, Tag};

#[derive(Debug, Serialize, Deserialize)]
pub struct Message {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    pub id_in_platform: String,
    pub text: String,
    pub source_platform: SourcePlatform,
    #[serde(with = "time::serde::rfc3339")]
    pub created_at: OffsetDateTime,
    pub tags: Vec<Tag>,
    pub author: User,
}

