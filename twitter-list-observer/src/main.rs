mod env;
mod twitter;

use std::error::Error;

use futures::prelude::*;
use rust_lib::{env::load_env, portal::Message};
use tracing::{error, info, warn};
use twitter::{get_api_app_ctx, Rule, query_stream};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    tracing_subscriber::fmt::init();

    load_env()?;

    let client = get_api_app_ctx()?;

    let rules = [
        Rule::new("La priere", "from:Lapriere_info"),
        Rule::new("Cat", "猫"),
    ];

    Rule::query_initialize_rules(&client, &rules).await?;

    loop {
        let stream = match query_stream(&client).await {
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
            let payload = match item {
                Ok(ok) => ok,
                Err(err) => {
                    warn!("fail to get tweet:{:?}", err);
                    break;
                }
            };

            let message = match Message::from_twitter_api_payload(payload){
                Some(some)=>some,
                None =>{
                    warn!("fail to convert message from payload");
                    break;
                }
            };

            if let Ok(serialized) = serde_json::to_string(&message) {
                info!(serialized);
            } else {
                warn!("fail to map message");
            }
        }

        info!(r#"stream stop"#);
        tokio::time::sleep(tokio::time::Duration::from_secs(960)).await;
        info!("stream restarting");
    }
}
