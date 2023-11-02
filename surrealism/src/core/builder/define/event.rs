//! # Define Event
//! ```txt
//! @author:syf20020816@Outlook.com
//! @date:2023/10/31
//! @version:0.0.1
//! @description:
//! ```
use std::fmt::{Display, Formatter};
use super::{OnType};
use crate::db::{Condition};
use crate::db::constants::{DEFINE_EVENT, ON_TABLE, STMT_END, THEN, WHEN};

#[derive(Debug, Clone)]
pub struct DefineEvent<'a> {
    name: &'a str,
    on: OnType<'a>,
    when: Option<Condition>,
    then: &'a str,
}

impl<'a> Default for DefineEvent<'a> {
    fn default() -> Self {
        DefineEvent {
            name: "",
            on: OnType::TABLE(""),
            when: None,
            then: "",
        }
    }
}

impl<'a> DefineEvent<'a> {
    pub fn new(
        name: &'a str,
        on: &'a str,
        when: Option<Condition>,
        then: &'a str,
    ) -> Self {
        DefineEvent {
            name,
            on: OnType::TABLE(on),
            when,
            then,
        }
    }
    pub fn name(&mut self, name: &'a str) -> &mut Self {
        self.name = name;
        self
    }
    pub fn on(&mut self, table: &'a str) -> &mut Self {
        self.on = OnType::TABLE(table);
        self
    }
    pub fn when(&mut self, condition: Condition) -> &mut Self {
        self.when.replace(condition);
        self
    }
    pub fn then(&mut self, expression: &'a str) -> &mut Self {
        self.then = expression;
        self
    }
    pub fn build(&self) -> String { self.to_string() }
}

impl<'a> Display for DefineEvent<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {} {} {}", DEFINE_EVENT, self.name, ON_TABLE, self.on.to_string());
        if let Some(condition) = self.when.as_ref() {
            write!(f, " {} {}", WHEN, condition.to_string());
        }
        write!(f, " {} {}{}", THEN, self.then, STMT_END)
    }
}