use rust_lib::{
    custom_error::OptionalError,
    otor,
    portal::{Message, SourcePlatform, User},
};
use twitter_v2::{ApiPayload, Tweet};

pub trait Convert<T, E> {
    fn convert(self) -> Result<T, E>;
}

impl<M> Convert<Message, OptionalError> for ApiPayload<Tweet, M> {
    fn convert(self) -> Result<Message, OptionalError> {
        let tweet = otor!(self.data)?;

        let includes = otor!(self.includes)?;
        let users = otor!(includes.users)?;
        let author = otor!(users.get(0))?.clone();

        let matching_rules = otor!(self.matching_rules)?;
        let tags: Vec<String> = matching_rules.iter().map(|rule| rule.tag.clone()).collect();

        Ok(Message {
            tag: tags,
            id: tweet.id.to_string(),
            created_at: otor!(tweet.created_at)?,
            text: tweet.text,
            source_platform: SourcePlatform::Twitter,
            author: User {
                id: author.id.to_string(),
                user_display_name: Some(author.name),
                user_name: author.username,
            },
        })
    }
}
