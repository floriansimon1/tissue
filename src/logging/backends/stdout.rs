pub struct StdoutBackend;

use crate::logging::entry::LogEntry;
use crate::logging::backends::LoggingBackend;

impl LoggingBackend for StdoutBackend {
    fn log_entry(&self, entry: &LogEntry) {
        println!(">> {}", entry.message);
    }
}
