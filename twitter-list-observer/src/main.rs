mod lib;

use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    Ok(())
    // tracing_subscriber::fmt::init();
    //
    // load_env()?;
    // info!("loaded .env");
    //
    // let database_connection = database::get_connection().await?;
    // info!("connected");
    //
    // // let tagged_message_query: Option<(tagged_message::Model, Option<message::Model>)> =
    // //     tagged_message::Entity::find()
    // //         .find_also_related(message::Entity)
    // //         .one(&db_con)
    // //         .await?;
    //
    // let twitter_client = twitter::get_client()?;
    //
    // // let rules = [Rule::new("La priere", "from:Lapriere_info")];
    // let rules = [Rule::new("cat", "猫")];
    //
    // Rule::query_initialize_rules(&twitter_client, &rules).await?;
    //
    // let queue_connection_pool = get_pool()?;
    //
    // Queue::PortalMessage
    //     .declare(&Queue::create_channel(&queue_connection_pool).await?)
    //     .await?;
    //
    // loop {
    //     let stream = match twitter::query_stream(&twitter_client).await {
    //         Ok(ok) => ok,
    //         Err(err) => {
    //             error!("fail to query stream:{:?}", err);
    //             sleep(Duration::from_secs(960)).await;
    //             continue;
    //         }
    //     };
    //
    //     info!("stream start");
    //
    //     pin_mut!(stream);
    //     while let Some(item) = stream.next().await {
    //         let payload = match item {
    //             Ok(ok) => ok,
    //             Err(err) => {
    //                 error!("fail to get tweet:{:?}", err);
    //                 break;
    //             }
    //         };
    //
    //         let message: twitter::Message = match payload.try_into() {
    //             Ok(ok) => ok,
    //             Err(err) => {
    //                 warn!("fail to convert tweet to message:{:?}", err);
    //                 continue;
    //             }
    //         };
    //
    //         let channel = match Queue::create_channel(&queue_connection_pool).await {
    //             Ok(ok) => ok,
    //             Err(err) => {
    //                 warn!("fail to create rabbitmq channel:{:?}", err);
    //                 continue;
    //             }
    //         };
    //
    //         match Queue::PortalMessage.publish(&channel, &message).await {
    //             Ok(_) => info!("{:?}", message),
    //             Err(err) => {
    //                 warn!("fail to publish to queue:{:?}", err);
    //                 continue;
    //             }
    //         };
    //     }
    //
    //     info!("stream stop");
    //     sleep(Duration::from_secs(960)).await;
    //     info!("stream restarting");
    // }
}
