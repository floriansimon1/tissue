use crate::errors;
use crate::issues::issue;
use crate::io::safe_stdio;
use crate::issues::meta::{self, field_mapping};

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

    pub fn report_errors(&self) {
        for error in &self.errors {
            safe_stdio::safe_println(&format!("{error:?}"));
        }
    }

    pub fn try_add_mapped_meta(&mut self, field_mapping: &field_mapping::FieldMapping, raw_field_id: &str, raw_value: &str) {
        if self.meta.is_none() {
            self.meta = Some(meta::Meta::new());
        }

        let meta = self.meta.as_mut().unwrap();

        let field = field_mapping.get_field(raw_field_id);

        if field.is_none() {
            self.push_error(errors::IssueParsingError::UnknownMappingMetaField(String::from(raw_field_id), String::from(raw_value)));

            return;
        }

        let value = field.unwrap().descriptor.get_sanitized_value(raw_value);

        if value.is_none() {
            self.push_error(errors::IssueParsingError::UnknownMappingMetaFieldValue(String::from(raw_field_id), String::from(raw_value)));

            return;
        }

        let field = *field.as_ref().unwrap();

        if value.is_none() {
            self.push_error(errors::IssueParsingError::DuplicateMetaField(String::from(raw_field_id)));

            return;
        }

        meta.add_value(&field.id, value.unwrap());
    }

    pub fn add_free_form_meta(&mut self, field_title: &str, text: &str) {
        if self.meta.is_none() {
            self.meta = Some(meta::Meta::new());
        }

        let meta = self.meta.as_mut().unwrap();

        meta.add_free_form_meta(String::from(field_title), String::from(text));
    }
}
