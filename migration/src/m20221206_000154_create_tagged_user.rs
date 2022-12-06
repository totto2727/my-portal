use rust_lib::portal;
use sea_orm_migration::prelude::*;

use crate::{m20221203_185943_create_tag::Tag, m20221206_000136_create_user::Users};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(TaggedUser::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(TaggedUser::Tag).string())
                    .col(ColumnDef::new(TaggedUser::UserId).string())
                    .col(ColumnDef::new(TaggedUser::SourcePlatform).string())
                    .col(
                        ColumnDef::new(TaggedUser::CreatedAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .col(
                        ColumnDef::new(TaggedUser::UpdatedAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .primary_key(
                        Index::create()
                            .col(TaggedUser::SourcePlatform)
                            .col(TaggedUser::Tag)
                            .col(TaggedUser::UserId),
                    )
                    .foreign_key(
                        ForeignKeyCreateStatement::new()
                            .from(TaggedUser::Table, TaggedUser::Tag)
                            .to(Tag::Table, Tag::Name)
                            .on_update(ForeignKeyAction::Cascade)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKeyCreateStatement::new()
                            .from_tbl(TaggedUser::Table)
                            .from_col(TaggedUser::UserId)
                            .from_col(TaggedUser::SourcePlatform)
                            .to_tbl(Users::Table)
                            .to_col(Users::Id)
                            .to_col(Users::SourcePlatform)
                            .on_update(ForeignKeyAction::Cascade)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .exec_stmt(
                Query::insert()
                    .into_table(TaggedUser::Table)
                    .columns([
                        TaggedUser::UserId,
                        TaggedUser::SourcePlatform,
                        TaggedUser::Tag,
                    ])
                    .on_conflict(
                        OnConflict::columns([
                            TaggedUser::UserId,
                            TaggedUser::SourcePlatform,
                            TaggedUser::Tag,
                        ])
                        .do_nothing()
                        .to_owned(),
                    )
                    .values_panic([
                        "1222832820906680320".into(),
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
            .drop_table(Table::drop().table(TaggedUser::Table).to_owned())
            .await
    }
}

/// Learn more at https://docs.rs/sea-query#iden
#[derive(Iden)]
pub enum TaggedUser {
    Table,
    Tag,
    UserId,
    SourcePlatform,
    CreatedAt,
    UpdatedAt,
}
