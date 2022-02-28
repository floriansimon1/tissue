use crate::issues::issue;
use crate::logging::logger;
use crate::phases::prepare_project_lazy_values;
use crate::issues::parser::issue_file::section_parser;

pub struct IgnoredSectionParser;

impl IgnoredSectionParser {
    pub fn new() -> Box<IgnoredSectionParser> {
        Box::new(IgnoredSectionParser {})
    }
}

impl section_parser::SectionParser<'_> for IgnoredSectionParser {
    fn process<'input>(&mut self, _: &logger::Logger, _: &mut issue::Issue, _: pulldown_cmark::Event<'input>)
    {}

    fn save_on(self: Box<Self>, _: &logger::Logger, _: &prepare_project_lazy_values::ProjectLazyValues, _: &mut issue::Issue)
    {}
}
