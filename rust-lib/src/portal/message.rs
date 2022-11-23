use serde::{Deserialize, Serialize};
use time::OffsetDateTime;

use crate::portal::author::Author;

#[derive(Debug, Serialize, Deserialize)]
pub struct Message {
    pub tag: Vec<String>,
    pub id: String,
    pub author: Author,
    #[serde(with = "time::serde::rfc3339")]
    pub created_at: OffsetDateTime,
    pub message: String,
}
