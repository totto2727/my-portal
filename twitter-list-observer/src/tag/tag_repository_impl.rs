use super::{tag::Tag, tag_repository_trait::TagRepositoryTrait};
use crate::domain::Result;
use crate::user::user::User;
use async_trait::async_trait;
use derive_getters::Getters;
use rust_lib::{
    database::entity::{tagged_rule, tagged_user},
    portal::SourcePlatform,
};
use sea_orm::{
    prelude::*,
    sea_query::{Query, QueryBuilder, UnionType},
    DbBackend, FromQueryResult, QuerySelect, Statement,
};
use std::collections::HashSet;

#[derive(Getters)]
pub struct TagRepository<'a> {
    connection: &'a DatabaseConnection,
}

impl<'a> TagRepository<'a> {
    pub fn new(connection: &'a DatabaseConnection) -> Self {
        Self { connection }
    }
}

#[async_trait]
impl<'a> TagRepositoryTrait for TagRepository<'a> {
    async fn find_filter_source_platform(
        &self,
        source_platform: SourcePlatform,
    ) -> Result<HashSet<Tag>> {
        #[derive(FromQueryResult)]
        struct TagName {
            tag: String,
        }

        let stmt = Query::select()
            .column(tagged_user::Column::Tag)
            .distinct()
            .and_where(tagged_user::Column::SourcePlatform.eq(source_platform.to_string()))
            .from(tagged_user::Entity)
            .unions([(
                UnionType::Distinct,
                Query::select()
                    .column(tagged_rule::Column::Tag)
                    .distinct()
                    .and_where(tagged_rule::Column::SourcePlatform.eq(source_platform.to_string()))
                    .from(tagged_rule::Entity)
                    .to_owned(),
            )])
            .to_owned();
        // .build::<QueryBuilder>(DbBackend::Postgres.get_query_builder());

        let tags: HashSet<Tag> = tagged_user::Entity::find()
            .select_only()
            .distinct()
            .filter(tagged_user::Column::SourcePlatform.eq(source_platform.to_string()))
            .from_raw_sql(DbBackend::Postgres.build(&stmt))
            // .into_values::<_, QueryAs>()
            .into_model::<TagName>()
            .all(self.connection())
            .await?
            .iter()
            .map(|t| t.tag.to_owned().into())
            .collect();

        // let tag_include_rules: Vec<String> = tagged_rule::Entity::find()
        //     .select_only()
        //     .column(tagged_rule::Column::Tag)
        //     .distinct()
        //     .filter(tagged_rule::Column::SourcePlatform.eq(source_platform.to_string()))
        //     .into_values::<_, QueryAs>()
        //     .all(self.connection())
        //     .await?;
        //
        // let tags: HashSet<Tag> = tag_include_users
        //     .into_iter()
        //     .chain(tag_include_rules)
        //     .collect::<HashSet<String>>()
        //     .iter()
        //     .map(|t| t.to_owned().into())
        //     .collect();

        Ok(tags)
    }
    // async fn find_all_tagged_user(
    //     &self,
    //     source_platform: SourcePlatform,
    //     tag: Tag,
    // ) -> Result<Vec<User>, Self::Error> {
    //     Ok(Vec::new())
    // }
}
