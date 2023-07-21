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
use std::ops::{Deref, DerefMut};
use serde::{Serialize, Deserialize};

struct SurrealismError;

#[derive(Debug, Clone)]
pub struct ConfigDirNotFoundError {
    line: u32,
    file: String,
    level: ErrorLevel,
    msg: String,
    recommend: String,
    print_msg: String,
}

impl ConfigDirNotFoundError {
    pub fn new(line: u32, file: &str, level: ErrorLevel) -> ConfigDirNotFoundError {
        ConfigDirNotFoundError {
            line,
            file: String::from(file),
            level,
            msg: "".to_string(),
            recommend: "".to_string(),
            print_msg: "".to_string(),
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
    pub fn print_description(&mut self) -> &mut ConfigDirNotFoundError {
        self.print_msg = format!("ConfigDirNotFoundError : \nError : {}\nCheck Line : {} line {}\nHow To Solve : {}", &self.msg, &self.file, &self.line, &self.recommend);
        self
    }
    pub fn deref_mut(&mut self) -> ConfigDirNotFoundError {
        ConfigDirNotFoundError {
            line: self.line,
            file: self.file.clone(),
            level: self.level,
            msg: self.msg.clone(),
            recommend: self.recommend.clone(),
            print_msg: self.print_msg.clone(),
        }
    }
    pub fn get_level(&self)->ErrorLevel{
        self.level
    }
}


impl Display for ConfigDirNotFoundError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Display::fmt(&self.description(), f)
    }
}

impl Error for ConfigDirNotFoundError {
    fn description(&self) -> &str {
        &self.print_msg
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Copy)]
pub enum ErrorLevel {
    Error,
    Warn,
    Debug,
    Info,
    Trace,
}