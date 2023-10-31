//! # Define Param
//! ```txt
//! @author:syf20020816@Outlook.com
//! @date:2023/10/31
//! @version:0.0.1
//! @description:
//! ```


use std::fmt::{Display, Formatter};
use crate::db::constants::{DEFINE_PARAM, STMT_END, VALUE};
use crate::db::SurrealValue;

#[derive(Debug, Clone)]
pub struct DefineParam<'a> {
    name: &'a str,
    value: SurrealValue,
}

impl<'a> Default for DefineParam<'a> {
    fn default() -> Self {
        DefineParam {
            name: "",
            value: SurrealValue::None,
        }
    }
}

impl<'a> DefineParam<'a> {
    pub fn new(name: &'a str, value: SurrealValue) -> Self {
        DefineParam {
            name,
            value,
        }
    }
    pub fn name(&mut self, name: &'a str) -> &mut Self {
        self.name = name;
        self
    }
    pub fn value(&mut self, value: SurrealValue) -> &mut Self {
        self.value = value;
        self
    }
    pub fn build(&self) -> String {
        self.to_string()
    }
}

impl<'a> Display for DefineParam<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} ${} {} {}{}", DEFINE_PARAM, name, VALUE, value.to_string(), STMT_END)
    }
}