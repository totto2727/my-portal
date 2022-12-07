use std::error::Error;

use chrono::{DateTime, FixedOffset};
use rust_lib::portal::SourcePlatform;
use rust_lib::{custom_error::OptionalError, otor};
use serde::{Deserialize, Serialize};
use time::format_description::well_known::Rfc3339;
use twitter_v2::{ApiPayload, Tweet};

use super::tag::Tag;
use super::user::User;

#[derive(Debug, Serialize, Deserialize)]
pub struct Message {
    id: String,
    text: String,
    source_platform: SourcePlatform,
    created_at: DateTime<FixedOffset>,
    tags: Vec<Tag>,
    author: User,
}

impl Message {
    fn new<S>(
        id: S,
        source_platform: SourcePlatform,
        text: S,
        created_at: DateTime<FixedOffset>,
        author: User,
        tags: Vec<Tag>,
    ) -> Message
    where
        S: Into<String>,
    {
        Message {
            id: id.into(),
            text: text.into(),
            source_platform: source_platform.into(),
            created_at: created_at.into(),
            tags,
            author,
        }
    }
}

impl<M> TryFrom<ApiPayload<Tweet, M>> for Message {
    type Error = Box<dyn Error>;
    fn try_from(value: ApiPayload<Tweet, M>) -> Result<Self, Self::Error> {
        let tweet = otor!(value.data)?;

        let includes = otor!(value.includes)?;
        let users = otor!(includes.users)?;
        let author: User = otor!(users.get(0))?.clone().into();

        let matching_rules = otor!(value.matching_rules)?;
        let tags: Vec<Tag> = matching_rules.iter().map(|rule| rule.tag.to_owned().into()).collect();

        let rfc3389 = otor!(tweet.created_at)?.format(&Rfc3339)?;
        let created_at = DateTime::parse_from_rfc3339(&rfc3389)?;

        Ok(Self::new(
            tweet.id.to_string(),
            SourcePlatform::Twitter,
            tweet.text,
            created_at,
            author,
            tags,
        ))
    }
}
