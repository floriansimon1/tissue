use std::sync;

use futures::executor;
use antidote;

use crate::issues::parser::issue_file::meta_section_parser;
use crate::issues::meta::field_mapping;
use crate::phases::global;
use crate::issues::issue;
use crate::errors;

use crate::issues::parser::issue_file::meta_section_parser::meta_section_parser_trace_logger::log_meta_trace;
use crate::issues::parser::issue_file::field_mapping_parsing_result::FieldMappingParsingResult;

impl<'input> meta_section_parser::MetaSectionParser<'input> {
    pub fn do_save_on(self: Box<Self>, global: sync::Arc<antidote::RwLock<global::Global>>, field_mapping: FieldMappingParsingResult, issue: &mut issue::Issue) {
        if self.has_ignored_data_outside_of_table {
            log_meta_trace(&global.read().logger, "Found content outside of the meta table");

            issue.push_error(errors::IssueParsingError::FoundContentOutsideMetaTable);
        }

        let empty_mapping  = field_mapping::FieldMapping::default();
        let result         = executor::block_on(field_mapping);

        let mapping = (*result).as_ref().unwrap_or(&empty_mapping);

        for raw_field in &self.fields {
            let value = match &raw_field.value {
                meta_section_parser::meta_section_parser::RawFieldCell::Link(cell_id, _) => cell_id,
                meta_section_parser::meta_section_parser::RawFieldCell::Text(text)       => text,
            };

            match &raw_field.field {
                meta_section_parser::meta_section_parser::RawFieldCell::Link(_, text) => {
                    issue.try_add_mapped_meta(mapping, text, value);
                }

                meta_section_parser::meta_section_parser::RawFieldCell::Text(text) => {
                    issue.add_free_form_meta(text, value);
                },
            }
        }
    }
}
