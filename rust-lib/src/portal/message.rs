use serde::{Deserialize, Serialize};
use time::OffsetDateTime;
use twitter_v2::{ApiPayload, Tweet};

use crate::portal::author::{Author, TwitterAuther};

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
    pub fn from_twitter_api_payload<M>(payload: ApiPayload<Tweet, M>) -> Option<Message> {
        let tweet = payload.data?;
        let author = payload.includes?.users?.pop()?;
        let matching_rules = payload.matching_rules?;

        let tags: Vec<String> = matching_rules.iter().map(|rule| rule.tag.clone()).collect();

        return Some(Message {
            tag: tags,
            id: tweet.id.to_string(),
            author: Author::TwitterAuther(TwitterAuther {
                id: author.id.to_string(),
                name: author.name,
                user_name: author.username,
            }),
            created_at: tweet.created_at?,
            message: tweet.text,
        });
    }
}

