use std::sync;

use git2;

use crate::base::{lazy_value, phase};
use crate::issues::meta::field_mapping;
use crate::phases::{execute_command, global};

pub struct PrepareProjectLazyValues {
    repository: sync::Arc<git2::Repository>
}

pub struct ProjectLazyValues {
    pub field_mapping: lazy_value::LazyValue<field_mapping::FieldMapping>,
}

impl PrepareProjectLazyValues {
    pub fn new(repository: sync::Arc<git2::Repository>) -> Box<dyn phase::NonTerminalPhaseTrait<global::Global>> {
        Box::new(PrepareProjectLazyValues { repository })
    }
}

impl phase::NonTerminalPhaseTrait<global::Global> for PrepareProjectLazyValues {
    fn name(&self) -> &'static str {
        "PrepareProjectLazyValues"
    }

    fn run(self: Box<Self>, global: &mut global::Global) -> phase::Phase<global::Global> {
        lazy_value::make_lazy(global, |_| async {

        });

        phase::continue_with(execute_command::ExecuteCommand::new(self.repository.clone(), ProjectLazyValues::new()))
    }
}

impl ProjectLazyValues {
    fn new() -> ProjectLazyValues {
        todo!()
    }
}
