use futures::Stream;
use rust_lib::portal::Message;
use tracing::warn;
use twitter_v2::{
    meta::SentMeta,
    query::{TweetExpansion, TweetField},
    ApiPayload, Authorization, Tweet, TwitterApi,
};

pub async fn query_stream<T: Authorization>(
    client: &TwitterApi<T>,
) -> Result<
    impl Stream<Item = Result<ApiPayload<Tweet, SentMeta>, twitter_v2::Error>>,
    twitter_v2::Error,
> {
    match client
        .get_tweets_search_stream()
        .tweet_fields([TweetField::AuthorId, TweetField::CreatedAt])
        .expansions([TweetExpansion::AuthorId])
        .stream()
        .await
    {
        Ok(suc) => return Ok(suc),
        Err(err) => {
            println!("fail to get stream");
            return Err(err);
        }
    };
}

pub fn convert_message<M>(item: Result<ApiPayload<Tweet, M>, twitter_v2::Error>)->Result<Message, Box<dyn std::error::Error>> {
    let payload = match item {
        Ok(ok) => ok,
        Err(err) => {
            warn!("fail to get tweet:{:?}", err);
            return Err(err.into());
        }
    };

    match Message::from_twitter_api_payload(payload) {
        Ok(message) => return Ok(message),
        Err(err) => {
            warn!("fail to convert message from payload:{:?}", err);
            return Err(err.into());
        }
    };
}
