use tokio::task;
use toml;

use crate::steps;
use crate::errors;
use crate::structure::paths;
use crate::issues::meta::field_mapping;
use crate::steps::retrieve_text_file::retrieve_text_file;

pub async fn parse_field_mapping(input: steps::CommandInput) -> Result<field_mapping::FieldMapping, errors::FieldMappingParsingError> {
    task
    ::spawn(async move {
        let mapping_text = {
            let (global, repository) = input.read();

            global.logger.log_trace(String::from("Started parsing the field mapping"));

            retrieve_text_file(&*global, &*repository, &paths::get_field_mapping_path()).map_err(errors::FieldMappingParsingError::Fetching)?
        };

        task::yield_now().await;

        toml::from_str(&mapping_text).map_err(errors::FieldMappingParsingError::Parsing)
    })
    .await
    .unwrap()
}
