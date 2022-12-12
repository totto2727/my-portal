use std::collections::HashSet;

use crate::domain::Result;
use async_trait::async_trait;

use super::rule::Rule;

#[async_trait]
pub trait RuleClientTrait: Send + Sync {
    async fn query_reset(&self) -> Result<()>;
    async fn query_add_rules(&self, rules: HashSet<Rule>) -> Result<()>;
    async fn query_get_rules(&self) -> Result<HashSet<Rule>>;
}
