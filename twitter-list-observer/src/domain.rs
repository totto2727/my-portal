use derive_getters::Getters;
use sea_orm::{DatabaseConnection, DbErr};
use thiserror::Error;

use crate::tag::tag_repository_impl::TagRepository;

#[derive(Getters)]
pub struct Repositories<'a> {
    tag_repository: TagRepository<'a>,
}

impl<'a> Repositories<'a> {
    pub fn new(db: &'a DatabaseConnection) -> Self {
        Self {
            tag_repository: TagRepository::new(db),
        }
    }
}

#[derive(Debug, Error)]
pub enum DomainError {
    #[error("{0}")]
    Validation(String),
    #[error(r#"{entity_type} was not found for entity_id "{entity_id}" and user_id "{user_id}"."#)]
    NotFound {
        entity_type: &'static str,
        entity_id: String,
        user_id: String,
    },
    #[error(transparent)]
    InfrastructureError(anyhow::Error),
    #[error("{0}")]
    Unexpected(String),
}

pub type Result<T> = std::result::Result<T, DomainError>;

impl From<DbErr> for DomainError {
    fn from(error: DbErr) -> Self {
        DomainError::InfrastructureError(error.into())
    }
}

impl From<twitter_v2::Error> for DomainError {
    fn from(error: twitter_v2::Error) -> Self {
        DomainError::InfrastructureError(error.into())
    }
}
