mod di;
// mod domain;
// mod infrastructure;
// mod service;
mod tag;
// mod usecase;
mod domain;
mod user;

use di::di;
use rust_lib::{database::postgres::get_connection, env::load_env, portal::SourcePlatform};
use std::error::Error;
use tag::tag_repository_trait::TagRepositoryTrait;
use tracing::{error, info};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    tracing_subscriber::fmt::init();
    load_env()?;
    info!("loaded .env");

    let database = match get_connection().await {
        Ok(ok) => ok,
        Err(err) => {
            error!(err);
            return Err(err);
        }
    };
    let di_ = di(&database);
    let tags = di_
        .tag_repository()
        .find_filter_source_platform(SourcePlatform::Twitter)
        .await?;

    println!("{:?}", tags);

    Ok(())

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
