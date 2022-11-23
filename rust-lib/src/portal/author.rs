use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct TwitterAuther {
    pub id: String,
    pub name: String,
    pub user_name: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Author {
    TwitterAuther(TwitterAuther),
}
