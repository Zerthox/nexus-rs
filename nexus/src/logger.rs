use crate::log::{log as nexus_log, LogLevel};
use env_filter::{Builder, Filter};
use log::Log;

#[derive(Debug)]
pub struct NexusLogger {
    channel_name: &'static str,
    filter: Filter,
}

impl NexusLogger {
    pub fn set_logger(channel_name: &'static str, filter: Option<Filter>) {
        let _ = log::set_boxed_logger(Box::new(NexusLogger {
            channel_name,
            filter: filter
                .unwrap_or_else(|| Builder::new().filter_level(log::LevelFilter::Trace).build()),
        }));
        log::set_max_level(log::LevelFilter::Trace);
    }
}

impl Log for NexusLogger {
    fn enabled(&self, metadata: &log::Metadata) -> bool {
        self.filter.enabled(metadata)
    }

    fn log(&self, record: &log::Record) {
        if self.filter.matches(record) {
            let message = format!("{}", record.args());
            nexus_log(record.level().into(), self.channel_name, message)
        }
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
