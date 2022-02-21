use crate::errors::tree_listing;

pub enum IssuesTreeRetrievalError {
    TreeListingError(tree_listing::TreeListingError),
    CannotResolveBranchCommit,
    CannotFindProjectBranch,
}

pub fn explain_error(error: IssuesTreeRetrievalError) -> &'static str {
    let file_listing_error = match error {
        IssuesTreeRetrievalError::CannotFindProjectBranch              => return "Cannot resolve the commit of the issues branch!",
        IssuesTreeRetrievalError::CannotResolveBranchCommit            => return "Cannot find the issues branch!",
        IssuesTreeRetrievalError::TreeListingError(file_listing_error) => file_listing_error,
    };

    match file_listing_error {
        tree_listing::TreeListingError::CannotGetTreeEntry       => "Could not retrieve the issues directory inside the project branch tree!",
        tree_listing::TreeListingError::TreeNotADirectory        => "The issues tree entry is not a directory!",
        tree_listing::TreeListingError::CannotGetDirectoryObject => "Cannot retrieve worktree object!",
        tree_listing::TreeListingError::CannotGetRoot            => "Cannot find worktree!",
    }
}
