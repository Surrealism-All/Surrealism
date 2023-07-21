//! # Config Errors
//! - super : ConfigError
//!
//! 各类配置错误
//! ```txt
//! @author:syf20020816@Outlook.com
//! @date:2023/7/21
//! @version:0.2.0
//! @description:
//! ```

use std::error::Error;
use std::fmt::{Debug, Display, Formatter};
use super::{ErrorLevel, ErrorType};

/// 顶层Config Error
#[derive(Debug, Clone)]
pub struct ConfigError {
    line: u32,
    file: String,
    level: ErrorLevel,
    msg: String,

}

impl ConfigError {
    pub const ERROR_TYPE_ID: ErrorType = ErrorType::ConfigError;
    pub fn from_not_found(e: ConfigNotFoundError) -> ConfigError {
        ConfigError {
            line: e.line,
            file: e.file,
            level: e.level,
            msg: e.print_msg,
        }
    }
    pub fn from_parse(e: ConfigParseError) -> ConfigError {
        ConfigError {
            line: 0_u32,
            file: "".to_string(),
            level: e.level,
            msg: e.msg,
        }
    }
}

impl Display for ConfigError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Display::fmt(&self, f)
    }
}

impl Error for ConfigError {
    fn description(&self) -> &str {
        &self.msg
    }
}

/// # 配置无法找到的错误
/// 当配置文件或配置目录无法找到时触发报错
#[derive(Debug, Clone)]
pub struct ConfigNotFoundError {
    line: u32,
    file: String,
    level: ErrorLevel,
    msg: String,
    recommend: String,
    print_msg: String,
}

impl ConfigNotFoundError {
    pub const ERROR_TYPE_ID: ErrorType = ErrorType::ConfigNotFoundError;
    pub fn new(line: u32, file: &str, level: ErrorLevel) -> ConfigNotFoundError {
        ConfigNotFoundError {
            line,
            file: String::from(file),
            level,
            msg: "".to_string(),
            recommend: "".to_string(),
            print_msg: "".to_string(),
        }
    }
    pub fn set_msg(&mut self, msg: &str) -> &mut ConfigNotFoundError {
        self.msg = String::from(msg);
        self
    }
    pub fn set_recommend(&mut self, recommend: &str) -> &mut ConfigNotFoundError {
        self.recommend = String::from(recommend);
        self
    }
    pub fn print_description(&mut self) -> &mut ConfigNotFoundError {
        self.print_msg = format!("ConfigNotFoundError : \nError : {}\nCheck Line : {} line {}\nHow To Solve : {}", &self.msg, &self.file, &self.line, &self.recommend);
        self
    }
    pub fn deref_mut(&mut self) -> ConfigNotFoundError {
        ConfigNotFoundError {
            line: self.line,
            file: self.file.clone(),
            level: self.level,
            msg: self.msg.clone(),
            recommend: self.recommend.clone(),
            print_msg: self.print_msg.clone(),
        }
    }
    pub fn get_level(&self) -> ErrorLevel {
        self.level
    }
}


impl Display for ConfigNotFoundError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Display::fmt(&self, f)
    }
}

impl Error for ConfigNotFoundError {
    fn description(&self) -> &str {
        &self.print_msg
    }
}

/// # 配置解析错误
/// 在JSON或TOML解析为SurrealismConfig错误时触发
///
/// parse config data to SurrealismConfig
#[derive(Debug)]
pub struct ConfigParseError {
    msg: String,
    level: ErrorLevel,
}

impl ConfigParseError {
    pub const ERROR_TYPE_ID: ErrorType = ErrorType::ConfigParseError;
    pub fn new(msg: &str, level: ErrorLevel) -> ConfigParseError {
        ConfigParseError {
            msg: String::from(msg),
            level,
        }
    }
}

impl Display for ConfigParseError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Display::fmt(&self, f)
    }
}

impl Error for ConfigParseError {
    fn description(&self) -> &str {
        &self.msg
    }
}