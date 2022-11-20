mod env;
mod twitter;

use std::error::Error;

use futures::prelude::*;
use rust_lib::rabbitmq::channel;
use rust_lib::re_export::deadpool_lapin::lapin::options::{
    BasicPublishOptions, QueueDeclareOptions,
};
use rust_lib::re_export::deadpool_lapin::lapin::types::FieldTable;
use rust_lib::re_export::deadpool_lapin::lapin::BasicProperties;
use rust_lib::re_export::redis::JsonAsyncCommands;
use rust_lib::re_export::{serde, serde_json};
use rust_lib::redis::client::get_redis_client;
use rust_lib::{env::load_env, portal::Message};
use tracing::{error, info};
use twitter::{convert_message, get_api_app_ctx, query_stream, Rule};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    tracing_subscriber::fmt::init();

    load_env()?;

    let twitter = get_api_app_ctx()?;

    let rules = [
        Rule::new("La priere", "from:Lapriere_info"),
    ];

    Rule::query_initialize_rules(&twitter, &rules).await?;

    let channel = channel().await?;

    channel
        .queue_declare(
            "portal_message",
            QueueDeclareOptions::default(),
            FieldTable::default(),
        )
        .await?;

    loop {
        let stream = match query_stream(&twitter).await {
            Ok(ok) => ok,
            Err(err) => {
                error!("fail to query stream:{:?}", err);
                tokio::time::sleep(tokio::time::Duration::from_secs(960)).await;
                continue;
            }
        };

        info!("stream start");

        futures::pin_mut!(stream);
        while let Some(item) = stream.next().await {
            let message = match convert_message(item) {
                Ok(ok) => ok,
                Err(_) => break,
            };

            let json = serde_json::to_string(&message)?;
            channel
                .basic_publish(
                    "",
                    "portal_message",
                    BasicPublishOptions::default(),
                    Vec::from(json),
                    BasicProperties::default(),
                )
                .await?;
            info!("{:?}", message);
        }

        info!("stream stop");
        tokio::time::sleep(tokio::time::Duration::from_secs(960)).await;
        info!("stream restarting");
    }
}
