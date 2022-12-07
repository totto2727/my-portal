use async_trait::async_trait;
use derive_getters::Getters;

#[derive(Getters)]
pub struct RuleApiService<'a, Api> {
    api: &'a Api,
}

impl<'a, Api> RuleApiService<'a, Api> {
    pub fn new(api: &'a Api) -> Self {
        RuleApiService { api }
    }
}

#[async_trait]
pub trait RuleApiServiceTrait: Sized {
    type Error;
    async fn query_reset(&self) -> Result<(), Self::Error>;
    async fn query_add_rule(&self) -> Result<(), Self::Error>;
}
