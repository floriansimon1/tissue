use crate::logging::logger::Logger;

pub struct Global {
    pub logger: Logger
}

pub fn instantiate() -> Global {
    match Logger::new() {
        Ok(logger) => Global { logger },
        Err(error) => panic!("Could not create the logging thread: {:?}", error),
    }
}
