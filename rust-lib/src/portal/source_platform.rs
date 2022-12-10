use serde::{Deserialize, Serialize};
use strum_macros::{EnumIter, IntoStaticStr};

#[derive(
    Debug,
    PartialEq,
    Eq,
    Clone,
    Copy,
    Hash,
    Serialize,
    Deserialize,
    strum_macros::EnumString,
    strum_macros::Display,
    IntoStaticStr,
    EnumIter,
)]
pub enum SourcePlatform {
    #[strum(serialize = "twitter")]
    #[serde(alias = "twitter")]
    Twitter,
}
