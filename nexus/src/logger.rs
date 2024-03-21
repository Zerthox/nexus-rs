use crate::log::{log as nexus_log, LogLevel};
use log::Log;

#[derive(Debug, Clone)]
pub struct NexusLogger {
    channel_name: &'static str,
}

impl NexusLogger {
    pub fn set_logger(channel_name: &'static str) {
        let _ = log::set_boxed_logger(Box::new(NexusLogger { channel_name }));
        log::set_max_level(log::LevelFilter::Trace);
    }
}

impl Log for NexusLogger {
    fn enabled(&self, _metadata: &log::Metadata) -> bool {
        true
    }

    fn log(&self, record: &log::Record) {
        let message = format!("{}", record.args());
        nexus_log(record.level().into(), self.channel_name, message)
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
