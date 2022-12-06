use crate::{m20221203_185943_create_tag::Tag, m20221204_012419_create_rule::Rule};
use rust_lib::portal;
use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(TaggedRule::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(TaggedRule::Tag).string())
                    .col(ColumnDef::new(TaggedRule::Rule).string())
                    .col(ColumnDef::new(TaggedRule::SourcePlatform).string())
                    .primary_key(
                        Index::create()
                            .col(TaggedRule::SourcePlatform)
                            .col(TaggedRule::Tag)
                            .col(TaggedRule::Rule),
                    )
                    .foreign_key(
                        ForeignKeyCreateStatement::new()
                            .from(TaggedRule::Table, TaggedRule::Tag)
                            .to(Tag::Table, Tag::Name)
                            .on_update(ForeignKeyAction::Cascade)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKeyCreateStatement::new()
                            .from_tbl(TaggedRule::Table)
                            .from_col(TaggedRule::Rule)
                            .from_col(TaggedRule::SourcePlatform)
                            .to_tbl(Rule::Table)
                            .to_col(Rule::Name)
                            .to_col(Rule::SourcePlatform)
                            .on_update(ForeignKeyAction::Cascade)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .exec_stmt(
                Query::insert()
                    .into_table(TaggedRule::Table)
                    .columns([
                        TaggedRule::Rule,
                        TaggedRule::SourcePlatform,
                        TaggedRule::Tag,
                    ])
                    .on_conflict(
                        OnConflict::columns([
                            TaggedRule::Rule,
                            TaggedRule::SourcePlatform,
                            TaggedRule::Tag,
                        ])
                        .do_nothing()
                        .to_owned(),
                    )
                    .values_panic([
                        "Test".to_string().into(),
                        portal::SourcePlatform::Twitter.to_string().into(),
                        "La priere".into(),
                    ])
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(TaggedRule::Table).to_owned())
            .await
    }
}

/// Learn more at https://docs.rs/sea-query#iden
#[derive(Iden)]
enum TaggedRule {
    Table,
    Tag,
    Rule,
    SourcePlatform,
}
