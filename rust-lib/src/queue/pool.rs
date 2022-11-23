use std::error::Error;

use deadpool::Runtime;
use deadpool_lapin::{Config, PoolConfig, Timeouts};

use crate::env::{env_rabbitmq_domain, env_rabbitmq_pass, env_rabbitmq_port, env_rabbitmq_user};

pub fn get_pool() -> Result<deadpool::managed::Pool<deadpool_lapin::Manager>, Box<dyn Error>> {
    let user = env_rabbitmq_user()?;
    let pass = env_rabbitmq_pass()?;
    let domain = env_rabbitmq_domain()?;
    let port = env_rabbitmq_port()?;

    Ok(Config {
        url: Some(String::from(format!(
            "amqp://{}:{}@{}:{}/%2f",
            user, pass, domain, port
        ))),
        pool: Some(PoolConfig {
            max_size: 100,
            timeouts: Timeouts::default(),
        }),
        ..Default::default()
    }
    .create_pool(Some(Runtime::Tokio1))?)
}
