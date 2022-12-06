use crate::m20221203_202947_create_source_platform::SourcePlatform;
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
                    .table(Rule::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(Rule::Name).string())
                    .col(ColumnDef::new(Rule::SourcePlatform).string())
                    .col(ColumnDef::new(Rule::Text).string().not_null())
                    .primary_key(Index::create().col(Rule::SourcePlatform).col(Rule::Name))
                    .foreign_key(
                        ForeignKeyCreateStatement::new()
                            .from(Rule::Table, Rule::SourcePlatform)
                            .to(SourcePlatform::Table, SourcePlatform::Name)
                            .on_update(ForeignKeyAction::Cascade)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .exec_stmt(
                Query::insert()
                    .into_table(Rule::Table)
                    .columns([Rule::Name, Rule::SourcePlatform, Rule::Text])
                    .on_conflict(
                        OnConflict::columns([Rule::Name, Rule::SourcePlatform])
                            .do_nothing()
                            .to_owned(),
                    )
                    .values_panic([
                        "Test".into(),
                        portal::SourcePlatform::Twitter.to_string().into(),
                        "".into(),
                    ])
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Rule::Table).to_owned())
            .await
    }
}

/// Learn more at https://docs.rs/sea-query#iden
#[derive(Iden)]
pub enum Rule {
    Table,
    Name,
    SourcePlatform,
    Text,
}
