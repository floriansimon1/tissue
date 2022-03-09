use toml::de;

use crate::errors;

pub enum FieldMappingParsingError {
    Parsing(de::Error),
    Fetching(errors::TextFileFetchError),
}
