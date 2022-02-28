use crate::logging::logger;

pub fn log_meta_trace(logger: &logger::Logger, message: &'static str) {
    logger.log_trace(format!("Issue parsing - Meta: {message}"));
}
