use std::sync::mpsc;
use std::{io, sync, time, thread};

use antidote;

use crate::logging::entry::LogEntry;
use crate::logging::backends::LoggingBackend;

type LoggingBackendsVector = Vec<Box<dyn LoggingBackend>>;

pub struct Logger {
    end_signal_sender:   mpsc::Sender<()>,
    log_message_sender:  mpsc::Sender<LogEntry>,
    logging_join_handle: thread::JoinHandle<()>,
    logging_backends:    sync::Arc<antidote::Mutex<LoggingBackendsVector>>,
}

impl Logger {
    fn log_in_current_thread(logging_backend: &Box<dyn LoggingBackend>, entry: LogEntry) {
        (*logging_backend).log_entry(&entry);
    }

    pub fn register_backend(&self, backend: Box<dyn LoggingBackend>) {
        (*self.logging_backends.lock()).push(backend);
    }

    pub fn await_termination(self) {
        if let Err(_) = self.end_signal_sender.send(()) {
            return;
        }

        let _ = self.logging_join_handle.join();
    }

    pub fn log_error(&self, message: String) {
        self.log_trace(message);
    }

    pub fn log_info(&self, message: String) {
        self.log_trace(message);
    }

    pub fn log_trace(&self, message: String) {
        let _ = self.log_message_sender.send(LogEntry::new(message));
    }

    pub fn new() -> io::Result<Logger> {
        let (end_signal_sender,  end_signal_receiver)  = mpsc::channel::<()>();
        let (log_message_sender, log_message_receiver) = mpsc::channel::<LogEntry>();
        let logging_backends                           = sync::Arc::new(antidote::Mutex::new(LoggingBackendsVector::new()));

        let logging_backend_copy                       = logging_backends.clone();

        thread
        ::Builder
        ::new()
        .name(String::from("Logging thread"))
        .spawn(move || {
            let mut should_continue = true;

            while should_continue {
                if let Ok(_) = end_signal_receiver.try_recv() {
                    should_continue = false;
                }

                if let Ok(logging_backends) = logging_backend_copy.try_lock() {
                    for logging_backend in &*logging_backends {
                        while let Ok(log_entry) = log_message_receiver.try_recv() {
                            Logger::log_in_current_thread(&logging_backend, log_entry);
                        }
                    }
                }

                thread::sleep(time::Duration::from_millis(50));
            }
        })
        .and_then(move |logging_join_handle| {
            Ok(Logger { logging_join_handle, log_message_sender, end_signal_sender, logging_backends })
        })
    }
}
