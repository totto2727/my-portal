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
                    .table(PortalPlatform::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(PortalPlatform::Name).string().primary_key())
                    .to_owned(),
            )
            .await?;

        let mut insert_many = Query::insert()
            .into_table(PortalPlatform::Table)
            .columns([PortalPlatform::Name])
            .on_conflict(
                OnConflict::column(PortalPlatform::Name)
                    .do_nothing()
                    .to_owned(),
            )
            .to_owned();

        for platform in portal::PortalPlatform::iter() {
            insert_many.values_panic([platform.to_string().into()]);
        }

        manager.exec_stmt(insert_many).await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(PortalPlatform::Table).to_owned())
            .await
    }
}

/// Learn more at https://docs.rs/sea-query#iden
#[derive(Iden)]
pub enum PortalPlatform {
    Table,
    Name,
}
