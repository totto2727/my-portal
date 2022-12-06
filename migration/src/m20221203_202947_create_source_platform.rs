use rust_lib::portal;
use sea_orm_migration::prelude::*;
use strum::IntoEnumIterator;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(SourcePlatform::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(SourcePlatform::Name).string().primary_key())
                    .to_owned(),
            )
            .await?;

        let mut insert_many = Query::insert()
            .into_table(SourcePlatform::Table)
            .columns([SourcePlatform::Name])
            .on_conflict(
                OnConflict::column(SourcePlatform::Name)
                    .do_nothing()
                    .to_owned(),
            )
            .to_owned();

        for platform in portal::SourcePlatform::iter() {
            insert_many.values_panic([platform.to_string().into()]);
        }

        manager.exec_stmt(insert_many).await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(SourcePlatform::Table).to_owned())
            .await
    }
}

/// Learn more at https://docs.rs/sea-query#iden
#[derive(Iden)]
pub enum SourcePlatform {
    Table,
    Name,
}
