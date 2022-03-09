use std::collections;

use serde;

use crate::{base, users};

#[derive(Default, serde::Deserialize)]
pub struct FieldMapping {
    pub fields: Vec<Field>,
}

impl FieldMapping {
    pub fn get_field(&self, raw_field_id: &str) -> Option<&Field> {
        self.fields.iter().find(|field| field.id == *raw_field_id)
    }
}

impl FieldValuesDescriptor {
    pub fn get_sanitized_value(&self, value: &str) -> Option<FieldValue> {
        match &self {
            FieldValuesDescriptor::User     => todo!(),
            FieldValuesDescriptor::Integer  => value.parse::<i32>().ok().map(FieldValue::Integer),
            FieldValuesDescriptor::Checkbox => base::parse_lax_bool(value).map(FieldValue::Checkbox),

            FieldValuesDescriptor::Choice { values } => {
                values
                .iter()
                .find(|(legit_value_id, _)| legit_value_id.chars().map(|character| character.to_ascii_lowercase()).eq(value.chars()))
                .map(|(legit_value_id, _)| String::from(legit_value_id))
                .map(FieldValue::Choice)
            },
        }
    }
}

#[derive(serde::Deserialize)]
pub struct Field {
    pub id:         String,
    pub name:       String,

    #[serde(flatten)]
    pub descriptor: FieldValuesDescriptor,
}

#[derive(serde::Deserialize)]
#[serde(tag = "type")]
pub enum FieldValuesDescriptor {
    #[serde(alias =     "user")] User,
    #[serde(alias =  "integer")] Integer,
    #[serde(alias = "checkbox")] Checkbox,
    #[serde(alias =   "choice")] Choice { values: ChoiceOptions },
}

pub enum FieldValue {
    User(users::UserId),
    Choice(ValueId),
    Checkbox(bool),
    Integer(i32),
}

pub type FieldId = String;
pub type ValueId = String;

type ChoiceOptions = collections::BTreeMap<ValueId, String>;
