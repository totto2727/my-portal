use derive_getters::Getters;
use rust_lib::database::entity::tag;
use rust_lib::database::sea_orm::IntoActiveModel;
use sea_orm::ActiveValue::NotSet;
use sea_orm::ActiveValue::Set;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Getters)]
pub struct Tag {
    name: String,
}

impl Tag {
    fn new(name: impl Into<String>) -> Self {
        Tag { name: name.into() }
    }
}

impl From<String> for Tag {
    fn from(string: String) -> Self {
        Self::new(string)
    }
}

impl From<tag::Model> for Tag {
    fn from(model: tag::Model) -> Self {
        Self::new(model.name)
    }
}

impl IntoActiveModel<tag::ActiveModel> for Tag {
    fn into_active_model(self) -> tag::ActiveModel {
        tag::ActiveModel {
            name: Set(self.name),
            created_at: NotSet,
            updated_at: Set(chrono::Utc::now().into()),
        }
    }
}
