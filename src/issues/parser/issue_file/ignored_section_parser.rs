use std::sync;

use antidote;
use async_trait;

use crate::issues::issue;
use crate::phases::global;
use crate::issues::parser::issue_file::section_parser;

use crate::issues::parser::issue_file::field_mapping_parsing_result::FieldMappingParsingResult;

pub struct IgnoredSectionParser;

impl IgnoredSectionParser {
    pub fn new() -> Box<IgnoredSectionParser> {
        Box::new(IgnoredSectionParser {})
    }
}

#[async_trait::async_trait]
impl section_parser::SectionParser<'_> for IgnoredSectionParser {
    fn process<'input>(&mut self, _: sync::Arc<antidote::RwLock<global::Global>>, _: &mut issue::Issue, _: pulldown_cmark::Event<'input>)
    {}

    async fn save_on(self: Box<Self>, _: sync::Arc<antidote::RwLock<global::Global>>, _: FieldMappingParsingResult, _: &mut issue::Issue)
    {}
}
