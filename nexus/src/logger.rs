use crate::log::{log as nexus_log, LogLevel};
use log::Log;

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

#[derive(Debug)]
pub struct NexusLogger {
    channel_name: &'static str,
}

impl NexusLogger {
    pub fn set_logger(channel_name: &'static str, filter: Option<&'static str>) {
        #[cfg(not(feature = "log_filter"))]
        let logger = Self { channel_name };

        #[cfg(feature = "log_filter")]
        let logger = filter::NexusLoggerFiltered::new(channel_name, filter);

        let _ = log::set_boxed_logger(Box::new(logger));
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

#[cfg(feature = "log_filter")]
mod filter {
    use super::NexusLogger;
    use env_filter::{Builder, Filter};
    use log::Log;

    #[derive(Debug)]
    pub struct NexusLoggerFiltered {
        logger: NexusLogger,
        filter: Filter,
    }

    impl NexusLoggerFiltered {
        pub fn new(channel_name: &'static str, filter: Option<&'static str>) -> Self {
            Self {
                logger: NexusLogger { channel_name },
                filter: filter
                    .map(|f| Builder::new().parse(f).build())
                    .unwrap_or_else(|| {
                        Builder::new().filter_level(log::LevelFilter::Trace).build()
                    }),
            }
        }
    }

    impl Log for NexusLoggerFiltered {
        fn enabled(&self, metadata: &log::Metadata) -> bool {
            self.filter.enabled(metadata)
        }

        fn log(&self, record: &log::Record) {
            if self.filter.matches(record) {
                self.logger.log(record);
            }
        }

        fn flush(&self) {}
    }
}
