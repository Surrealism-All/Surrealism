//! # Define NS
//! ```txt
//! @author:syf20020816@Outlook.com
//! @date:2023/10/31
//! @version:0.0.1
//! @description:
//! ```
use std::fmt::{Display, Formatter};

use crate::db::constants::{DEFINE_NS, STMT_END};

#[derive(Debug, Clone)]
pub struct DefineNS<'a> {
    name: &'a str,
}

impl<'a> Default for DefineNS<'a> {
    fn default() -> Self {
        DefineNS {
            name: ""
        }
    }
}

impl<'a> DefineNS<'a> {
    pub fn new(name: &'a str) -> Self {
        DefineNS {
            name
        }
    }
    pub fn name(&mut self, name: &'a str) -> &mut Self {
        self.name = name;
        self
    }
    pub fn build(&self) -> String {
        DefineNS::to_string()
    }
}

impl<'a> Display for DefineNS<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {}{}", DEFINE_NS, self.name, STMT_END)
    }
}