pub enum TextFileFetchError {
    RootDirectoryError,
    IssueIsInvalidUtf8,
    CannotGetTreeEntry,
    CannotReadIssueBlob,
    CannotGetIssueObject,
}

pub fn explain_text_file_fetch_error(error: &TextFileFetchError) -> &'static str {
    match error {
        TextFileFetchError::CannotReadIssueBlob  => "Cannot read file data!",
        TextFileFetchError::CannotGetTreeEntry   => "The requested file was not found!",
        TextFileFetchError::RootDirectoryError   => "Cannot retrieve the root directory!",
        TextFileFetchError::IssueIsInvalidUtf8   => "The requested file is not valid UTF-8!",
        TextFileFetchError::CannotGetIssueObject => "Cannot transform the file tree entry to a Git object!",
    }
}
