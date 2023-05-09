use log::{LevelFilter::{Info, Warn, Debug, Error, Trace}};
use simple_logger::SimpleLogger;

#[warn(dead_code)]
pub fn default_log(log_level: &str)->SimpleLogger {
    let logger = SimpleLogger::new();
    match log_level {
        "error" => logger.with_level(Error),
        "warn" => logger.with_level(Warn),
        "info" => logger.with_level(Info),
        "debug" => logger.with_level(Debug),
        "trace" => logger.with_level(Trace),
        _ => logger.with_level(Info),
    }

}
