use rust_lib::portal;
use sea_orm_migration::prelude::*;

use crate::{m20221203_185943_create_tag::Tag, m20221205_230421_create_channel::Channel};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(TaggedChannel::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(TaggedChannel::Tag).string().not_null())
                    .col(ColumnDef::new(TaggedChannel::ChannelId).string())
                    .col(ColumnDef::new(TaggedChannel::PortalPlatform).string())
                    .primary_key(
                        Index::create()
                            .col(TaggedChannel::PortalPlatform)
                            .col(TaggedChannel::Tag)
                            .col(TaggedChannel::ChannelId),
                    )
                    .foreign_key(
                        ForeignKeyCreateStatement::new()
                            .from(TaggedChannel::Table, TaggedChannel::Tag)
                            .to(Tag::Table, Tag::Name)
                            .on_update(ForeignKeyAction::Cascade)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKeyCreateStatement::new()
                            .from_tbl(TaggedChannel::Table)
                            .from_col(TaggedChannel::ChannelId)
                            .from_col(TaggedChannel::PortalPlatform)
                            .to_tbl(Channel::Table)
                            .to_col(Channel::Id)
                            .to_col(Channel::PortalPlatform)
                            .on_update(ForeignKeyAction::Cascade)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .exec_stmt(
                Query::insert()
                    .into_table(TaggedChannel::Table)
                    .columns([
                        TaggedChannel::ChannelId,
                        TaggedChannel::PortalPlatform,
                        TaggedChannel::Tag,
                    ])
                    .on_conflict(
                        OnConflict::columns([
                            TaggedChannel::ChannelId,
                            TaggedChannel::PortalPlatform,
                            TaggedChannel::Tag,
                        ])
                        .do_nothing()
                        .to_owned(),
                    )
                    .values_panic([
                        "C03BVNQD6T1".into(),
                        portal::PortalPlatform::Slack.to_string().into(),
                        "La priere".into(),
                    ])
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(TaggedChannel::Table).to_owned())
            .await
    }
}

/// Learn more at https://docs.rs/sea-query#iden
#[derive(Iden)]
enum TaggedChannel {
    Table,
    Tag,
    ChannelId,
    PortalPlatform,
}
