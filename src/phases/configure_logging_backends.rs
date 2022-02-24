use colored;

use crate::base::phase;
use crate::phases::{global, verify_git_repository};

#[allow(unused_imports)]
use crate::logging;

pub struct ConfigureLoggingBackends;

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

        #[cfg(debug)]
        {
            global.logger.register_backend(Box::new(logging::backends::StdoutBackend));
        }

        phase::continue_with(Box::new(verify_git_repository::VerifyGitRepository))
    }
}
