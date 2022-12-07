use serde::{Deserialize, Serialize};
use derive_getters::Getters;

#[derive(Debug, Clone, Getters, Serialize, Deserialize)]
pub struct User {
    id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
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

impl From<twitter_v2::User> for User {
    fn from(value: twitter_v2::User) -> Self {
        User {
            id: value.id.to_string(),
            name: value.username,
            display_name: Some(value.name),
        }
    }
}
