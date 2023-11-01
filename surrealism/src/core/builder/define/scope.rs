//! # Define Scope
//! ```txt
//! @author:syf20020816@Outlook.com
//! @date:2023/10/31
//! @version:0.0.1
//! @description:
//! ```
use std::fmt::{Display, Formatter};
use crate::db::constants::{DEFINE_SCOPE, SIGN_IN, SIGN_UP, STMT_END};
use crate::db::TimeOut;

#[derive(Debug, Clone)]
pub struct DefineScope<'a> {
    name: &'a str,
    session: TimeOut,
    sign_up: &'a str,
    sign_in: &'a str,
}

impl<'a> Default for DefineScope<'a> {
    fn default() -> Self {
        DefineScope {
            name: "",
            session: TimeOut::default(),
            sign_up: "",
            sign_in: "",
        }
    }
}

impl<'a> DefineScope<'a> {
    pub fn new(name: &'a str,
               session: TimeOut,
               sign_up: &'a str,
               sign_in: &'a str,
    ) -> Self {
        DefineScope {
            name,
            session,
            sign_up,
            sign_in,
        }
    }
    pub fn session(&mut self, session: TimeOut) -> &mut Self {
        self.session = session;
        self
    }
    pub fn sign_in(&mut self, sign_in: &'a str) -> &mut Self {
        self.sign_in = sign_in;
        self
    }
    pub fn sign_up(&mut self, sign_up: &'a str) -> &mut Self {
        self.sign_up = sign_up;
        self
    }
    pub fn build(&self) -> String {
        self.to_string()
    }
}

impl<'a> Display for DefineScope<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {} {} {} ( {} ) {} ( {} ){}", DEFINE_SCOPE, self.name, &self.session.to_string(), SIGN_UP, self.sign_up, SIGN_IN, self.sign_in, STMT_END)
    }
}