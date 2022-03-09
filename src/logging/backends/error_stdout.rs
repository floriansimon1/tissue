use std::io;

use crate::logging::{backends, entry};

pub struct ErrorStdoutBackend;

impl backends::LoggingBackend for ErrorStdoutBackend {
    fn log_entry(&self, entry: &entry::LogEntry) {
        use colored::Colorize;

        if entry.level < entry::Level::Warning {
            return;
        }

        let symbol = if entry.level == entry::Level::Warning { "⚠".yellow() } else { "✘".red() };

        eprintln!("{symbol} {}", entry.message);
    }

    fn try_flush(&self) {
        use io::Write;

        let _ = io::stderr().flush();
    }
}
