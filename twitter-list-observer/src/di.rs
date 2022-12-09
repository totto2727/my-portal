use derive_getters::Getters;
use sea_orm::DatabaseConnection;

use crate::tag::tag_repository_impl::TagRepository;
// use twitter_v2::{authorization::BearerToken, TwitterApi};

#[derive(Getters)]
pub struct Repositories<'a> {
    tag_repository: TagRepository<'a>,
}

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
    let tag_repository = TagRepository::new(sea_orm);

    let rp = Repositories { tag_repository };

    return rp;
}
