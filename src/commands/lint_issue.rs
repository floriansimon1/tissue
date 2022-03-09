use std::sync;

use git2;
use antidote;

use crate::logging::logger;
use crate::{commands, issues, steps};
use crate::errors::{self, text_file_fetch};
use crate::phases::{global, prepare_project_lazy_values};

#[derive(Debug, Clone)]
pub struct LintCommand {
    pub issue_name: String,
}

pub fn make_lint_command(issue_name: String) -> commands::Command {
    commands::Command::Lint(LintCommand { issue_name })
}

pub async fn lint_issue(
    global:              sync::Arc<antidote::RwLock<global::Global>>,
    project_lazy_values: &prepare_project_lazy_values::ProjectLazyValues,
    repository:          sync::Arc<antidote::RwLock<git2::Repository>>,
    issue_name:          String
) -> Result<(), ()> {
    let field_mapping = project_lazy_values.field_mapping.get((global.clone(), repository.clone()).into());

    let issue_text = steps
    ::retrieve_issue_text(global.clone(), &*repository.read(), &issue_name)
    .map_err(errors::issue_fetch::explain_issue_fetch_error)
    .map_err(String::from)
    .map_err(|error| global.read().logger.log_error(error))?;

    let issue = issues::parse_issue_file(global.clone(), field_mapping.clone(), &issue_text).await;

    if let Err(ref error) = &*field_mapping.await {
        report_field_mapping_errors(&global.read().logger, error);
    }

    issue.report_errors();

    Ok(())
}

fn report_field_mapping_errors(logger: &logger::Logger, error: &errors::FieldMappingParsingError) {
    match error {
        errors::FieldMappingParsingError::Fetching(error) => {
            logger.log_warning(format!("Could not fetch the field mapping: {}", text_file_fetch::explain_text_file_fetch_error(error)));
        },

        errors::FieldMappingParsingError::Parsing(error) => {
            logger.log_error(format!("There is a problem with your field mapping file (parsing crate error: {:?})", error));
        },
    }
}
