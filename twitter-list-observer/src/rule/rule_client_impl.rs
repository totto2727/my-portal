use super::rule_client_trait::RuleClientTrait;
use crate::domain::{DomainError, Result};
use crate::rule::rule::Rule;
use async_trait::async_trait;
use derive_getters::Getters;
use rust_lib::custom_error::OptionalError;
use rust_lib::otor;
use std::collections::HashSet;
use twitter_v2::{authorization::BearerToken, TwitterApi};

#[derive(Getters)]
pub struct RuleClient<'a> {
    client: &'a TwitterApi<BearerToken>,
}

impl<'a> RuleClient<'a> {
    pub fn new(client: &'a TwitterApi<BearerToken>) -> Self {
        RuleClient { client }
    }
}

#[async_trait]
impl<'a> RuleClientTrait for RuleClient<'a> {
    async fn query_reset(&self) -> Result<()> {
        self.client()
            .get_tweets_search_stream_rules()
            .send()
            .await?;

        Ok(())
    }
    async fn query_add_rules(&self, rules: HashSet<Rule>) -> Result<()> {
        let mut builder = self.client().post_tweets_search_stream_rule().to_owned();

        for rule in rules.iter() {
            builder.add_tagged(rule.text().to_string(), rule.tag().to_string());
        }

        builder.send().await?;
        Ok(())
    }
    async fn query_get_rules(&self) -> Result<HashSet<Rule>> {
        Ok(otor!(self
            .client()
            .get_tweets_search_stream_rules()
            .send()
            .await?
            .data())?
        .to_owned()
        .into_iter()
        .filter_map(|r| r.try_into().ok())
        .collect())
    }
}
