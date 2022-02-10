#[derive(Debug)]
pub struct LogEntry {
    pub message: String
}

impl Clone for LogEntry {
    fn clone(&self) -> LogEntry {
        LogEntry::new(self.message.clone())
    }
}

impl LogEntry {
    pub fn new(message: String) -> LogEntry {
        LogEntry { message }
    }
}
