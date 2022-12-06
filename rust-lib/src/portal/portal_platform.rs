use serde::{Deserialize, Serialize};
use strum_macros::{EnumIter, IntoStaticStr};

#[derive(
    Debug,
    PartialEq,
    Clone,
    Copy,
    Serialize,
    Deserialize,
    strum_macros::EnumString,
    strum_macros::Display,
    IntoStaticStr,
    EnumIter,
)]
pub enum PortalPlatform {
    #[strum(serialize = "slack")]
    #[serde(alias = "slack")]
    Slack,
}
