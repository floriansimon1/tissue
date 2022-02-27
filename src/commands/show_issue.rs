use std::{path, str};

use git2;

use crate::phases::global;
use crate::steps::get_tree;
use crate::structure::paths;
use crate::{commands, errors};
use crate::io::{paging, safe_stdio};

#[derive(Debug)]
pub struct ShowCommand {
    pub issue_name: String,
}

pub fn make_show_command(issue_name: String) -> commands::Command {
    commands::Command::Show(ShowCommand { issue_name })
}

pub fn show_issue<'repository>(global: &global::Global, repository: &'repository git2::Repository, issue_name: &str)
-> Result<(), ()> {
    let issue_text = get_tree
    ::get_project_tree(&global, &repository, path::Path::new("."))
    .or(Err(errors::issue::IssueFetchError::RootDirectoryError))
    .and_then(|issues_tree| {
        issues_tree
        .get_path(&paths::get_issue_file_path(issue_name))
        .or(Err(errors::issue::IssueFetchError::CannotGetTreeEntry))
    })
    .and_then(|tree_entry| {
        tree_entry.to_object(repository).or(Err(errors::issue::IssueFetchError::CannotGetIssueObject))
    })
    .and_then(|object: git2::Object<'repository>| {
        object.into_blob().or(Err(errors::issue::IssueFetchError::CannotReadIssueBlob))
    })
    .and_then(|blob| String::from_utf8(blob.content().to_owned()).or(Err(errors::issue::IssueFetchError::IssueIsInvalidUtf8)))
    .map_err(errors::issue::explain_error)
    .map_err(String::from)
    .map_err(|error| global.logger.log_error(error))?;

    let pager = paging::Pager::new();

    safe_stdio::safe_println(&issue_text);

    pager.wait();

    Ok(())
}
