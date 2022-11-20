use std::env::VarError;

use twitter_v2::{TwitterApi, authorization::BearerToken};

use crate::env::env_twitter_bearer;

pub fn get_api_app_ctx() -> Result<TwitterApi<BearerToken>, VarError> {
    let twitter_bearer = env_twitter_bearer()?;

    let auth = BearerToken::new(twitter_bearer);
    Ok(TwitterApi::new(auth))
}
