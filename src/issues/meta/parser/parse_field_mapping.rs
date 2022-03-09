use crate::errors;
use crate::phases::global;
use crate::issues::meta::field_mapping;

pub async fn parse_field_mapping(global: &global::Global)
-> Result<field_mapping::FieldMapping, Vec<errors::FieldMappingParsingError>> {
    todo!();
}
