use std::path;

use crate::logging::logger::Logger;
use crate::configuration::Configuration;

pub struct Global {
    pub working_directory_path: path::PathBuf,
    pub configuration:          Configuration,
    pub logger:                 Logger,
}

pub fn instantiate() -> Global {
    let current_directory_path = path::PathBuf::from(".");

    match Logger::new() {
        Err(error) => panic!("Could not create the logging thread: {:?}", error),

        Ok(logger) => {
            Global {
                logger,

                working_directory_path: current_directory_path.clone(),
                configuration:          Configuration::default(current_directory_path),
            }
        },
    }
}
