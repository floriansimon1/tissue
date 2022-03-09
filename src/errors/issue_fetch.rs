use crate::errors;

pub fn explain_issue_fetch_error(error: errors::TextFileFetchError) -> &'static str {
    match error {
        errors::TextFileFetchError::CannotReadIssueBlob  => "Cannot read the issue data!",
        errors::TextFileFetchError::IssueIsInvalidUtf8   => "The issue is not valid UTF-8!",
        errors::TextFileFetchError::CannotGetTreeEntry   => "The requested issue was not found!",
        errors::TextFileFetchError::RootDirectoryError   => "Cannot retrieve the root directory!",
        errors::TextFileFetchError::CannotGetIssueObject => "Cannot transform the issue tree entry to a Git object!",
    }
}
