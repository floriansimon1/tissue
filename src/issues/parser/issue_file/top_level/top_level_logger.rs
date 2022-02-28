use crate::logging::logger;

pub fn log_top_level_trace(logger: &logger::Logger, message: &'static str) {
    logger.log_trace(format!("Issue parsing - Top-level: {message}"));
}
