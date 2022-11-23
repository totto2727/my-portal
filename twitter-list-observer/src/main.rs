mod env;
mod queue;
mod twitter;

use crate::queue::message::Convert;
use futures::{pin_mut, prelude::*};
use rust_lib::queue::get_pool;
use rust_lib::{env::load_env, queue::Queue};
use std::error::Error;
use tokio::time::{sleep, Duration};
use tracing::{error, info, warn};
use tracing_subscriber;
use twitter::{get_api_app_ctx, query_stream, Rule};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    tracing_subscriber::fmt::init();

    load_env()?;

    let twitter = get_api_app_ctx()?;

    // let rules = [Rule::new("La priere", "from:Lapriere_info")];
    let rules = [Rule::new("cat", "猫")];

    Rule::query_initialize_rules(&twitter, &rules).await?;

    let queue_connection_pool = get_pool()?;

    Queue::PortalMessage
        .declare(&Queue::create_channel(&queue_connection_pool).await?)
        .await?;

    loop {
        let stream = match query_stream(&twitter).await {
            Ok(ok) => ok,
            Err(err) => {
                error!("fail to query stream:{:?}", err);
                sleep(Duration::from_secs(960)).await;
                continue;
            }
        };

        info!("stream start");

        pin_mut!(stream);
        while let Some(item) = stream.next().await {
            let payload = match item {
                Ok(ok) => ok,
                Err(err) => {
                    error!("fail to get tweet:{:?}", err);
                    break;
                }
            };

            let message = match payload.convert() {
                Ok(ok) => ok,
                Err(err) => {
                    warn!("fail to convert tweet to message:{:?}", err);
                    continue;
                }
            };

            let channel = match Queue::create_channel(&queue_connection_pool).await {
                Ok(ok) => ok,
                Err(err) => {
                    warn!("fail to create rabbitmq channel:{:?}", err);
                    continue;
                }
            };

            match Queue::PortalMessage.publish(&channel, &message).await {
                Ok(_) => info!("{:?}", message),
                Err(err) => {
                    warn!("fail to publish to queue:{:?}", err);
                    continue;
                }
            };
        }

        info!("stream stop");
        sleep(Duration::from_secs(960)).await;
        info!("stream restarting");
    }
}
