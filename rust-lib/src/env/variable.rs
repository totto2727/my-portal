use std::env::VarError;

pub fn env_redis_url() -> Result<String, VarError> {
    std::env::var("REDIS_URL")
}
