use std::sync;

use git2;

use crate::commands;
use crate::base::phase;
use crate::phases::{global, prepare_project_lazy_values};

pub struct ExecuteCommand {
    repository:          sync::Arc<git2::Repository>,
    project_lazy_values: prepare_project_lazy_values::ProjectLazyValues,
}

impl ExecuteCommand {
    pub fn new(repository: sync::Arc<git2::Repository>, project_lazy_values: prepare_project_lazy_values::ProjectLazyValues)
    -> Box<dyn phase::NonTerminalPhaseTrait<global::Global>> {
        Box::new(ExecuteCommand { repository, project_lazy_values })
    }
}

impl phase::NonTerminalPhaseTrait<global::Global> for ExecuteCommand {
    fn name(&self) -> &'static str {
        "ExecuteCommand"
    }

    fn run(self: Box<Self>, global: &mut global::Global) -> phase::Phase<global::Global> {
        global.logger.log_trace(format!("Executing command `{:?}`", global.command));

        let result = match &global.command {
            commands::Command::Help       => Ok(()),
            commands::Command::List       => commands::list_issues(&global, self.repository),
            commands::Command::Show(show) => commands::show_issue(&global, self.repository, &show.issue_name),
            commands::Command::Lint(lint) => commands::lint_issue(&global, &self.project_lazy_values, self.repository, &lint.issue_name),
        };

        result
        .map(|_| phase::Phase::TerminalSuccess)
        .unwrap_or(phase::Phase::TerminalError)
    }
}
