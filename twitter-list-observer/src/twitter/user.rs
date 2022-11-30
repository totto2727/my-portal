use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_display_name: Option<String>,
    pub user_name: String,
}

impl From<twitter_v2::User> for User {
    fn from(value: twitter_v2::User) -> Self {
        User {
            id: value.id.to_string(),
            user_name: value.username,
            user_display_name: Some(value.name),
        }
    }
}
