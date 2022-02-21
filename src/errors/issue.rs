use crate::errors::issues_tree;

pub enum IssueFetchError {
    IssueIsInvalidUtf8,
    CannotGetTreeEntry,
    CannotReadIssueBlob,
    CannotGetIssueObject,
    IssuesDirectoryError(issues_tree::IssuesTreeRetrievalError)
}

pub fn explain_error(error: IssueFetchError) -> &'static str {
    match error {
        IssueFetchError::CannotReadIssueBlob                     => "Cannot read the issue data!",
        IssueFetchError::IssueIsInvalidUtf8                      => "The issue is not valid UTF-8!",
        IssueFetchError::CannotGetTreeEntry                      => "The requested issue was not found!",
        IssueFetchError::CannotGetIssueObject                    => "Cannot transform the issue tree entry to a Git object!",
        IssueFetchError::IssuesDirectoryError(issues_tree_error) => issues_tree::explain_error(issues_tree_error),
    }
}
