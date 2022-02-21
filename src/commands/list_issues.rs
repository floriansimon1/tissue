use std::str;

use git2;

use crate::phases::global;
use crate::issues::parser;
use crate::steps::issues_tree;
use crate::io::{decorate, paging};
use crate::{git, logging, errors};

fn extract_formatted_title<'repository>(repository: &'repository git2::Repository, blob_tree_entry: &git2::TreeEntry<'repository>)
-> Result<String, &'static str> {
    let object = blob_tree_entry
    .to_object(&repository)
    .or(Err("<Invalid Git object>"))?;

    let blob = object
    .as_blob()
    .ok_or("<Invalid Git object type>")?;

    let text = str::from_utf8(blob.content()).or(Err("<Invalid UTF-8>"))?;

    parser::parse_title(text).ok_or("<Untitled>")
}

fn format_entry<'repository>(repository: &'repository git2::Repository, blob_tree_entry: &git2::TreeEntry<'repository>)
-> String {
    let title     = extract_formatted_title(&repository, &blob_tree_entry).unwrap_or_else(String::from);
    let file_name = blob_tree_entry.name().unwrap_or("<Invalid file>");

    decorate::list_element(&format!("{file_name} - {title}"))
}

pub fn list_issues<'repository>(global: &global::Global, repository: &'repository git2::Repository)
-> Result<(), ()> {
    issues_tree
    ::get_issues_tree(&global, &repository)
    .map(|tree| {
        let mut pager = paging::Pager::new();
        let files     = git::list_files(&tree);

        let empty = pager.page_lines(&global.logger, files.map(|blob| format_entry(&repository, &blob)));

        pager.wait();

        if empty {
            logging::safe_println("There is no issue in this project yet!");
        }
    })
    .map_err(errors::issues_tree::explain_error)
    .map_err(String::from)
    .map_err(|error| global.logger.log_error(error))
}

