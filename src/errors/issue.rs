pub enum IssueFetchError {
    RootDirectoryError,
    IssueIsInvalidUtf8,
    CannotGetTreeEntry,
    CannotReadIssueBlob,
    CannotGetIssueObject,
}

pub fn explain_error(error: IssueFetchError) -> &'static str {
    match error {
        IssueFetchError::CannotReadIssueBlob  => "Cannot read the issue data!",
        IssueFetchError::IssueIsInvalidUtf8   => "The issue is not valid UTF-8!",
        IssueFetchError::CannotGetTreeEntry   => "The requested issue was not found!",
        IssueFetchError::RootDirectoryError   => "Cannot retrieve the root directory!",
        IssueFetchError::CannotGetIssueObject => "Cannot transform the issue tree entry to a Git object!",
    }
}
