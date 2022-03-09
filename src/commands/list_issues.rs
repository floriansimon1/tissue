use std::{path, str, sync};

use git2;
use antidote;

use crate::phases::global;
use crate::steps::get_tree;
use crate::structure::paths;
use crate::{errors, git, issues};
use crate::io::{decorate, paging, safe_stdio};

fn extract_formatted_title<'repository>(repository: &'repository git2::Repository, issue_tree_tree_entry: &git2::TreeEntry<'repository>)
-> Result<String, &'static str> {
    let directory_object = issue_tree_tree_entry
    .to_object(&repository)
    .or(Err("Cannot get the Git object of the issue directory"))?;

    let directory = directory_object.as_tree().ok_or("Issue is not a directory")?;

    let issue_file_tree_entry = directory
    .get_path(path::Path::new(paths::ISSUE_FILE))
    .or(Err("Issue file not found"))?;

    let file_object = issue_file_tree_entry
    .to_object(&repository)
    .or(Err("Cannot get the Git object of the issue file"))?;

    let blob = file_object.as_blob().ok_or("Cannot read the issue file")?;

    let text = str
    ::from_utf8(blob.content())
    .or(Err("Invalid UTF-8"))?;

    issues::parse_title(text).ok_or("Untitled")
}

fn format_issue_directory<'repository>(repository: &'repository git2::Repository, issue_tree_tree_entry: &git2::TreeEntry<'repository>)
-> String {
    let title     = extract_formatted_title(&repository, &issue_tree_tree_entry).unwrap_or_else(decorate::decorate_placeholder);
    let file_name = issue_tree_tree_entry.name().map(String::from).unwrap_or_else(|| decorate::decorate_placeholder("Invalid file"));

    decorate::list_element(&format!("{file_name} - {title}"))
}

pub fn list_issues(global: sync::Arc<antidote::RwLock<global::Global>>, repository: sync::Arc<antidote::RwLock<git2::Repository>>) -> Result<(), ()> {
    get_tree
    ::get_issues_tree(&global.read(), &*repository.read())
    .map(|tree| {
        let mut pager   = paging::Pager::new();
        let directories = git::list_directories(&tree);

        let empty = pager.page_lines(&global.read().logger, directories.map(|blob| format_issue_directory(&*repository.read(), &blob)));

        pager.wait();

        if empty {
            safe_stdio::safe_println(&decorate::decorate_success("There is no issue in this project yet!"));
        }
    })
    .map_err(errors::issues_tree::explain_error)
    .map_err(String::from)
    .map_err(|error| global.read().logger.log_error(error))
}
