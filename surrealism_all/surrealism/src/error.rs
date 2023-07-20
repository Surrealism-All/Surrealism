//! Surrealism Error
//! ```txt
//! @author:syf20020816@Outlook.com
//! @date:2023/7/20
//! @version:0.0.1
//! @description:
//! ```

use std::error::Error;
use std::fmt::{Debug, Display, Formatter};
use std::line;

struct SurrealismError;

#[derive(Debug)]
pub struct ConfigDirNotFoundError {
    line: u32,
    level: ErrorLevel,
    msg: String,
    recommend: String,
}

impl ConfigDirNotFoundError {
    pub fn new(line: u32, level: ErrorLevel) -> &mut ConfigDirNotFoundError {
        &mut ConfigDirNotFoundError {
            line,
            level,
            msg: "".to_string(),
            recommend: "".to_string(),
        }
    }
    pub fn set_msg(&mut self, msg: &str) -> &mut ConfigDirNotFoundError {
        self.msg = String::from(msg);
        self
    }
    pub fn set_recommend(&mut self, recommend: &str) -> &mut ConfigDirNotFoundError {
        self.recommend = String::from(recommend);
        self
    }
}

impl Display for ConfigDirNotFoundError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.description().fmt(f)
    }
}

impl Error for ConfigDirNotFoundError {
    fn description(&self) -> &str {
        "Config Dir Not Found"
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ErrorLevel {
    Error,
    Warn,
    Debug,
    Info,
    Trace,
}