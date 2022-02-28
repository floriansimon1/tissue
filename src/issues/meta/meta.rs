use std::collections;

use crate::issues::meta::field_mapping;

pub struct Meta {
    pub values: collections::BTreeMap<field_mapping::FieldId, field_mapping::ValueId>,
}

impl Meta {
    pub fn new() -> Meta {
        Meta { values: collections::BTreeMap::new() }
    }
}
