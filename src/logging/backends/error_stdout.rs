use crate::io::safe_stdio;
use crate::logging::{backends, entry};

pub struct ErrorStdoutBackend;

impl backends::LoggingBackend for ErrorStdoutBackend {
    fn log_entry(&self, entry: &entry::LogEntry) {
        use colored::Colorize;

        if entry.level < entry::Level::Error {
            return;
        }

        safe_stdio::safe_println(&format!("{} {}", "✘".red(), entry.message));
    }

    fn try_flush(&self) {
        safe_stdio::safe_flush_stdout();
    }
}
