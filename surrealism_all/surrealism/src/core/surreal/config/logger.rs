//! logger level for surreal
//! ```txt
//! @author:syf20020816@Outlook.com
//! @date:2023/10/20
//! @version:0.3.0
//! @description:
//! ```

use std::path::PathBuf;
use std::str::FromStr;
use serde::{Serialize, Deserialize};
use crate::error::ConfigParseError;

/// # Surreal Log Level
/// match `surreal start --log log_level`
/// > 默认使用 Info 级别
/// - Full : 完整日志
/// - None : 不打印日志
/// - Error : 错误级别
/// - Warn : 警告级别
/// - Debug : 调试级别
/// - Info : 信息级别
/// - Trace : 跟踪级别
/// ## update change!
/// log struct for configuration :SurrealLogger 废弃⛔
/// - level : log level (Error,Warn,Debug,Info,Trace)
/// - print : true/false (open log or not) ⛔ 0.3.0去除该配置
/// - path : the path for logging ⛔ 0.3.0去除该配置
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum LogLevel {
    Error,
    Warn,
    Debug,
    Info,
    Trace,
    Full,
    None,
}

impl FromStr for LogLevel {
    type Err = ConfigParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "error" => Ok(LogLevel::Error),
            "warn" => Ok(LogLevel::Warn),
            "info" => Ok(LogLevel::Info),
            "debug" => Ok(LogLevel::Debug),
            "trace" => Ok(LogLevel::Trace),
            "full" => Ok(LogLevel::Full),
            "none" => Ok(LogLevel::None),
            _ => Err(
                ConfigParseError::new(
                    "ConfigParseError : Unable to match LogLevel, parsing error, please check the configuration file -> Surrealism.json|.toml",
                    line!(),
                    PathBuf::from_str(file!()).unwrap())
            )
        }
    }
}

impl From<&str> for LogLevel {
    fn from(value: &str) -> Self {
        LogLevel::from_str(value).unwrap()
    }
}

impl From<&LogLevel> for LogLevel {
    fn from(value: &LogLevel) -> Self {
        value.clone()
    }
}

impl Default for LogLevel {
    fn default() -> Self {
        LogLevel::Info
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_log_level() {
        let log = LogLevel::from("Full");
        assert_eq!(log, LogLevel::Full)
    }

    #[test]
    fn test_log_level_default() {
        assert_eq!(LogLevel::Info, LogLevel::default());
    }
}