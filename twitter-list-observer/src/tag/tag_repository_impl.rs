use super::{tag::Tag, tag_repository_trait::TagRepositoryTrait};
use crate::{domain::Result, rule::rule_part::RulePart, user::user::User};
use async_trait::async_trait;
use derive_getters::Getters;
use itertools::Itertools;
use rust_lib::{
    database::entity::{rule, tagged_rule, tagged_user, users},
    portal::SourcePlatform,
};
use sea_orm::{
    prelude::*,
    sea_query::{Query, UnionType},
    Condition, DbBackend, FromQueryResult, QuerySelect,
};
use std::collections::{HashMap, HashSet};

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
        struct QueryAs {
            tag: String,
        }

        let sub_query = Query::select()
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

        let tags: HashSet<Tag> = tagged_user::Entity::find()
            .select_only()
            .distinct()
            .filter(tagged_user::Column::SourcePlatform.eq(source_platform.to_string()))
            .from_raw_sql(DbBackend::Postgres.build(&sub_query))
            .into_model::<QueryAs>()
            .all(self.connection())
            .await?
            .iter()
            .map(|t| t.tag.clone().into())
            .collect();

        Ok(tags)
    }

    async fn find_all_tagged_user(
        &self,
        source_platform: SourcePlatform,
        tags: HashSet<Tag>,
    ) -> Result<HashMap<Tag, HashSet<User>>> {
        let tagged_users: HashMap<Tag, HashSet<User>> = tagged_user::Entity::find()
            .filter(
                Condition::all()
                    .add(tagged_user::Column::Tag.is_in(tags.iter().map(|t| t.name().to_owned())))
                    .add(tagged_user::Column::SourcePlatform.eq(source_platform.to_string())),
            )
            .find_also_related(users::Entity)
            .all(self.connection())
            .await?
            .into_iter()
            .filter_map(|(tag, user_o)| {
                user_o
                    .to_owned()
                    .map(|user| (Tag::from(tag.to_owned()), User::from(user)))
            })
            .into_group_map()
            .into_iter()
            .map(|(k, v)| (k, HashSet::from_iter(v)))
            .collect();

        Ok(tagged_users)
    }

    async fn find_all_tagged_rule_parts(
        &self,
        source_platform: SourcePlatform,
        tags: HashSet<Tag>,
    ) -> Result<HashMap<Tag, HashSet<RulePart>>> {
        let tagged_rules: HashMap<Tag, HashSet<RulePart>> = tagged_rule::Entity::find()
            .filter(
                Condition::all()
                    .add(tagged_rule::Column::Tag.is_in(tags.iter().map(|t| t.name().to_owned())))
                    .add(tagged_rule::Column::SourcePlatform.eq(source_platform.to_string())),
            )
            .find_also_related(rule::Entity)
            .all(self.connection())
            .await?
            .into_iter()
            .filter_map(|(tag, rule_o)| {
                rule_o
                    .to_owned()
                    .map(|rule| (Tag::from(tag.to_owned()), RulePart::from(rule)))
            })
            .into_group_map()
            .into_iter()
            .map(|(k, v)| (k, HashSet::from_iter(v)))
            .collect();

        Ok(tagged_rules)
    }
}
