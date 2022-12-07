use sea_orm::DatabaseConnection;
use twitter_v2::{authorization::BearerToken, TwitterApi};

use super::domain::{
    repository::tag_repository::TagRepository, service::rule_api_service::RuleApiService,
};

pub fn di(sea_orm: DatabaseConnection, twitter_api: TwitterApi<BearerToken>) -> () {
    let tab_repository = TagRepository::new(&sea_orm);

    let rule_repository = RuleApiService::new(&twitter_api);
}
