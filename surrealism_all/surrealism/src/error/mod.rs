//! Surrealism Error
//! ```txt
//! @author:syf20020816@Outlook.com
//! @date:2023/7/20
//! @version:0.2.0
//! @description:
//! ```


mod config;
mod msg;

pub use config::{ConfigNotFoundError, ConfigParseError, ConfigError};
pub use msg::*;
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

pub struct ErrorTypeCode;

impl ErrorTypeCode {
    pub const CONFIG_ERROR: u32 = 1000;
    pub const CONFIG_NOT_FOUND_ERROR: u32 = 1001;
    pub const CONFIG_PARSE_ERROR: u32 = 1002;
}

#[macro_export]
macro_rules! err_panic {
    ($ErrMsg:expr,$Code:expr) => {
         error!("{}", $ErrMsg);
         exit($Code);
    };
}
