use serde::{Deserialize, Serialize};
use strum_macros::EnumString;

#[derive(Debug, EnumString, Serialize, Deserialize)]
pub enum SourcePlatform {
    #[strum(serialize = "twitter")]
    Twitter,
}
