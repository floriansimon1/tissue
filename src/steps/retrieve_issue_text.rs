use std::{path, str};

use git2;

use crate::errors;
use crate::phases::global;
use crate::steps::get_tree;
use crate::structure::paths;

pub fn retrieve_issue_text<'repository>(global: &global::Global, repository: &'repository git2::Repository, issue_name: &str)
-> Result<String, errors::IssueFetchError> {
    get_tree
    ::get_project_tree(&global, &repository, path::Path::new("."))
    .or(Err(errors::IssueFetchError::RootDirectoryError))
    .and_then(|issues_tree| {
        issues_tree
        .get_path(&paths::get_issue_file_path(issue_name))
        .or(Err(errors::IssueFetchError::CannotGetTreeEntry))
    })
    .and_then(|tree_entry| {
        tree_entry.to_object(repository).or(Err(errors::IssueFetchError::CannotGetIssueObject))
    })
    .and_then(|object: git2::Object<'repository>| {
        object.into_blob().or(Err(errors::IssueFetchError::CannotReadIssueBlob))
    })
    .and_then(|blob| String::from_utf8(blob.content().to_owned()).or(Err(errors::IssueFetchError::IssueIsInvalidUtf8)))
}
