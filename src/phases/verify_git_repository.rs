use std::{convert, path};

use crate::git;
use crate::base::phase;
use crate::phases::global;
use crate::logging::logger;

pub struct VerifyGitRepository;

impl phase::NonTerminalPhaseTrait<global::Global> for VerifyGitRepository {
    fn name(&self) -> &'static str {
        "VerifyGitRepository"
    }

    fn run(self: Box<Self>, global: &mut global::Global) -> phase::Phase<global::Global> {
        open_repository(&global.logger, &global.working_directory_path)
        .and_then(|_| Ok(phase::Phase::TerminalSuccess))
        .unwrap_or_else(convert::identity)
    }
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
