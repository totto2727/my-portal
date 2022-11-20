mod env;
mod twitter;

use std::error::Error;

use futures::prelude::*;
use rust_lib::{env::load_env, redis::client::get_redis_client, re_export::redis::Commands};
use tracing::{error, info, warn};
use twitter::{convert_message, get_api_app_ctx, query_stream, Rule};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    tracing_subscriber::fmt::init();

    load_env()?;

    // let twitter = get_api_app_ctx()?;

    let redis = get_redis_client()?;
    let mut redis_con = redis.get_connection()?;

    redis_con.set("my_key", 42)?;
    redis_con.get("my_key")?;

    return Ok(())
    // let rules = [
    //     Rule::new("La priere", "from:Lapriere_info"),
    //     Rule::new("Cat", "猫"),
    // ];
    //
    // Rule::query_initialize_rules(&twitter, &rules).await?;
    //
    // loop {
    //     let stream = match query_stream(&twitter).await {
    //         Ok(ok) => ok,
    //         Err(err) => {
    //             error!("fail to query stream:{:?}", err);
    //             tokio::time::sleep(tokio::time::Duration::from_secs(960)).await;
    //             continue;
    //         }
    //     };
    //
    //     info!("stream start");
    //
    //     futures::pin_mut!(stream);
    //     while let Some(item) = stream.next().await {
    //         let message = match convert_message(item) {
    //             Ok(ok) => ok,
    //             Err(_) => break,
    //         };
    //
    //         if let Ok(serialized) = serde_json::to_string(&message) {
    //             info!(serialized);
    //         } else {
    //             warn!("fail to map message");
    //         }
    //     }
    //
    //     info!(r#"stream stop"#);
    //     tokio::time::sleep(tokio::time::Duration::from_secs(960)).await;
    //     info!("stream restarting");
    // }
}
