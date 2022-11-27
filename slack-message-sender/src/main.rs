use std::error::Error;

use futures::stream::StreamExt;
use lapin::options::BasicAckOptions;
use rust_lib::{
    env::load_env,
    portal::Message,
    queue::{get_pool, Queue},
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    tracing_subscriber::fmt::init();

    load_env()?;
    let queue_connection_pool = get_pool()?;
    let channel = Queue::create_channel(&queue_connection_pool).await?;
    let mut consumer = Queue::PortalMessage.consume(&channel, None).await?;
    while let Some(delivery) = consumer.next().await {
        let delivery = delivery.expect("error in consumer");
        let message: Message = serde_json::from_slice(&delivery.data)?;
        println!("{:?}", message);
        delivery.ack(BasicAckOptions::default()).await.expect("ack");
    }

    Ok(())
}
