use sea_orm::DbErr;
// pub mod client;
// pub mod model;
// pub mod repository;
// pub mod service;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum DomainError {
    // #[error("{0}")]
    // Validation(String),
    // #[error(r#"{entity_type} was not found for entity_id "{entity_id}" and user_id "{user_id}"."#)]
    // NotFound {
    //     entity_type: &'static str,
    //     entity_id: String,
    //     user_id: String,
    // },
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
