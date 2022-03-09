use std::sync::{atomic, mpsc};
use std::{io, sync, time, thread};

use antidote;

use crate::logging::{backends, entry};

type LoggingBackendsList = Vec<Box<dyn backends::LoggingBackend>>;

pub struct Logger {
    end_signal_sender:   mpsc::SyncSender<()>,
    logging_join_handle: thread::JoinHandle<()>,
    flush_requested:     sync::Arc<atomic::AtomicBool>,
    log_message_sender:  mpsc::SyncSender<entry::LogEntry>,
    logging_backends:    sync::Arc<antidote::Mutex<LoggingBackendsList>>,
}

impl Logger {
    fn log_in_current_thread(logging_backend: &Box<dyn backends::LoggingBackend>, entry: &entry::LogEntry) {
        (*logging_backend).log_entry(&entry);
    }

    #[allow(dead_code)]
    pub fn register_backend(&self, backend: Box<dyn backends::LoggingBackend>) {
        (*self.logging_backends.lock()).push(backend);
    }

    pub fn await_termination(self) {
        if let Err(_) = self.end_signal_sender.send(()) {
            return;
        }

        let _ = self.logging_join_handle.join();
    }

    pub fn log_error(&self, message: String) {
        let _ = self.log_message_sender.send(entry::LogEntry::new(message, entry::Level::Error));
    }

    pub fn log_warning(&self, message: String) {
        let _ = self.log_message_sender.send(entry::LogEntry::new(message, entry::Level::Warning));
    }

    pub fn log_info(&self, message: String) {
        let _ = self.log_message_sender.send(entry::LogEntry::new(message, entry::Level::Info));
    }

    pub fn log_trace(&self, message: String) {
        let _ = self.log_message_sender.send(entry::LogEntry::new(message, entry::Level::Trace));
    }

    pub fn try_flush_all(&self) {
        self.flush_requested.store(true, atomic::Ordering::Relaxed);

        while self.flush_requested.load(atomic::Ordering::Relaxed) {
            thread::sleep(time::Duration::from_millis(50));
        }
    }

    pub fn new() -> io::Result<Logger> {
        let (end_signal_sender,  end_signal_receiver)  = mpsc::sync_channel::<()>(1);
        let (log_message_sender, log_message_receiver) = mpsc::sync_channel::<entry::LogEntry>(50);
        let flush_requested                            = sync::Arc::new(atomic::AtomicBool::new(false));
        let logging_backends                           = sync::Arc::new(antidote::Mutex::new(LoggingBackendsList::new()));

        let flush_requested_copy                       = flush_requested.clone();
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

                let should_flush = flush_requested_copy.load(atomic::Ordering::Relaxed);

                if let Ok(logging_backends) = logging_backend_copy.try_lock() {
                    while let Ok(log_entry) = log_message_receiver.try_recv() {
                        for backend in &*logging_backends {
                            Logger::log_in_current_thread(backend, &log_entry);
                        }
                    }

                    if should_flush {
                        for backend in &*logging_backends {
                            backend.try_flush();
                        }
                    }
                }

                flush_requested_copy.store(false, atomic::Ordering::Relaxed);

                thread::sleep(time::Duration::from_millis(50));
            }
        })
        .and_then(move |logging_join_handle| {
            Ok(Logger {
                logging_join_handle,
                log_message_sender,
                end_signal_sender,
                logging_backends,
                flush_requested,
            })
        })
    }
}
