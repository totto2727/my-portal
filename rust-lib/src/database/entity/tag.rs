//! `SeaORM` Entity. Generated by sea-orm-codegen 0.10.5

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "tag")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub name: String,
    pub created_at: DateTimeWithTimeZone,
    pub updated_at: DateTimeWithTimeZone,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::tagged_channel::Entity")]
    TaggedChannel,
    #[sea_orm(has_many = "super::tagged_message::Entity")]
    TaggedMessage,
    #[sea_orm(has_many = "super::tagged_rule::Entity")]
    TaggedRule,
    #[sea_orm(has_many = "super::tagged_user::Entity")]
    TaggedUser,
}

impl Related<super::tagged_channel::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::TaggedChannel.def()
    }
}

impl Related<super::tagged_message::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::TaggedMessage.def()
    }
}

impl Related<super::tagged_rule::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::TaggedRule.def()
    }
}

impl Related<super::tagged_user::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::TaggedUser.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}