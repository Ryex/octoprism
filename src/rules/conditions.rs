use enum_dispatch::enum_dispatch;

use crate::types::PullRequestOrIssue;

#[enum_dispatch]
pub trait TestCondition {
    fn matches(&self, issue: &PullRequestOrIssue) -> bool;
}

#[enum_dispatch(TestCondition)]
#[derive(serde::Deserialize, Debug)]
pub enum Condition {
    And,
    Or,
    Not,
    HasLabels,
    IsIssue,
    IsPullRequest,
}

#[derive(serde::Deserialize, Debug)]
pub struct And(Vec<Condition>);

impl TestCondition for And {
    #[inline]
    fn matches(&self, issue: &PullRequestOrIssue) -> bool {
        self.0.iter().all(|cond| cond.matches(issue))
    }
}

#[derive(serde::Deserialize, Debug)]
pub struct Or(Vec<Condition>);

impl TestCondition for Or {
    #[inline]
    fn matches(&self, issue: &PullRequestOrIssue) -> bool {
        self.0.iter().any(|cond| cond.matches(issue))
    }
}

#[derive(serde::Deserialize, Debug)]
pub struct Not(Box<Condition>);

impl TestCondition for Not {
    #[inline]
    fn matches(&self, issue: &PullRequestOrIssue) -> bool {
        !self.0.matches(issue)
    }
}

#[derive(serde::Deserialize, Debug)]
pub struct HasLabels(Vec<String>);

impl TestCondition for HasLabels {
    #[inline]
    fn matches(&self, issue: &PullRequestOrIssue) -> bool {
        self.0.iter().all(|cond_label| {
            issue
                .labels()
                .is_none_or(|labels| labels.iter().any(|label| &label.name == cond_label))
        })
    }
}

#[derive(serde::Deserialize, Debug)]
pub struct IsIssue();

impl TestCondition for IsIssue {
    fn matches(&self, issue: &PullRequestOrIssue) -> bool {
        matches!(issue, PullRequestOrIssue::Issue(_))
    }
}

#[derive(serde::Deserialize, Debug)]
pub struct IsPullRequest();

impl TestCondition for IsPullRequest {
    fn matches(&self, issue: &PullRequestOrIssue) -> bool {
        matches!(issue, PullRequestOrIssue::PullRequest(_))
    }
}
