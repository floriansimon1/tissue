#[derive(PartialEq)]
pub enum IssueParsingError {
    NoIssueTitleFound,
    FieldWithoutValue,
    UnexpectedTitleStart,
    UnidentifiedMetaField,
    SecondMetaSectionFound,
    FoundContentOutsideMetaTable,
    AdditionalTopLevelHeadingFound,
}
