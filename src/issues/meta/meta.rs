use std::collections;

use crate::issues::meta::field_mapping;

pub enum ValueAdded {
    Yes,
    No
}

pub struct Meta {
    pub values:         collections::BTreeMap<field_mapping::FieldId, field_mapping::FieldValue>,
    pub free_form_meta: Vec<(field_mapping::FieldId, String)>,
}

impl Meta {
    pub fn new() -> Meta {
        Meta {
            values:         collections::BTreeMap::new(),
            free_form_meta: Vec::new()
        }
    }

    pub fn add_value(&mut self, field_id: &field_mapping::FieldId, value: field_mapping::FieldValue) -> ValueAdded {
        let entry = self.values.entry(String::from(field_id));

        match &entry {
            collections::btree_map::Entry::Vacant(_)   => { entry.or_insert(value); ValueAdded::Yes },
            collections::btree_map::Entry::Occupied(_) => ValueAdded::No,
        }
    }

    pub fn add_free_form_meta(&mut self, field_title: String, text: String) {
        self.free_form_meta.push((String::from(field_title), String::from(text)));
    }
}
