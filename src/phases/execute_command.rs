use git2;

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
        phase::Phase::TerminalSuccess
    }
}
