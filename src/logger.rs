use std::io::{stderr, Write};
use std::process::exit;
use std::str::FromStr;
use log::{set_logger, Log, LogLevel, LogLevelFilter, LogMetadata, LogRecord, MaxLogLevelFilter};

macro_rules! print_log {
    ($log_level:expr, $message:expr) => {
        writeln!(stderr(), "{}: {}", $log_level, $message).unwrap();
    };
}

struct Logger {
    filter: MaxLogLevelFilter,
}

impl Log for Logger {
    fn enabled(&self, m: &LogMetadata) -> bool {
        m.level() <= self.filter.get()
    }

    fn log(&self, r: &LogRecord) {
        if self.enabled(r.metadata()) {
            print_log!(r.level(), r.args());
        }
    }
}

pub fn init(log_level: &str) {
    set_logger(|filter| match LogLevelFilter::from_str(log_level) {
        Ok(l) => {
            filter.set(l);
            Box::new(Logger { filter })
        }
        Err(_) => {
            print_log!(LogLevel::Error,
                       format!("Invalid log level \"{}\". Available log levels are off, error, warn, info, debug, and trace.",
                               log_level));
            exit(1)
        }
    }).unwrap();
}
