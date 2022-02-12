use std::path;

const DEFAULT_BRANCH_PREFIX: &'static str = "tissue/";
const DEFAULT_PROJECT_NAME:  &'static str = "main";

pub struct Configuration {
    pub repository_root: path::PathBuf,
    pub branch_prefix:   String,
    pub project_name:    String,
}

impl Configuration {
    pub fn default(repository_root: path::PathBuf) -> Configuration {
        Configuration {
            repository_root,

            branch_prefix:   String::from(DEFAULT_BRANCH_PREFIX),
            project_name:    String::from(DEFAULT_PROJECT_NAME),
        }
    }

    pub fn get_project_branch(&self) -> String {
        self.branch_prefix.clone() + &self.project_name
    }
}
