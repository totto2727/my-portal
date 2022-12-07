use std::env::{var, VarError};

type Result<T> = std::result::Result<T, VarError>;

pub fn env_redis_url() -> Result<String> {
    std::env::var("REDIS_URL")
}

pub fn env_rabbitmq_user() -> Result<String> {
    var("RABBITMQ_USER")
}
pub fn env_rabbitmq_pass() -> Result<String> {
    var("RABBITMQ_PASS")
}
pub fn env_rabbitmq_domain() -> Result<String> {
    var("RABBITMQ_DOMAIN")
}
pub fn env_rabbitmq_port() -> Result<String> {
    var("RABBITMQ_PORT")
}

pub fn env_surrealdb_user() -> Result<String> {
    var("SURREALDB_USER")
}

pub fn env_surrealdb_password() -> Result<String> {
    var("SURREALDB_PASSWORD")
}

pub fn env_postgres_user() -> Result<String> {
    var("POSTGRES_USER")
}

pub fn env_postgres_password() -> Result<String> {
    var("POSTGRES_PASSWORD")
}

pub fn env_postgres_db() -> Result<String> {
    var("POSTGRES_DB")
}

pub fn env_postgres_domain() -> Result<String> {
    var("POSTGRES_DOMAIN")
}

pub fn env_twitter_bearer() -> Result<String> {
    var("TWITTER_BEARER")
}
