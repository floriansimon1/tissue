use std::sync;

use git2;
use antidote;

use crate::{errors, steps};
use crate::base::{lazy_value, phase};
use crate::issues::meta::field_mapping;
use crate::phases::{execute_command, global};

pub struct PrepareProjectLazyValues {
    repository: sync::Arc<antidote::RwLock<git2::Repository>>
}

pub struct ProjectLazyValues {
    pub field_mapping: lazy_value::LazyValue<steps::CommandInput, Result<field_mapping::FieldMapping, errors::FieldMappingParsingError>>,
}

impl PrepareProjectLazyValues {
    pub fn new(repository: sync::Arc<antidote::RwLock<git2::Repository>>) -> Box<dyn phase::NonTerminalPhaseTrait<global::Global>> {
        Box::new(PrepareProjectLazyValues { repository })
    }
}

impl phase::NonTerminalPhaseTrait<global::Global> for PrepareProjectLazyValues {
    fn name(&self) -> &'static str {
        "PrepareProjectLazyValues"
    }

    fn run(self: Box<Self>, _: sync::Arc<antidote::RwLock<global::Global>>) -> phase::Phase<global::Global> {
        phase::continue_with(execute_command::ExecuteCommand::new(self.repository, ProjectLazyValues::new()))
    }
}

impl ProjectLazyValues {
    fn new() -> ProjectLazyValues {
        ProjectLazyValues {
            field_mapping: lazy_value::make_lazy(steps::parse_field_mapping),
        }
    }
}