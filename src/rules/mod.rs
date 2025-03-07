mod actions;
mod conditions;

#[derive(serde::Deserialize, Debug)]
pub struct Rule {
    condition: conditions::Condition,
    actions: Vec<actions::Action>,
}

impl Rule {
    pub async fn apply(&self, issue: &crate::types::PullRequestOrIssue) -> eyre::Result<()> {
        if conditions::TestCondition::matches(&self.condition, issue) {
            for action in &self.actions {
                actions::ExecuteAction::execute(action, issue).await?;
            }
        }
        Ok(())
    }
}
