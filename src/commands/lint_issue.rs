use std::sync;

use git2;

use crate::steps;
use crate::issues;
use crate::{commands, errors};
use crate::phases::{global, prepare_project_lazy_values};

#[derive(Debug)]
pub struct LintCommand {
    pub issue_name: String,
}

pub fn make_lint_command(issue_name: String) -> commands::Command {
    commands::Command::Lint(LintCommand { issue_name })
}

pub fn lint_issue(
    global:              &global::Global,
    project_lazy_values: &prepare_project_lazy_values::ProjectLazyValues,
    repository:          sync::Arc<git2::Repository>,
    issue_name:          &str
) -> Result<(), ()> {
    let issue_text = steps
    ::retrieve_issue_text(&global, &repository, issue_name)
    .map_err(errors::issue::explain_error)
    .map_err(String::from)
    .map_err(|error| global.logger.log_error(error))?;

    issues::parse_issue_file(&global.logger, project_lazy_values, &issue_text);

    Ok(())
}
