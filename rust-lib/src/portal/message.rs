use serde::{Deserialize, Serialize};
use time::OffsetDateTime;
use twitter_v2::{ApiPayload, Tweet};

use crate::{
    custom_error::OptionError,
    portal::author::{Author, TwitterAuther}, otor,
};

#[derive(Debug, Serialize, Deserialize)]
pub struct Message {
    pub tag: Vec<String>,
    pub id: String,
    pub author: Author,
    #[serde(with = "time::serde::rfc3339")]
    pub created_at: OffsetDateTime,
    pub message: String,
}

impl Message {
    pub fn from_twitter_api_payload<M>(
        payload: ApiPayload<Tweet, M>,
    ) -> Result<Message, OptionError> {
        let tweet = otor!(payload.data)?;

        let includes = otor!(payload.includes)?;
        let users = otor!(includes.users)?;
        let author = otor!(users.get(0))?.clone();

        let matching_rules = otor!(payload.matching_rules)?;
        let tags: Vec<String> = matching_rules.iter().map(|rule| rule.tag.clone()).collect();

        return Ok(Message {
            tag: tags,
            id: tweet.id.to_string(),
            author: Author::TwitterAuther(TwitterAuther {
                id: author.id.to_string(),
                name: author.name,
                user_name: author.username,
            }),
            created_at: otor!(tweet.created_at)?,
            message: tweet.text,
        });
    }
}
