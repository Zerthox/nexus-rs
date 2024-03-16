use crate::log::{log as nexus_log, LogLevel};
use log::Log;

pub struct NexusLogger;

impl NexusLogger {
    pub fn set_logger() {
        let _ = log::set_boxed_logger(Box::new(NexusLogger));
    }
}

impl Log for NexusLogger {
    fn enabled(&self, _metadata: &log::Metadata) -> bool {
        true
    }

    fn log(&self, record: &log::Record) {
        let message = format!(
            "{}: {}",
            record.level().to_string().to_lowercase(),
            record.args()
        );
        nexus_log(record.level().into(), "file", message)
    }

    fn flush(&self) {}
}

impl From<log::Level> for LogLevel {
    #[inline]
    fn from(level: log::Level) -> Self {
        match level {
            log::Level::Error => Self::Critical,
            log::Level::Warn => Self::Warning,
            log::Level::Info => Self::Info,
            log::Level::Debug => Self::Debug,
            log::Level::Trace => Self::Trace,
        }
    }
}
