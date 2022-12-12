use super::rule_part::RulePart;
use crate::{
    domain::{DomainError, Result},
    user::user::User,
};
use derive_getters::Getters;
use rust_lib::portal::SourcePlatform;
use std::{collections::HashSet, fmt::Display};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Getters)]
pub struct RuleText {
    value: String,
}

pub trait RuleTextFactoryTrait {
    fn create(parts: Option<HashSet<RulePart>>, users: Option<HashSet<User>>) -> Result<RuleText>;
}

impl Display for RuleText {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value())
    }
}

impl RuleText {
    pub fn new(source_platform: SourcePlatform, rule: String) -> Result<Self> {
        match source_platform {
            SourcePlatform::Twitter => {
                Self::check_over_length_twitter(rule).map(|r| Self { value: r })
            }
        }
    }

    fn check_over_length_twitter(rule: String) -> Result<String> {
        if rule.len() < 500 {
            Ok(rule)
        } else {
            Err(DomainError::Validation(
                format!(
                    "exceeded the character count for the stream as defined by Twitter:{} > 500",
                    rule.len()
                )
                .into(),
            ))
        }
    }
}
