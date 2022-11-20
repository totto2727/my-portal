use std::{env::VarError, error::Error};

use deadpool::managed::{PoolConfig, Timeouts};
use deadpool_lapin::{lapin::Channel, Config};

use crate::env::{env_rabbitmq_domain, env_rabbitmq_pass, env_rabbitmq_port, env_rabbitmq_user};

pub fn config() -> Result<Config, VarError> {
    let user = env_rabbitmq_user()?;
    let pass = env_rabbitmq_pass()?;
    let domain = env_rabbitmq_domain()?;
    let port = env_rabbitmq_port()?;

    Ok(Config {
        // vhostはuri encodeされるので%2f => "/"
        url: Some(String::from(format!(
            "amqp://{}:{}@{}:{}/%2f",
            user, pass, domain, port
        ))),
        pool: Some(PoolConfig {
            max_size: 100,
            runtime: deadpool::Runtime::Tokio1,
            timeouts: Timeouts::default(),
        }),
        ..Default::default()
    })
}

pub async fn channel() -> Result<Channel, Box<dyn Error>> {
    let pool = config()?.create_pool();
    let connection = pool.get().await?;
    Ok(connection.create_channel().await?)
}
