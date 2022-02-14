use std::path;

const TISSUE_ROOT:      &'static str = ".tissue";
const ISSUES_DIRECTORY: &'static str = "issues";

pub fn get_issues_directory() -> path::PathBuf {
    path::PathBuf::from(TISSUE_ROOT).join(ISSUES_DIRECTORY)
}
