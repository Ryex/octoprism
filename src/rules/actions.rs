use enum_dispatch::enum_dispatch;

use crate::types::PullRequestOrIssue;

#[enum_dispatch]
pub trait ExecuteAction {
    async fn execute(&self, issue: &PullRequestOrIssue) -> eyre::Result<()>;
}

#[enum_dispatch(ExecuteAction)]
#[derive(serde::Deserialize, Debug)]
pub enum Action {
    Log,
}

#[derive(serde::Deserialize, Debug)]
pub struct Log {}

impl ExecuteAction for Log {
    async fn execute(&self, issue: &PullRequestOrIssue) -> eyre::Result<()> {
        tracing::debug!(
            "Action::Log | Title: {} | Labels: {:?}",
            issue.title().unwrap_or(&String::new()),
            issue
                .labels()
                .unwrap_or(&vec![])
                .iter()
                .map(|l| &l.name)
                .collect::<Vec<_>>()
        );
        println!(
            "Action::Log | Title: {} | Labels: {:?}",
            issue.title().unwrap_or(&String::new()),
            issue
                .labels()
                .unwrap_or(&vec![])
                .iter()
                .map(|l| &l.name)
                .collect::<Vec<_>>()
        );
        Ok(())
    }
}
