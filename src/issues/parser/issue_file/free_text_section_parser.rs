use std::sync;

use async_trait;
use antidote;
use matches;

use crate::issues::issue;
use crate::phases::global;
use crate::logging::logger;
use crate::issues::parser::issue_file::section_parser;

use crate::issues::parser::issue_file::field_mapping_parsing_result::FieldMappingParsingResult;

pub enum FreeTextSectionParser<'input> {
    EmptySection,
    OpenedSection(issue::FreeTextSectionData<'input>),
}

impl<'input> FreeTextSectionParser<'input> {
    pub fn new_empty() -> FreeTextSectionParser<'input> {
        FreeTextSectionParser::EmptySection
    }

    pub fn new_titled(title: String) -> FreeTextSectionParser<'input> {
        FreeTextSectionParser::OpenedSection(issue::FreeTextSectionData {
            title:  Some(title),
            events: Vec::new(),
        })
    }

    pub fn new_untitled() -> FreeTextSectionParser<'input> {
        FreeTextSectionParser::OpenedSection(issue::FreeTextSectionData {
            title:  None,
            events: Vec::new(),
        })
    }
}

#[async_trait::async_trait]
impl<'input> section_parser::SectionParser<'input> for FreeTextSectionParser<'input> {
    fn process(&mut self, global: sync::Arc<antidote::RwLock<global::Global>>, _: &mut issue::Issue, event: pulldown_cmark::Event<'input>) {
        if matches!(self, FreeTextSectionParser::EmptySection) {
            *self = FreeTextSectionParser::new_untitled();

            log_free_text_trace(&global.read().logger, "Initializing a new untitled section with an event");
        } else {
            log_free_text_trace(&global.read().logger, "Accumulating a new event");
        }

        if let FreeTextSectionParser::OpenedSection(data) = self {
            data.events.push(event);
        }
    }

    async fn save_on(self: Box<Self>, global: sync::Arc<antidote::RwLock<global::Global>>, _: FieldMappingParsingResult, issue: &mut issue::Issue<'input>) {
        match *self {
            FreeTextSectionParser::EmptySection => {
                log_free_text_trace(&global.read().logger, "Ignoring empty free-text section");
            },

            FreeTextSectionParser::OpenedSection(data) => {
                log_free_text_trace(&global.read().logger, "Saving a new free-text section in the issue");

                issue.add_free_text_section(data);
            }
        }
    }
}

fn log_free_text_trace(logger: &logger::Logger, message: &'static str) {
    logger.log_trace(format!("Issue parsing - Free text: {message}"));
}
