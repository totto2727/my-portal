use super::source_platform::SourcePlatform;
use super::Tag;
use crate::database::portal;
use crate::portal::User;
use serde::{Deserialize, Serialize};
use time::OffsetDateTime;

#[derive(Debug, Serialize, Deserialize)]
pub struct Message {
    pub id: String,
    pub text: String,
    pub source_platform: SourcePlatform,
    #[serde(with = "time::serde::rfc3339")]
    pub created_at: OffsetDateTime,
    pub tags: Vec<Tag>,
    pub author: User,
}

impl From<portal::Message> for Message {
    fn from(value: portal::Message) -> Self {
        Message {
            id: value.id_in_platform,
            text: value.text,
            created_at: value.created_at,
            source_platform: value.source_platform.into(),
            author: value.author.into(),
            tags: value.tags.iter().map(|t| t.clone().into()).collect(),
        }
    }
}
