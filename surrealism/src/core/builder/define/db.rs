//! # Define DB
//! ```txt
//! @author:syf20020816@Outlook.com
//! @date:2023/10/31
//! @version:0.0.1
//! @description:
//! ```
use std::fmt::{Display, Formatter};

use crate::db::constants::{DEFINE_DB, STMT_END};

#[derive(Debug, Clone)]
pub struct DefineDB<'a> {
    name: &'a str,
}

impl<'a> Default for DefineDB<'a> {
    fn default() -> Self {
        DefineDB {
            name: ""
        }
    }
}

impl<'a> DefineDB<'a> {
    pub fn new(name: &'a str) -> Self {
        DefineDB {
            name
        }
    }
    pub fn name(&mut self, name: &'a str) -> &mut Self {
        self.name = name;
        self
    }
    pub fn build(&self) -> String {
        self.to_string()
    }
}

impl<'a> Display for DefineDB<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {}{}", DEFINE_DB, self.name, STMT_END)
    }
}