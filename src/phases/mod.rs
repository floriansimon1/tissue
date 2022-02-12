mod initial;
mod execute_command;
mod parse_command_line;
mod setup_crash_handler;
mod verify_git_repository;
mod configure_logging_backends;

#[cfg(test)]
mod verify_git_repository_tests;

pub mod global;

pub use initial::make_initial_phase;
