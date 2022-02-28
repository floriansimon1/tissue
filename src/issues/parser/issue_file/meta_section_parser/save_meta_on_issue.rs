use crate::issues::parser::issue_file::meta_section_parser;
use crate::phases::prepare_project_lazy_values;
use crate::logging::logger;
use crate::issues::issue;
use crate::errors;

use crate::issues::parser::issue_file::meta_section_parser::meta_section_parser_trace_logger::log_meta_trace;

impl<'input> meta_section_parser::MetaSectionParser<'input> {
    pub fn do_save_on(self: Box<Self>, logger: &logger::Logger, project_lazy_values: &prepare_project_lazy_values::ProjectLazyValues, issue: &mut issue::Issue) {
        if self.has_ignored_data_outside_of_table {
            log_meta_trace(logger, "Found content outside of the meta table");

            issue.push_error(errors::IssueParsingError::FoundContentOutsideMetaTable);
        }
    }
}
