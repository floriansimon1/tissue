use crate::io::safe_stdio;
use crate::logging::{backends, entry};

pub struct DebugStdoutBackend;

impl backends::LoggingBackend for DebugStdoutBackend {
    fn log_entry(&self, entry: &entry::LogEntry) {
        safe_stdio::safe_println(&format!(">> {}", entry.message));
    }

    fn try_flush(&self) {
        safe_stdio::safe_flush_stdout();
    }
}
