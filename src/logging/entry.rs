#[derive(Debug)]

// The order of enum values here is important!
#[derive(PartialEq, PartialOrd, Clone, Copy)]
pub enum Level {
    Trace,
    Info,
    Warning,
    Error,
}

pub struct LogEntry {
    pub message: String,
    pub level:   Level
}

impl Clone for LogEntry {
    fn clone(&self) -> LogEntry {
        LogEntry::new(self.message.clone(), self.level)
    }
}

impl LogEntry {
    pub fn new(message: String, level: Level) -> LogEntry {
        LogEntry { message, level }
    }
}
