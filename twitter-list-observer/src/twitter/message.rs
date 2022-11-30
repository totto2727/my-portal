use super::User;
use rust_lib::portal::SourcePlatform;
use rust_lib::{custom_error::OptionalError, otor};
use serde::{Deserialize, Serialize};
use time::OffsetDateTime;
use twitter_v2::{ApiPayload, Tweet};

#[derive(Debug, Serialize, Deserialize)]
pub struct Message {
    pub id: String,
    pub text: String,
    pub source_platform: SourcePlatform,
    #[serde(with = "time::serde::rfc3339")]
    pub created_at: OffsetDateTime,
    pub tags: Vec<String>,
    pub author: User,
}

impl<M> TryFrom<ApiPayload<Tweet, M>> for Message {
    type Error = OptionalError;
    fn try_from(value: ApiPayload<Tweet, M>) -> Result<Self, Self::Error> {
        let tweet = otor!(value.data)?;

        let includes = otor!(value.includes)?;
        let users = otor!(includes.users)?;
        let author:User = otor!(users.get(0))?.clone().into();

        let matching_rules = otor!(value.matching_rules)?;
        let tags: Vec<String> = matching_rules.iter().map(|rule| rule.tag.clone()).collect();

        Ok(Message {
            id: tweet.id.to_string(),
            created_at: otor!(tweet.created_at)?,
            text: tweet.text,
            source_platform: SourcePlatform::Twitter,
            tags,
            author,
        })
    }
}
