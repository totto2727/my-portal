use crate::domain::model::user::User;

pub struct RuleService {}

pub trait RuleServiceTrait {
    fn build_or_rule(rules: Vec<String>) -> String;
    fn build_and_rule(rules: Vec<String>) -> String;
    fn build_filter_user_rule(user: &User) -> String;
    fn build_filter_users_rule(users: &Vec<User>) -> String;
}

impl RuleServiceTrait for RuleService {
    fn build_or_rule(rules: Vec<String>) -> String {
        format!("({})", rules.join(" OR "))
    }

    fn build_and_rule(rules: Vec<String>) -> String {
        format!("({})", rules.join(" "))
    }

    fn build_filter_user_rule(user: &User) -> String {
        format!("from:{}", user.id())
    }

    fn build_filter_users_rule(users: &Vec<User>) -> String {
        Self::build_or_rule(users.iter().map(Self::build_filter_user_rule).collect())
    }
}

#[cfg(test)]
mod tests {
    use crate::domain::{
        model::user::User,
        service::rule_service::{RuleService, RuleServiceTrait},
    };

    #[test]
    fn test_build_or_rule() {
        assert_eq!(
            RuleService::build_or_rule(vec!["cat".to_owned(), "dog".to_owned()]),
            "(cat OR dog)"
        );
    }

    #[test]
    fn test_build_and_rule() {
        assert_eq!(
            RuleService::build_and_rule(vec!["cat".to_owned(), "dog".to_owned()]),
            "(cat dog)"
        );
    }

    #[test]
    fn test_build_filter_user_rule() {
        let user = User::new("1234567890", None, "");
        let rule = RuleService::build_filter_user_rule(&user);
        assert_eq!(rule, "from:1234567890");
    }

    #[test]
    fn test_build_filter_users_rule() {
        let user1 = User::new("1234567890", None, "");
        let user2 = User::new("0987654321", None, "");
        let rule = RuleService::build_filter_users_rule(&vec![user1, user2]);
        assert_eq!(rule, "(from:1234567890 OR from:0987654321)");
    }
}
