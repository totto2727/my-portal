use crate::twitter::domain::service::rule_api_service::{RuleApiService, RuleApiServiceTrait};
use async_trait::async_trait;
use twitter_v2::{authorization::BearerToken, Authorization, Error, TwitterApi};

impl<'a, A: Authorization> RuleApiService<'a, TwitterApi<A>> {
}

#[async_trait]
impl<'a> RuleApiServiceTrait for RuleApiService<'a, TwitterApi<BearerToken>> {
    type Error = Error;
    async fn query_reset(&self) -> Result<(), Self::Error> {
        self.api()
            .get_tweets_search_stream_rules()
            .send()
            .await
            .map(|_| ())
    }
    async fn query_add_rule(&self) -> Result<(), Self::Error> {
        Ok(())
    }
}
