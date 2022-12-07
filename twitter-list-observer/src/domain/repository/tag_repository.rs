use async_trait::async_trait;

use crate::twitter::domain::model::tag::Tag;

pub struct TagRepository<'a, Source: 'a> {
    pub source: &'a Source,
}

impl<'a, Source: 'a> TagRepository<'a, Source> {
    pub fn new(source: &'a Source) -> TagRepository<Source> {
        TagRepository { source }
    }
}

#[async_trait]
pub trait TagRepositoryTrait: Sized {
    type Error;
    async fn find_all(&self) -> Result<Vec<Tag>, Self::Error>;
}
