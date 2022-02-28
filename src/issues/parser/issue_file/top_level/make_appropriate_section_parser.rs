use crate::errors;
use crate::issues::issue;
use crate::logging::logger;
use crate::issues::parser::issue_file::section_parser;

use crate::issues::parser::issue_file::{
    meta_section_parser,
    ignored_section_parser,
    comments_section_parser,
    free_text_section_parser,
};

use crate::issues::parser::issue_file::top_level::top_level_logger::log_top_level_trace;

pub fn make_appropriate_section_parser<'input>(logger: &logger::Logger, mut title: String, issue: &mut issue::Issue)
-> Box<dyn section_parser::SectionParser<'input> + 'input> {
    title.make_ascii_lowercase();

    // prevent double meta
    match title.trim() {
        meta_section_parser::META_SECTION => {
            if issue.meta.is_some() {
                log_top_level_trace(logger, "Ignoring the second meta section found while parsing the issue file");

                issue.push_error(errors::IssueParsingError::SecondMetaSectionFound);

                ignored_section_parser::IgnoredSectionParser::new()
            } else {
                meta_section_parser::MetaSectionParser::new()
            }
        },

        comments_section_parser::COMMENTS_SECTION => comments_section_parser::CommentsSectionParser::new(),
        _                                         => Box::new(free_text_section_parser::FreeTextSectionParser::new_titled(title)),
    }
}
