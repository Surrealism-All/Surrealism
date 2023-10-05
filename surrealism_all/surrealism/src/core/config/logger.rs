//! logger service
//! ```txt
//! @author:syf20020816@Outlook.com
//! @date:2023/7/21
//! @version:0.0.1
//! @description:
//! ```

use std::path::PathBuf;
use serde::{Serialize, Deserialize};

/// log struct for configuration
/// - level : log level (Error,Warn,Debug,Info,Trace)
/// - print : true/false (open log or not)
/// - path : the path for logging
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SurrealLogger {
    level: LogLevel,
    print: bool,
    path: PathBuf,
}

impl SurrealLogger {
    pub fn new(level: LogLevel) -> SurrealLogger {
        SurrealLogger {
            level,
            print: true,
            path: PathBuf::new(),
        }
    }
    pub fn get_level(&self) -> &LogLevel {
        &self.level
    }
    ///from SurrealLogger -> SurrealLogger
    pub fn from(logger: SurrealLogger) -> SurrealLogger {
        SurrealLogger {
            level: logger.level,
            print: logger.print,
            path: logger.path,
        }
    }
}

impl Default for SurrealLogger {
    fn default() -> Self {
        SurrealLogger {
            level: LogLevel::Info,
            print: true,
            path: PathBuf::new(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum LogLevel {
    Error,
    Warn,
    Debug,
    Info,
    Trace,
}