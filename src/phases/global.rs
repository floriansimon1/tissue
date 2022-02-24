use std::path;

use crate::logging::logger;
use crate::configuration;
use crate::commands;

pub struct Global {
    pub configuration:          configuration::Configuration,
    pub command:                commands::Command,
    pub logger:                 logger::Logger,
    pub working_directory_path: path::PathBuf,
    pub terminal_is_tty:        bool,
}

pub fn instantiate() -> Global {
    let current_directory_path = path::PathBuf::from(".");

    match logger::Logger::new() {
        Err(error) => panic!("Could not create the logging thread: {:?}", error),

        Ok(logger) => {
            Global {
                logger,

                command:                commands::Command::Help,
                working_directory_path: current_directory_path.clone(),
                terminal_is_tty:        unsafe { libc::isatty(libc::STDOUT_FILENO) == 1 },
                configuration:          configuration::Configuration::default(current_directory_path),
            }
        },
    }
}
