//! # Define Token
//! ```txt
//! @author:syf20020816@Outlook.com
//! @date:2023/10/31
//! @version:0.0.1
//! @description:

use std::fmt::{Display, Formatter};
use crate::db::constants::{DEFINE_TOKEN, ON, STMT_END, VALUE};
use super::{OnType, TokenType};

/// DEFINE TOKEN @name ON [ NAMESPACE | DATABASE | SCOPE @scope ] TYPE @type VALUE @value
#[derive(Debug, Clone)]
pub struct DefineToken<'a> {
    name: &'a str,
    on: Option<OnType<'a>>,
    token_type: TokenType,
    value: &'a str,
}

impl<'a> Default for DefineToken<'a> {
    fn default() -> Self {
        DefineToken {
            name: "",
            on: None,
            token_type: TokenType::default(),
            value: "",
        }
    }
}


impl<'a> DefineToken<'a> {
    pub fn new(
        name: &'a str,
        on: Option<OnType<'a>>,
        token_type: TokenType,
        value: &'a str,
    ) -> Self {
        if on.is_some() {
            if !on.as_ref().unwrap().on_token() {
                panic!("DEFINE TOKEN should use OnType::DB | OnType::NS | OnType::SCOPE!")
            }
        }
        DefineToken {
            name,
            on,
            token_type,
            value,
        }
    }
    pub fn name(&mut self, name: &'a str) -> &mut Self {
        self.name = name;
        self
    }
    pub fn on(&mut self, on: OnType<'a>) -> &mut Self {
        if !on.on_token() {
            panic!("DEFINE TOKEN should use OnType::DB | OnType::NS | OnType::SCOPE!")
        }
        self.on.replace(on);
        self
    }
    pub fn token_type(&mut self, token_type: TokenType) -> &mut Self {
        self.token_type = token_type;
        self
    }
    pub fn value(&mut self, value: &'a str) -> &mut Self {
        self.value = value;
        self
    }
    pub fn build(&self) -> String {
        self.to_string()
    }
}

impl<'a> Display for DefineToken<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {} {} ", DEFINE_TOKEN, self.name, ON);
        if let Some(on) = self.on.as_ref() {
            f.write_str(&on.to_string());
        }
        write!(f, " {} {} '{}'{}", self.token_type.to_string(), VALUE, self.value, STMT_END)
    }
}