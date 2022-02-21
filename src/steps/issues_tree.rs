use git2;

use crate::git;
use crate::phases::global;
use crate::errors::issues_tree;
use crate::structure::directories;

pub fn get_issues_tree<'repository>(global: &global::Global, repository: &'repository git2::Repository)
-> Result<git2::Tree<'repository>, issues_tree::IssuesTreeRetrievalError> {
    let branch = repository
    .find_branch(&global.configuration.get_project_branch(), git2::BranchType::Local)
    .map_err(|_| issues_tree::IssuesTreeRetrievalError::CannotFindProjectBranch)?;

    let commit = git
    ::resolve_branch_to_commit(repository, branch)
    .map_err(|_| issues_tree::IssuesTreeRetrievalError::CannotResolveBranchCommit)?;

    git
    ::get_tree(&repository, commit, &directories::get_issues_directory())
    .map_err(issues_tree::IssuesTreeRetrievalError::TreeListingError)
}
