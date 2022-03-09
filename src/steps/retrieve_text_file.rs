use std::path;

use git2;

use crate::errors;
use crate::phases::global;
use crate::steps::get_tree;

pub fn retrieve_text_file<'repository>(global: &global::Global, repository: &'repository git2::Repository, path: &path::Path)
-> Result<String, errors::TextFileFetchError> {
    get_tree
    ::get_project_tree(global, &repository, path::Path::new("."))
    .or(Err(errors::TextFileFetchError::RootDirectoryError))
    .and_then(|root_tree| {
        root_tree
        .get_path(path)
        .or(Err(errors::TextFileFetchError::CannotGetTreeEntry))
    })
    .and_then(|tree_entry| {
        tree_entry.to_object(repository).or(Err(errors::TextFileFetchError::CannotGetIssueObject))
    })
    .and_then(|object: git2::Object<'repository>| {
        object.into_blob().or(Err(errors::TextFileFetchError::CannotReadIssueBlob))
    })
    .and_then(|blob| String::from_utf8(blob.content().to_owned()).or(Err(errors::TextFileFetchError::IssueIsInvalidUtf8)))
}
