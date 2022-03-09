use std::path;

const TISSUE_ROOT:         &'static str = ".tissue";
const ISSUES_DIRECTORY:    &'static str = "issues";
const FIELDS_MAPPING_FILE: &'static str = "fields.toml";

pub const ISSUE_FILE: &'static str = "issue.md";

pub fn get_issues_directory_path() -> path::PathBuf {
    path::PathBuf::from(TISSUE_ROOT).join(ISSUES_DIRECTORY)
}

pub fn get_issue_file_path(issue_name: &str) -> path::PathBuf {
    get_issues_directory_path().join(issue_name).join(ISSUE_FILE)
}

pub fn get_field_mapping_path() -> path::PathBuf {
    path::PathBuf::from(TISSUE_ROOT).join(FIELDS_MAPPING_FILE)
}
