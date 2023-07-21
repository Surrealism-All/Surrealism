//! Surrealism Error
//! ```txt
//! @author:syf20020816@Outlook.com
//! @date:2023/7/20
//! @version:0.2.0
//! @description:
//! ```


mod config;

use std::process::exit;
pub use config::{ConfigNotFoundError, ConfigParseError, ConfigError};

use serde::{Serialize, Deserialize};

/// # 错误级别
/// 对应的是日志级别
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Copy)]
pub enum ErrorLevel {
    Error,
    Warn,
    Debug,
    Info,
    Trace,
}

/// # 错误类型枚举
/// range = [1000..]
/// SupperError的个位和十位为0
pub enum ErrorType {
    ConfigError = 1000,
    ConfigNotFoundError = 1001,
    ConfigParseError = 1002,
}

#[macro_export]
macro_rules! err_panic {
    ($ErrMsg:expr,$Code:expr) => {
         error!("{}", $ErrMsg);
         exit($Code);
    };
}
