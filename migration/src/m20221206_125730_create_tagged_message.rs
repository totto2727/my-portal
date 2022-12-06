use rust_lib::portal;
use sea_orm_migration::prelude::*;

use crate::{m20221203_185943_create_tag::Tag, m20221206_125718_create_message::Message};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(TaggedMessage::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(TaggedMessage::Tag).string())
                    .col(ColumnDef::new(TaggedMessage::MessageId).string())
                    .col(ColumnDef::new(TaggedMessage::SourcePlatform).string())
                    .col(
                        ColumnDef::new(TaggedMessage::CreatedAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .col(
                        ColumnDef::new(TaggedMessage::UpdatedAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .primary_key(
                        Index::create()
                            .col(TaggedMessage::SourcePlatform)
                            .col(TaggedMessage::Tag)
                            .col(TaggedMessage::MessageId),
                    )
                    .foreign_key(
                        ForeignKeyCreateStatement::new()
                            .from(TaggedMessage::Table, TaggedMessage::Tag)
                            .to(Tag::Table, Tag::Name)
                            .on_update(ForeignKeyAction::Cascade)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKeyCreateStatement::new()
                            .from_tbl(TaggedMessage::Table)
                            .from_col(TaggedMessage::MessageId)
                            .from_col(TaggedMessage::SourcePlatform)
                            .to_tbl(Message::Table)
                            .to_col(Message::Id)
                            .to_col(Message::SourcePlatform)
                            .on_update(ForeignKeyAction::Cascade)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .exec_stmt(
                Query::insert()
                    .into_table(TaggedMessage::Table)
                    .columns([
                        TaggedMessage::MessageId,
                        TaggedMessage::SourcePlatform,
                        TaggedMessage::Tag,
                    ])
                    .on_conflict(
                        OnConflict::columns([
                            TaggedMessage::MessageId,
                            TaggedMessage::SourcePlatform,
                            TaggedMessage::Tag,
                        ])
                        .do_nothing()
                        .to_owned(),
                    )
                    .values_panic([
                        "1598980368920678400".into(),
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
            .drop_table(Table::drop().table(TaggedMessage::Table).to_owned())
            .await
    }
}

/// Learn more at https://docs.rs/sea-query#iden
#[derive(Iden)]
enum TaggedMessage {
    Table,
    MessageId,
    SourcePlatform,
    Tag,
    CreatedAt,
    UpdatedAt,
}
