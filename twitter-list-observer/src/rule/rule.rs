use super::rule_part::RulePart;
use super::rule_text::{RuleText, RuleTextFactoryTrait};
use super::twitter_rule_text_factory_impl::TwitterRuleTextFactory;
use crate::domain::Result;
use crate::tag::tag::Tag;
use crate::user::user::User;
use derive_getters::Getters;
use rust_lib::portal::SourcePlatform;
use std::collections::HashSet;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Getters)]
pub struct Rule {
    tag: Tag,
    source_platform: SourcePlatform,
    text: RuleText,
}

impl Rule {
    pub fn new(
        source_platform: SourcePlatform,
        tag: Tag,
        parts: Option<HashSet<RulePart>>,
        user: Option<HashSet<User>>,
    ) -> Result<Self> {
        let rule_text: Result<RuleText> = match source_platform {
            SourcePlatform::Twitter => TwitterRuleTextFactory::create(parts, user),
        };
        rule_text.map(|text| Self {
            tag,
            source_platform,
            text,
        })
    }
}
