use serde::{Deserialize, Serialize};

use crate::database::portal;

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    pub id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_display_name: Option<String>,
    pub user_name: String,
}

impl From<portal::User> for User {
    fn from(value: portal::User) -> Self {
        User {
            id: value.id_in_platform,
            user_display_name: value.user_display_name,
            user_name: value.user_name,
        }
    }
}
