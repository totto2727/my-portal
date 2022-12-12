use crate::domain::Result;
use async_trait::async_trait;
use derive_getters::Getters;

#[cfg(test)]
use mockall::automock;

#[cfg_attr(test, automock)]
#[async_trait]
pub trait UserRepositoryTrait: Send + Sync {}
