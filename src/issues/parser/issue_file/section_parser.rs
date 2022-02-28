use pulldown_cmark;

use crate::issues::issue;
use crate::logging::logger;
use crate::phases::prepare_project_lazy_values;

pub trait SectionParser<'input> {
    fn process(&mut self, logger: &logger::Logger, issue: &mut issue::Issue, event: pulldown_cmark::Event<'input>);

    fn save_on(
        self:                Box<Self>,
        logger:              &logger::Logger,
        project_lazy_values: &prepare_project_lazy_values::ProjectLazyValues,
        issue:               &mut issue::Issue<'input>
    );
}
