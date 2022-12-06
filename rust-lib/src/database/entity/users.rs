//! `SeaORM` Entity. Generated by sea-orm-codegen 0.10.5

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "users")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: String,
    #[sea_orm(primary_key, auto_increment = false)]
    pub source_platform: String,
    pub name: String,
    pub display_name: Option<String>,
    pub created_at: DateTimeWithTimeZone,
    pub updated_at: DateTimeWithTimeZone,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::source_platform::Entity",
        from = "Column::SourcePlatform",
        to = "super::source_platform::Column::Name",
        on_update = "Cascade",
        on_delete = "NoAction"
    )]
    SourcePlatform,
    #[sea_orm(has_many = "super::tagged_user::Entity")]
    TaggedUser,
}

impl Related<super::source_platform::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::SourcePlatform.def()
    }
}

impl Related<super::tagged_user::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::TaggedUser.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
