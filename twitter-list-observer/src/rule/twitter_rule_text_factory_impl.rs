use super::{
    rule_part::RulePart,
    rule_text::{RuleText, RuleTextFactoryTrait},
};
use crate::{domain::Result, user::user::User};
use rust_lib::portal::SourcePlatform;
use std::{collections::HashSet, iter};

pub struct TwitterRuleTextFactory {}

impl RuleTextFactoryTrait for TwitterRuleTextFactory {
    fn create(parts: Option<HashSet<RulePart>>, users: Option<HashSet<User>>) -> Result<RuleText> {
        let rule_users = match users {
            Some(users) => {
                let mut users: Vec<String> = Vec::from_iter(users)
                    .into_iter()
                    .map(|p| p.id().to_owned())
                    .collect();
                users.sort_unstable();
                Self::join_or(users)
            }
            None => "".into(),
        };

        let rule_parts = match parts {
            Some(parts) => {
                let mut parts: Vec<String> = Vec::from_iter(parts)
                    .into_iter()
                    .map(|p| p.text().to_owned())
                    .collect();
                parts.sort_unstable();
                Self::join_or(parts)
            }
            None => "".into(),
        };

        let rule_text = Self::join_and(
            vec![rule_users, rule_parts]
                .into_iter()
                .filter(|t| !t.is_empty())
                .collect::<Vec<String>>(),
        );
        RuleText::new(SourcePlatform::Twitter, rule_text)
    }
}

impl TwitterRuleTextFactory {
    pub fn new() -> Self {
        Self {}
    }
    pub fn surround_brackts(rule: String) -> String {
        if rule.is_empty() {
            return rule;
        }
        format!("({})", rule)
    }
    fn iter_surrounded_brackts(
        rules: Vec<String>,
    ) -> iter::Map<std::vec::IntoIter<String>, impl Fn(String) -> String> {
        rules.into_iter().map(Self::surround_brackts)
    }
    pub fn join_or(rules: Vec<String>) -> String {
        match rules.len() {
            0 => "".into(),
            1 => rules.get(0).unwrap().to_owned(),
            _ => Self::iter_surrounded_brackts(rules)
                .collect::<Vec<String>>()
                .join(" OR "),
        }
    }

    pub fn join_and(rules: Vec<String>) -> String {
        match rules.len() {
            0 => "".into(),
            1 => rules.get(0).unwrap().to_owned(),
            _ => Self::iter_surrounded_brackts(rules)
                .collect::<Vec<String>>()
                .join(" "),
        }
    }
    //
    // fn build_filter_user_rule(user: &User) -> String {
    //     format!("from:{}", user.id())
    // }
    //
    // fn build_filter_users_rule(users: &Vec<User>) -> String {
    //     Self::build_or_rule(users.iter().map(Self::build_filter_user_rule).collect())
    // }
}

#[cfg(test)]
mod tests {
    use rust_lib::portal::SourcePlatform;

    use crate::rule::rule_part::RulePart;
    use crate::rule::rule_text::RuleTextFactoryTrait;
    use crate::rule::{
        rule_text::RuleText, twitter_rule_text_factory_impl::TwitterRuleTextFactory,
    };
    use crate::user::user::User;
    use std::collections::HashSet;

    #[test]
    fn test_surround_brackets_not_empty_string() {
        assert_eq!(
            TwitterRuleTextFactory::surround_brackts("a".into()),
            "(a)".to_string()
        )
    }
    #[test]
    fn test_surround_brackets_empty_string() {
        assert_eq!(
            TwitterRuleTextFactory::surround_brackts("".into()),
            "".to_string()
        )
    }

    #[test]
    fn test_join_or() {
        let list = vec!["cat".to_owned(), "dog".to_owned()];
        assert_eq!(TwitterRuleTextFactory::join_or(list), "(cat) OR (dog)");
    }
    #[test]
    fn test_join_or_empty() {
        let list = vec![];
        assert_eq!(TwitterRuleTextFactory::join_or(list), "");
    }

    #[test]
    fn test_join_and() {
        let list = vec!["cat".to_owned(), "dog".to_owned()];
        assert_eq!(TwitterRuleTextFactory::join_and(list), "(cat) (dog)");
    }
    #[test]
    fn test_join_and_empty() {
        let list = vec![];
        assert_eq!(TwitterRuleTextFactory::join_and(list), "");
    }

    #[test]
    fn test_create() {
        let rules = vec![RulePart::new("", "cat"), RulePart::new("", "dog")];
        let rules: HashSet<RulePart> = HashSet::from_iter(rules);
        let users = vec![User::new("1", None, "")];
        let users: HashSet<User> = HashSet::from_iter(users);
        let created = TwitterRuleTextFactory::create(rules.into(), users.into());
        assert!(created.is_ok());
        assert_eq!(
            created.unwrap(),
            RuleText::new(SourcePlatform::Twitter, "(1) ((cat) OR (dog))".into()).unwrap()
        );
    }

    // #[test]
    // fn test_build_filter_user_rule() {
    //     let user = User::new("1234567890", None, "");
    //     let rule = RuleService::build_filter_user_rule(&user);
    //     assert_eq!(rule, "from:1234567890");
    // }
    //
    // #[test]
    // fn test_build_filter_users_rule() {
    //     let user1 = User::new("1234567890", None, "");
    //     let user2 = User::new("0987654321", None, "");
    //     let rule = RuleService::build_filter_users_rule(&vec![user1, user2]);
    //     assert_eq!(rule, "(from:1234567890 OR from:0987654321)");
    // }
}
