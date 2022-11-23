use futures::Stream;
use twitter_v2::Error;
use twitter_v2::{
    meta::SentMeta,
    query::{TweetExpansion, TweetField},
    ApiPayload, Authorization, Tweet, TwitterApi,
};

pub async fn query_stream<T: Authorization>(
    client: &TwitterApi<T>,
) -> Result<impl Stream<Item = Result<ApiPayload<Tweet, SentMeta>, twitter_v2::Error>>, Error> {
    match client
        .get_tweets_search_stream()
        .tweet_fields([TweetField::AuthorId, TweetField::CreatedAt])
        .expansions([TweetExpansion::AuthorId])
        .stream()
        .await
    {
        Ok(suc) => Ok(suc),
        Err(err) => {
            println!("fail to get stream");
            Err(err)
        }
    }
}
