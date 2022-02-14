use git2;

use crate::phases::global;
use crate::{git, logging};
use crate::io::{decorate, paging};
use crate::structure::directories::get_issues_directory;

pub fn list_issues(global: &global::Global, repository: &git2::Repository) -> Result<(), ()> {
    let branch = repository
    .find_branch(&global.configuration.get_project_branch(), git2::BranchType::Local)
    .map_err(|error| global.logger.log_error(format!("An error occurred while looking for project branch: `{error:?}`")))?;

    let commit = git
    ::resolve_branch_to_commit(repository, branch)
    .map_err(|error| global.logger.log_error(format!("An error occurred while resolving the project branch to a commit: `{error}`")))?;

    let empty = git
    ::get_tree(&repository, commit, &get_issues_directory())
    .map(|tree| {
        let mut pager = paging::Pager::new();

        pager.page_lines(&global.logger, (
            git
            ::list_files(&tree)
            .map(|file| {
                let file_name = file.name().unwrap_or("<Invalid file>");
                let title     = "<Unknown title>";

                decorate::list_element(&format!("{file_name} - {title}"))
            })
        ));

        pager.wait();

        tree.is_empty()
    })
    .or_else(|error| match error {
        git::FileListingError::CannotGetDirectory         => Ok(true),
        git::FileListingError::CannotGetRoot              => { global.logger.log_error(format!("Cannot find worktree!"));                    Err(()) },
        git::FileListingError::CannotGetDirectoryObject   => { global.logger.log_error(format!("Cannot retrieve worktree object!"));         Err(()) },
        git::FileListingError::ProjectDirectoryIsNotATree => { global.logger.log_error(format!("The issues tree entry is not a directory")); Err(()) },
    })?;

    if empty {
        logging::safe_println(String::from("There is no issue in this project yet!"));
    }

    Ok(())
}
