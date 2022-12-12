use sea_orm::DatabaseConnection;
use twitter_v2::{authorization::BearerToken, TwitterApi};

use crate::domain::{Clients, Domains, Repositories};

// use twitter_v2::{authorization::BearerToken, TwitterApi};

// #[derive(Getters)]
// pub struct Clients<'a> {}
//
// #[derive(Getters)]
// pub struct DomainServices<'a> {}
//
// #[derive(Getters)]
// pub struct ApplicationServices<'a> {}

pub fn di<'a>(db: &'a DatabaseConnection, twitter: &'a TwitterApi<BearerToken>) -> Domains<'a> {
    let rp = Repositories::new(db);
    let cl = Clients::new(twitter);

    let domains = Domains::new(rp, cl);
    domains
}
