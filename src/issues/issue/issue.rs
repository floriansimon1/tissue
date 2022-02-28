use crate::errors;
use crate::issues::meta;
use crate::issues::issue;

pub struct Issue<'input> {
    pub title:          Option<String>,
    pub meta:           Option<meta::Meta>,

    errors:             Vec<errors::IssueParsingError>,
    free_text_sections: Vec<issue::FreeTextSectionData<'input>>,
}

impl<'input> Issue<'input> {
    pub fn create_empty() -> Issue<'input> {
        Issue::<'input> {
            free_text_sections: Vec::new(),
            errors:             Vec::new(),
            title:              None,
            meta:               None,
        }
    }

    pub fn push_error(&mut self, error: errors::IssueParsingError) {
        self.errors.push(error);
    }

    pub fn add_free_text_section(&mut self, section: issue::FreeTextSectionData<'input>) {
        self.free_text_sections.push(section);
    }
}
