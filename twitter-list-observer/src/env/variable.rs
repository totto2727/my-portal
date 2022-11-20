use std::env::VarError;

pub fn env_twitter_bearer()->Result<String, VarError>{
    std::env::var("TWITTER_BEARER")
}

