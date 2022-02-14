use crate::logging::entry::LogEntry;

pub trait LoggingBackend : Send {
    fn try_flush(&self);
    fn log_entry(&self, entry: &LogEntry);
}
