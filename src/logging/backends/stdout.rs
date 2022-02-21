use std::io;

use crate::logging;
use crate::logging::entry::LogEntry;
use crate::logging::backends::LoggingBackend;

pub struct StdoutBackend;

impl LoggingBackend for StdoutBackend {
    fn log_entry(&self, entry: &LogEntry) {
        logging::safe_println(&format!(">> {}", entry.message));
    }

    fn try_flush(&self) {
        use io::Write;

        let _ = io::stdout().flush();
    }
}
