use super::tag::Tag;
use crate::domain::Result;
use crate::user::user::User;
use async_trait::async_trait;
use rust_lib::portal::SourcePlatform;
use std::collections::HashSet;

#[cfg(test)]
use mockall::automock;

#[cfg_attr(test, automock)]
#[async_trait]
pub trait TagRepositoryTrait: Send + Sync {
    async fn find_filter_source_platform(&self, source: SourcePlatform) -> Result<HashSet<Tag>>;
    // async fn find_all_tagged_user(
    //     &self,
    //     source_platform: SourcePlatform,
    //     tag: Tag,
    // ) -> Result<Vec<User>, Self::Error>;
}
