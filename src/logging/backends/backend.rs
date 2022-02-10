use crate::logging::entry::LogEntry;

pub trait LoggingBackend : Send {
    fn log_entry(&self, entry: &LogEntry);
}
