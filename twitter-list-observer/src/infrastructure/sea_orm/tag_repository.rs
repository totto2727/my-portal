use crate::domain::{
    model::{tag::Tag, user::User},
    repository::tag_repository::{TagRepository, TagRepositoryTrait},
};
use async_trait::async_trait;
use rust_lib::{
    database::entity::{tag, tagged_user, users},
    portal::SourcePlatform,
};
use sea_orm::prelude::*;

#[async_trait]
impl<'a> TagRepositoryTrait for TagRepository<'a, DatabaseConnection> {
    type Error = DbErr;

    async fn find_all(&self) -> Result<Vec<Tag>, Self::Error> {
        let result: Vec<tag::Model> = tag::Entity::find().all(self.repository()).await?;
        Ok(result.iter().map(|v| v.to_owned().into()).collect())
    }

    async fn find_all_tagged_user(
        &self,
        source_platform: SourcePlatform,
        tag: Tag,
    ) -> Result<(Tag, Vec<User>), Self::Error> {
        let tagged_users: Vec<(tagged_user::Model, Vec<users::Model>)> =
            tagged_user::Entity::find()
                .filter(tagged_user::Column::Tag.contains(tag.name()))
                .filter(tagged_user::Column::SourcePlatform.contains(source_platform.into()))
                .find_with_related(users::Entity)
                .all(self.repository())
                .await?;

        let users: Vec<User> = tagged_users
            .iter()
            .flat_map(|(_, users)| users.to_owned())
            .map(|user| user.into())
            .collect();

        Ok((tag, users))
    }
}
