use crate::issues::meta::field_mapping;

// TODO: Replace Debug with something better.
#[derive(PartialEq, Debug)]
pub enum IssueParsingError {
    NoIssueTitleFound,
    FieldWithoutValue,
    UnexpectedTitleStart,
    UnidentifiedMetaField,
    SecondMetaSectionFound,
    FoundContentOutsideMetaTable,
    AdditionalTopLevelHeadingFound,
    UnknownMappingMetaField(String, String),
    DuplicateMetaField(field_mapping::FieldId),
    UnknownMappingMetaFieldValue(String, String),
}
