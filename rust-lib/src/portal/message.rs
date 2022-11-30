use super::source_platform::SourcePlatform;
use super::Tag;
use crate::dto;
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

impl From<dto::portal::Message> for Message {
    fn from(dto: dto::portal::Message) -> Self {
        Message {
            id: dto.id_in_platform,
            text: dto.text,
            created_at: dto.created_at,
            source_platform: dto.source_platform.into(),
            author: dto.author.into(),
            tags: dto.tags.iter().map(|t| t.clone().into()).collect(),
        }
    }
}
