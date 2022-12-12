use std::fmt::Display;
use derive_getters::Getters;
use rust_lib::database::entity::rule;

#[derive(Debug, Clone, PartialEq, Eq, Ord, PartialOrd, Hash, Getters)]
pub struct RulePart {
    name: String,
    text: String,
}

impl RulePart {
    pub fn new<S: Into<String>>(name: S, text: S) -> Self {
        Self {
            name: name.into(),
            text: text.into(),
        }
    }
}

impl From<rule::Model> for RulePart {
    fn from(model: rule::Model) -> Self {
        Self::new(model.name, model.text)
    }
}
