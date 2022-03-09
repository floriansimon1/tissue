use std::sync;

use git2;

use crate::errors;
use crate::phases::global;
use crate::structure::paths;
use crate::steps::retrieve_text_file;

pub fn retrieve_issue_text<'repository>(global: sync::Arc<antidote::RwLock<global::Global>>, repository: &'repository git2::Repository, issue_name: &str)
-> Result<String, errors::TextFileFetchError> {
    retrieve_text_file::retrieve_text_file(&global.read(), repository, &paths::get_issue_file_path(issue_name))
}
