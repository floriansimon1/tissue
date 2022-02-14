mod entry;
mod safe_print;

pub mod logger;
pub mod backends;

pub use safe_print::safe_print;
pub use safe_print::safe_println;

#[cfg(test)]
mod logger_tests;
