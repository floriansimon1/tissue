use git2;

use crate::commands;
use crate::base::phase;
use crate::phases::global;

pub struct ExecuteCommand {
    repository: git2::Repository,
}

impl ExecuteCommand {
    pub fn new(repository: git2::Repository) -> Box<dyn phase::NonTerminalPhaseTrait<global::Global>> {
        Box::new(ExecuteCommand { repository })
    }
}

impl phase::NonTerminalPhaseTrait<global::Global> for ExecuteCommand {
    fn name(&self) -> &'static str {
        "ExecuteCommand"
    }

    fn run(self: Box<Self>, global: &mut global::Global) -> phase::Phase<global::Global> {
        global.logger.log_trace(format!("Executing command `{:?}`", global.command));

        let result = match global.command {
            commands::Command::Help => Ok(()),
            commands::Command::List => commands::list_issues(&global, &self.repository),
        };

        result
        .map(|_| phase::Phase::TerminalSuccess)
        .unwrap_or(phase::Phase::TerminalError)
    }
}
