use std::sync;

use futures::future;

use crate::errors;
use crate::issues::meta::field_mapping;

type Value = sync::Arc<Result<field_mapping::FieldMapping, errors::FieldMappingParsingError>>;

pub type FieldMappingParsingResult = future::Shared<future::BoxFuture<'static, Value>>;
