use git2;

use crate::git::branch_resolution_error;

pub fn resolve_branch_to_commit<'repository>(repository: &'repository git2::Repository, branch: git2::Branch<'repository>)
-> Result<git2::Commit<'repository>, branch_resolution_error::BranchResolutionError> {
    let object_id = branch
    .get()
    .target()
    .ok_or(branch_resolution_error::BranchResolutionError::ReferenceHasNoObjectId)?;

    repository
    .find_commit(object_id)
    .map_err(|error| match &error.code() {
        git2::ErrorCode::NotFound => branch_resolution_error::BranchResolutionError::CommitNotFound,
        _                         => branch_resolution_error::BranchResolutionError::UnknownError(error)
    })
}
