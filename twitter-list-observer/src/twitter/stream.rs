use futures::Stream;
use twitter_v2::{meta::SentMeta, query::{TweetField, TweetExpansion}, ApiPayload, Authorization, Tweet, TwitterApi};

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
