use serde::{Deserialize, Serialize};

use crate::dto;

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    pub id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_display_name: Option<String>,
    pub user_name: String,
}

impl From<dto::portal::User> for User {
    fn from(dto: dto::portal::User) -> Self {
        User {
            id: dto.id_in_platform,
            user_display_name: dto.user_display_name,
            user_name: dto.user_name,
        }
    }
}
