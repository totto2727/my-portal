use sea_orm::DatabaseConnection;

use crate::domain::Repositories;

// use twitter_v2::{authorization::BearerToken, TwitterApi};

// #[derive(Getters)]
// pub struct Clients<'a> {}
//
// #[derive(Getters)]
// pub struct DomainServices<'a> {}
//
// #[derive(Getters)]
// pub struct ApplicationServices<'a> {}

pub fn di<'a>(
    sea_orm: &'a DatabaseConnection,
    // twitter_api: TwitterApi<BearerToken>,
) -> Repositories<'a> {
    let rp = Repositories::new(sea_orm);

    return rp;
}
