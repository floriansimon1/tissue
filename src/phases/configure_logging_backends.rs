use colored;

use crate::base::phase;
use crate::logging::{self, logger};
use crate::phases::{global, verify_git_repository};

pub struct ConfigureLoggingBackends;

#[allow(dead_code)]
pub fn register_debug_backends(logger: &logger::Logger) {
    logger.register_backend(Box::new(logging::backends::DebugStdoutBackend));
}

#[allow(dead_code)]
pub fn register_release_backends(logger: &logger::Logger) {
    logger.register_backend(Box::new(logging::backends::ErrorStdoutBackend));
}

impl phase::NonTerminalPhaseTrait<global::Global> for ConfigureLoggingBackends {
    fn name(&self) -> &'static str {
        "ConfigureLoggingBackends"
    }

    #[allow(unused_variables)]
    fn run(self: Box<Self>, global: &mut global::Global) -> phase::Phase<global::Global> {
        /*
        * We might be using pagers later on, in which case isatty might give different results.
        * We configure the color library to do what we want to prevent it from lazily detecting
        * if we're a TTY or not and giving us incorrect results.
        */
        if global.terminal_is_tty {
            colored::control::set_override(true);
        }

        #[cfg(debug_assertions)] {
            register_debug_backends(&global.logger);
        }

        #[cfg(not(debug_assertions))] {
            register_release_backends(&global.logger);
        }

        phase::continue_with(Box::new(verify_git_repository::VerifyGitRepository))
    }
}
