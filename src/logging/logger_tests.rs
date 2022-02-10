use antidote;
use std::sync;

use crate::logging::logger::Logger;
use crate::logging::entry::LogEntry;
use crate::logging::backends::LoggingBackend;

struct TestLoggingBackend {
    pub messages: sync::Arc<antidote::Mutex<Vec<LogEntry>>>
}

impl LoggingBackend for TestLoggingBackend {
    fn log_entry(&self, log_entry: &LogEntry) {
        self
        .messages
        .lock()
        .push(log_entry.clone());
    }
}

#[test]
pub fn test_logs_flushed_on_termination() {
    let messages         = sync::Arc::new(antidote::Mutex::new(Vec::<LogEntry>::new()));
    let logger           = Logger::new().expect("Could not create a logger!");
    let expected_message = String::from("Test");

    logger.register_backend(Box::new(TestLoggingBackend { messages: messages.clone() }));

    logger.log_trace(expected_message.clone());

    logger.await_termination();

    let messages = messages.lock();

    assert_eq!(messages.len(), 1);

    assert_eq!(messages[0].message, expected_message);
}

#[test]
pub fn test_message_queuing_before_backend_configured() {
    let messages          = sync::Arc::new(antidote::Mutex::new(Vec::<LogEntry>::new()));
    let logger            = Logger::new().expect("Could not create a logger!");
    let expected_quantity = 5;

    for _ in 1 ..= expected_quantity {
        logger.log_trace(String::new().clone());
    }

    logger.register_backend(Box::new(TestLoggingBackend { messages: messages.clone() }));

    logger.await_termination();

    assert_eq!(messages.lock().len(), expected_quantity);
}
