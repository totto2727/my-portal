use rust_lib::portal;
use sea_orm_migration::prelude::*;

use crate::m20221203_203001_create_portal_platform::PortalPlatform;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Channel::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(Channel::Id).string())
                    .col(ColumnDef::new(Channel::PortalPlatform).string())
                    .col(ColumnDef::new(Channel::Name).string().not_null())
                    .primary_key(
                        Index::create()
                            .col(Channel::PortalPlatform)
                            .col(Channel::Id),
                    )
                    .foreign_key(
                        ForeignKeyCreateStatement::new()
                            .from(Channel::Table, Channel::PortalPlatform)
                            .to(PortalPlatform::Table, PortalPlatform::Name)
                            .on_update(ForeignKeyAction::Cascade)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .exec_stmt(
                Query::insert()
                    .into_table(Channel::Table)
                    .columns([Channel::Id, Channel::PortalPlatform, Channel::Name])
                    .on_conflict(
                        OnConflict::columns([Channel::Id, Channel::PortalPlatform])
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
            .drop_table(Table::drop().table(Channel::Table).to_owned())
            .await
    }
}

/// Learn more at https://docs.rs/sea-query#iden
#[derive(Iden)]
pub enum Channel {
    Table,
    Id,
    PortalPlatform,
    Name,
}
