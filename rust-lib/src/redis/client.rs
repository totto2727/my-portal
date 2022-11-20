use std::error::Error;

use redis::Client;

use crate::env::env_redis_url;

pub fn get_redis_client() -> Result<Client, Box<dyn Error>> {
    let redis_url = env_redis_url()?;
    return Ok(redis::Client::open(redis_url)?);
}
