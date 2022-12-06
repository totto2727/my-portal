use chrono::{DateTime, Utc};
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
                    .table(Message::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(Message::Id).string())
                    .col(ColumnDef::new(Message::SourcePlatform).string())
                    .col(ColumnDef::new(Message::Text).string().not_null())
                    .col(
                        ColumnDef::new(Message::CreatedAt)
                            .timestamp_with_time_zone()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(Message::UpdatedAt)
                            .timestamp_with_time_zone()
                            .not_null(),
                    )
                    .col(ColumnDef::new(Message::AuthorId).string().not_null())
                    .primary_key(
                        Index::create()
                            .col(Message::Id)
                            .col(Message::SourcePlatform),
                    )
                    .foreign_key(
                        ForeignKeyCreateStatement::new()
                            .from(Message::Table, Message::SourcePlatform)
                            .to(SourcePlatform::Table, SourcePlatform::Name)
                            .on_update(ForeignKeyAction::Cascade)
                            .on_delete(ForeignKeyAction::NoAction),
                    )
                    .foreign_key(
                        ForeignKeyCreateStatement::new()
                            .from(Message::Table, Message::SourcePlatform)
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
                    .into_table(Message::Table)
                    .columns([
                        Message::Id,
                        Message::SourcePlatform,
                        Message::Text,
                        Message::CreatedAt,
                        Message::UpdatedAt,
                        Message::AuthorId,
                    ])
                    .on_conflict(
                        OnConflict::columns([Message::Id, Message::SourcePlatform])
                            .do_nothing()
                            .to_owned(),
                    )
                    .values_panic([
                        "1598980368920678400".into(),
                        portal::SourcePlatform::Twitter.to_string().into(),
                        r#"🌟グッズ通販 事前受付中🌟

追加公演「Three piece!!! ∞」グッズ！
受付は明日までです！お忘れなく👀✨

👇受付はこちら
https://hmv.co.jp/news/article/221117116/
※受付期間：12/4(日)23:59まで

来年1/20(金)より順次お届け予定です！
よろしくお願いいたします✨

#らぷりZepp"#
                            .into(),
                        DateTime::parse_from_rfc3339("2022-12-04T10:27:00+09:00")
                            .unwrap()
                            .with_timezone(&Utc)
                            .into(),
                        DateTime::parse_from_rfc3339("2022-12-04T10:27:00+09:00")
                            .unwrap()
                            .with_timezone(&Utc)
                            .into(),
                        "1222832820906680320".into(),
                    ])
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Message::Table).to_owned())
            .await
    }
}

/// Learn more at https://docs.rs/sea-query#iden
#[derive(Iden)]
pub enum Message {
    Table,
    Id,
    SourcePlatform,
    Text,
    CreatedAt,
    UpdatedAt,
    AuthorId,
}
