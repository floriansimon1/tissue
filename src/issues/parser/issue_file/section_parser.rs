use std::sync;

use async_trait;
use pulldown_cmark;

use crate::issues::issue;
use crate::phases::global;

use crate::issues::parser::issue_file::field_mapping_parsing_result::FieldMappingParsingResult;

#[async_trait::async_trait]
pub trait SectionParser<'input> {
    fn process(&mut self, global: sync::Arc<antidote::RwLock<global::Global>>, issue: &mut issue::Issue, event: pulldown_cmark::Event<'input>);

    async fn save_on(
        self:          Box<Self>,
        global:        sync::Arc<antidote::RwLock<global::Global>>,
        field_mapping: FieldMappingParsingResult,
        issue:         &mut issue::Issue<'input>
    );
}
