mod env;
mod twitter;

use futures::{pin_mut, prelude::*};
use rust_lib::custom_error::OptionalError;
use rust_lib::db::get_client;
// use rust_lib::dto::portal::Message;
use rust_lib::otor;
use rust_lib::queue::get_pool;
use rust_lib::{env::load_env, queue::Queue};
use std::collections::VecDeque;
use std::error::Error;
use surrealdb_rs::param::from_value;
use tokio::time::{sleep, Duration};
use tracing::{error, info, warn};
use twitter::{get_api_app_ctx, query_stream, Rule};
use twitter::Message;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    tracing_subscriber::fmt::init();

    load_env()?;
    info!("loaded .env");

    let db = get_client("test", "test").await?;
    info!("connected");
    //
    // let messages = db
    //     .query("select *, (select *, ->tags__channels.out.* as channels from (select in from message:lapriere1<-tags__messages) fetch channels.portal_platform) as tags from message:lapriere1 fetch author, author.source_platform, source_platform, tags, tags->tags__channels")
    //     .await?;
    // let message_object = otor!(otor!(messages.get(0))?.clone()?.get(0))?.to_owned();
    // let message:Message = from_value(&message_object)?;
    // info!("{:?}", message);
    //
    // Ok(())

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

            let message: Message = match payload.try_into() {
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
