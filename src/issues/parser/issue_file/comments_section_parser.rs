use crate::issues::issue;
use crate::logging::logger;
use crate::phases::prepare_project_lazy_values;
use crate::issues::parser::issue_file::section_parser;

pub const COMMENTS_SECTION: &'static str = "comments";

pub struct CommentsSectionParser;

impl CommentsSectionParser {
    pub fn new() -> Box<CommentsSectionParser> {
        Box::new(CommentsSectionParser {})
    }
}

impl section_parser::SectionParser<'_> for CommentsSectionParser {
    #[allow(unused_variables)]
    fn process<'input>(&mut self, logger: &logger::Logger, issue: &mut issue::Issue, event: pulldown_cmark::Event<'input>)
    {}

    #[allow(unused_variables)]
    fn save_on(self: Box<Self>, logger: &logger::Logger, project_lazy_values: &prepare_project_lazy_values::ProjectLazyValues, issue: &mut issue::Issue)
    {}
}
