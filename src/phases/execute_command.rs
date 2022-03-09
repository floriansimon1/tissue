use std::sync;

use git2;
use antidote;

use crate::commands;
use crate::base::phase;
use crate::phases::{global, prepare_project_lazy_values};

pub struct ExecuteCommand {
    repository:          sync::Arc<antidote::RwLock<git2::Repository>>,
    project_lazy_values: prepare_project_lazy_values::ProjectLazyValues,
}

unsafe impl Send for ExecuteCommand
{}

impl ExecuteCommand {
    pub fn new(repository: sync::Arc<antidote::RwLock<git2::Repository>>, project_lazy_values: prepare_project_lazy_values::ProjectLazyValues)
    -> Box<dyn phase::NonTerminalPhaseTrait<global::Global>> {
        Box::new(ExecuteCommand { repository, project_lazy_values })
    }
}

impl phase::NonTerminalPhaseTrait<global::Global> for ExecuteCommand {
    fn name(&self) -> &'static str {
        "ExecuteCommand"
    }

    fn run(self: Box<Self>, global: sync::Arc<antidote::RwLock<global::Global>>) -> phase::Phase<global::Global> {
        let command = {
            let global = global.read();

            global.logger.log_trace(format!("Executing command `{:?}`", global.command));

            global.command.clone()
        };

        let tokio_runtime = tokio::runtime::Builder::new_multi_thread()
        .worker_threads(2)
        .enable_all()
        .build()
        .unwrap();

        let result = match command {
            commands::Command::Help       => Ok(()),
            commands::Command::List       => commands::list_issues(global, self.repository),
            commands::Command::Show(show) => commands::show_issue(global, self.repository, &show.issue_name),
            commands::Command::Lint(lint) => tokio_runtime.block_on(commands::lint_issue(global, &self.project_lazy_values, self.repository.clone(), lint.issue_name.clone())),
        };

        result
        .map(|_| phase::Phase::TerminalSuccess)
        .unwrap_or(phase::Phase::TerminalError)
    }
}
