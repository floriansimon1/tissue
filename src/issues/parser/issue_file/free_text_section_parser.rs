use matches;

use crate::issues::issue;
use crate::logging::logger;
use crate::phases::prepare_project_lazy_values;
use crate::issues::parser::issue_file::section_parser;

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

impl<'input> section_parser::SectionParser<'input> for FreeTextSectionParser<'input> {
    fn process(&mut self, logger: &logger::Logger, _: &mut issue::Issue, event: pulldown_cmark::Event<'input>) {
        if matches!(self, FreeTextSectionParser::EmptySection) {
            *self = FreeTextSectionParser::new_untitled();

            log_free_text_trace(logger, "Initializing a new untitled section with an event");
        } else {
            log_free_text_trace(logger, "Accumulating a new event");
        }

        if let FreeTextSectionParser::OpenedSection(data) = self {
            data.events.push(event);
        }
    }

    fn save_on(self: Box<Self>, logger: &logger::Logger, _: &prepare_project_lazy_values::ProjectLazyValues, issue: &mut issue::Issue<'input>) {
        match *self {
            FreeTextSectionParser::EmptySection => {
                log_free_text_trace(logger, "Ignoring empty free-text section");
            },

            FreeTextSectionParser::OpenedSection(data) => {
                log_free_text_trace(logger, "Saving a new free-text section in the issue");

                issue.add_free_text_section(data);
            }
        }
    }
}

fn log_free_text_trace(logger: &logger::Logger, message: &'static str) {
    logger.log_trace(format!("Issue parsing - Free text: {message}"));
}
