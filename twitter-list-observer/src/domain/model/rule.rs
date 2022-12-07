use derive_getters::Getters;
use tracing::{error, info, warn};
use twitter_v2::{data::StreamRule, requests::StreamRuleBuilder, Authorization, TwitterApi};

#[derive(Debug, Clone, Getters)]
pub struct Rule {
    tag: String,
    text: String,
}

impl Rule {
    fn new<S: Into<String>>(tag: S, value: S) -> Rule {
        Rule {
            tag: tag.into(),
            text: value.into(),
        }
    }

    // TODO test
    pub fn add_tagged_rules_to_request<T: Authorization>(
        builder: &mut StreamRuleBuilder<T>,
        rules: &[Rule],
    ) {
        for rule in rules.iter() {
            builder.add_tagged(rule.text.clone(), rule.tag.clone());
        }
    }

    // TODO test
    pub async fn query_reset_rules<T: Authorization>(
        client: &TwitterApi<T>,
    ) -> Result<(), twitter_v2::Error> {
        let result = client.get_tweets_search_stream_rules().send().await?;

        if let Some(rules) = result.data() {
            match client
                .post_tweets_search_stream_rule()
                .delete_ids(rules.iter().map(|rule| rule.id))
                .send()
                .await
            {
                Ok(_) => info!("rules is reset"),
                Err(err) => {
                    error!("fail to reset rules:{:?}", err);
                    return Err(err);
                }
            };
        }
        Ok(())
    }

    // TODO test
    pub async fn query_initialize_rules<T: Authorization>(
        client: &TwitterApi<T>,
        rules: &[Rule],
    ) -> Result<Option<Vec<StreamRule>>, twitter_v2::Error> {
        info!("initilizing rule");

        Self::query_reset_rules(client).await?;

        let mut req_update_rules = client.post_tweets_search_stream_rule();
        Self::add_tagged_rules_to_request(&mut req_update_rules, rules);

        let res_update_rule = match req_update_rules.send().await {
            Ok(suc) => {
                if let Some(rules) = suc.data() {
                    info!("rules is updated:{:?}", rules);
                    Some(rules.clone())
                } else {
                    warn!("rules is empty");
                    None
                }
            }
            Err(err) => {
                error!("fail to update rules:{:?}", err);
                return Err(err);
            }
        };

        info!("rule has initilized");
        Ok(res_update_rule)
    }
}
