use rust_lib::portal;
use sea_orm_migration::prelude::*;

use crate::m20221203_202947_create_source_platform::SourcePlatform;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Users::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(Users::Id).string())
                    .col(ColumnDef::new(Users::SourcePlatform).string())
                    .col(ColumnDef::new(Users::Name).string().not_null())
                    .col(ColumnDef::new(Users::DisplayName).string())
                    .col(
                        ColumnDef::new(Users::CreatedAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .default(Expr::current_timestamp())
                    )
                    .col(
                        ColumnDef::new(Users::UpdatedAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .primary_key(Index::create().col(Users::Id).col(Users::SourcePlatform))
                    .foreign_key(
                        ForeignKeyCreateStatement::new()
                            .from(Users::Table, Users::SourcePlatform)
                            .to(SourcePlatform::Table, SourcePlatform::Name)
                            .on_update(ForeignKeyAction::Cascade)
                            .on_delete(ForeignKeyAction::NoAction),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .exec_stmt(
                Query::insert()
                    .into_table(Users::Table)
                    .columns([
                        Users::Id,
                        Users::SourcePlatform,
                        Users::Name,
                        Users::DisplayName,
                    ])
                    .on_conflict(
                        OnConflict::columns([Users::Id, Users::SourcePlatform])
                            .do_nothing()
                            .to_owned(),
                    )
                    .values_panic([
                        "1222832820906680320".into(),
                        portal::SourcePlatform::Twitter.to_string().into(),
                        "Lapriere_info".into(),
                        "La prière".into(),
                    ])
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Users::Table).to_owned())
            .await
    }
}

/// Learn more at https://docs.rs/sea-query#iden
#[derive(Iden)]
pub enum Users {
    Table,
    Id,
    SourcePlatform,
    Name,
    DisplayName,
    CreatedAt,
    UpdatedAt,
}
