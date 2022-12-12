mod di;
// mod domain;
// mod infrastructure;
// mod service;
mod tag;
// mod usecase;
mod domain;
mod rule;
mod user;

use di::di;
use itertools::Itertools;
use rule::rule_part::RulePart;
use rust_lib::{database::postgres::get_connection, env::load_env, portal::SourcePlatform};
use std::{
    collections::{HashMap, HashSet},
    error::Error,
};
use tag::{tag::Tag, tag_repository_trait::TagRepositoryTrait};
use tracing::{error, info};
use user::user::User;

use crate::rule::rule::Rule;

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

    let rules = di_
        .tag_repository()
        .find_all_tagged_rule_parts(SourcePlatform::Twitter, tags.clone())
        .await?;
    println!("{:?}", rules);

    let users = di_
        .tag_repository()
        .find_all_tagged_user(SourcePlatform::Twitter, tags.clone())
        .await?;
    println!("{:?}", users);

    let tagged: HashMap<Tag, (Option<HashSet<User>>, Option<HashSet<RulePart>>)> = tags
        .into_iter()
        .map(|tag| {
            (
                tag.clone(),
                (
                    users.get(&tag).map(|users| users.to_owned()),
                    rules.get(&tag).map(|rules| rules.to_owned()),
                ),
            )
        })
        .map(|x| x)
        .collect();

    let rules: HashSet<Rule> = tagged
        .into_iter()
        .filter_map(|(t, (us, rps))| Rule::new(SourcePlatform::Twitter, t, rps, us).ok())
        .collect();
    println!("{:?}", rules);

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
