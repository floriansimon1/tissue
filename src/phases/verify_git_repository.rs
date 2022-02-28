use std::{convert, path, sync};

use crate::git;
use crate::base::phase;
use crate::logging::logger;
use crate::phases::{global, prepare_project_lazy_values};

pub struct VerifyGitRepository;

impl phase::NonTerminalPhaseTrait<global::Global> for VerifyGitRepository {
    fn name(&self) -> &'static str {
        "VerifyGitRepository"
    }

    fn run(self: Box<Self>, global: &mut global::Global) -> phase::Phase<global::Global> {
        open_repository(&global.logger, &global.working_directory_path)
        .and_then(|repository| open_project_branch(&global.logger, repository, &global.configuration.get_project_branch()))
        .map(sync::Arc::new)
        .map(prepare_project_lazy_values::PrepareProjectLazyValues::new)
        .map(phase::continue_with)
        .unwrap_or_else(convert::identity)
    }
}

pub fn open_project_branch(logger: &logger::Logger, repository: git2::Repository, branch_name: &str)
-> Result<git2::Repository, phase::Phase<global::Global>> {
    repository
    .find_branch(branch_name, git2::BranchType::Local)
    .map(|_| (/* Release the branch */))
    .map(move |_| repository)
    .map(|repository| { logger.log_info(format!("Found local branch `{}`", branch_name)); repository })
    .map_err(|error| match error.code() {
        git2::ErrorCode::NotFound => format!("Could not find a local branch named `{}`!", branch_name),
        _                         => format!("An unknown error occurred while looking for a local branch named {}!", branch_name)
    })
    .map_err(|message| logger.log_error(message))
    .map_err(|_| phase::Phase::TerminalError)
}

// Exposed only for tests
pub fn open_repository<Global>(logger: &logger::Logger, working_directory_path: &path::Path)
-> Result<git2::Repository, phase::Phase<Global>>
{
    git
    ::discover_repository(working_directory_path)
    .map(|repository| {
        let path = repository
        .path()
        .parent()
        .expect("Seems like repository discovery does not always return the path of the .git folder!")
        .to_string_lossy();

        logger.log_info(format!("Using repository at `{}`", path));

        repository
    })
    .map_err(|error| {
        logger.log_error(format!("An error occurred while trying to find a Git repository: `{}`", error.message()));

        phase::Phase::TerminalError
    })
}
