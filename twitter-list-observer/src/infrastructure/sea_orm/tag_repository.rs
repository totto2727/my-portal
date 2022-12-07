use crate::domain::{
    model::tag::Tag,
    repository::tag_repository::{TagRepository, TagRepositoryTrait},
};
use async_trait::async_trait;
use rust_lib::database::entity::tag;
use sea_orm::{DatabaseConnection, DbErr, EntityTrait};

#[async_trait]
impl<'a> TagRepositoryTrait for TagRepository<'a, DatabaseConnection> {
    type Error = DbErr;
    async fn find_all(&self) -> Result<Vec<Tag>,Self::Error> {
        let result: Vec<tag::Model> = tag::Entity::find().all(self.source).await?;
        Ok(result.iter().map(|v| v.to_owned().into()).collect())
    }
}
