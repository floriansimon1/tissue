use std::collections;

use crate::users;

pub struct FieldMapping {
    fields: collections::BTreeMap<FieldId, FieldDescriptor>,
}

pub enum FieldDescriptor {
    Choice(ChoiceOptions),
    User(users::UserId),
    Checkbox,
}

pub type FieldId = String;
pub type ValueId = String;

type ChoiceOptions = collections::BTreeMap<ValueId, String>;
