use derive_getters::Getters;
use rust_lib::database::entity::users;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Getters)]
pub struct User {
    id: String,
    display_name: Option<String>,
    name: String,
}

impl User {
    pub fn new<S>(id: S, display_name: Option<String>, name: S) -> Self
    where
        S: Into<String>,
    {
        Self {
            id: id.into(),
            display_name,
            name: name.into(),
        }
    }
}

impl From<users::Model> for User {
    fn from(value: users::Model) -> Self {
        User {
            id: value.id.to_string(),
            name: value.name,
            display_name: value.display_name,
        }
    }
}

impl From<twitter_v2::User> for User {
    fn from(value: twitter_v2::User) -> Self {
        User {
            id: value.id.to_string(),
            name: value.username,
            display_name: Some(value.name),
        }
    }
}
