//! # Define Table
//! ```txt
//! @author:syf20020816@Outlook.com
//! @date:2023/10/31
//! @version:0.0.1
//! @description:
//! ```

use super::{Permissions, Schema};
use std::fmt::{Display, Formatter};
use crate::db::constants::{BLANK, DEFINE_TABLE, DROP, STMT_END};
use crate::db::TimeUnit;

#[derive(Debug, Clone)]
pub struct DefineTable<'a> {
    name: &'a str,
    drop: bool,
    changefeed: Option<String>,
    schema: Option<Schema>,
    as_expression: Option<&'a str>,
    permissions: Option<Permissions>,
}

impl<'a> Default for DefineTable<'a> {
    fn default() -> Self {
        DefineTable {
            name: "",
            drop: false,
            changefeed: None,
            schema: None,
            as_expression: None,
            permissions: None,
        }
    }
}

impl<'a> DefineTable<'a> {
    pub fn new() -> Self {
        DefineTable::default()
    }
    pub fn name(&mut self, name: &'a str) -> &mut Self {
        self.name = name;
        self
    }
    pub fn drop(&mut self) -> &mut Self {
        self.drop = true;
        self
    }
    pub fn changefeed(&mut self, duration: u32, unit: TimeUnit) -> &mut Self {
        self.changefeed.replace(format!("{}{}", duration, unit.get_unit()));
        self
    }
    pub fn schemafull(&mut self) -> &mut Self {
        self.schema.replace(Schema::SCHEMAFULL);
        self
    }
    pub fn schemaless(&mut self) -> &mut Self {
        self.schema.replace(Schema::SCHEMALESS);
        self
    }
    pub fn as_expression(&mut self, expression: &'a str) -> &mut Self {
        self.as_expression.replace(expression);
        self
    }
    pub fn permission(&mut self, permission: Permissions) -> &mut Self {
        self.permissions.replace(permission);
        self
    }
    pub fn build(&self) -> String {
        self.to_string()
    }
}

impl<'a> Display for DefineTable<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {}", DEFINE_TABLE, self.name);
        if self.drop {
            write!(f, " {}", DROP);
        }
        if let Some(schema) = self.schema.as_ref() {
            write!(f, " {}", &schema.to_string());
        }
        if let Some(expr) = self.as_expression {
            write!(f, " AS {}", expr);
        }
        if let Some(changefeed) = &self.changefeed {
            write!(f, " CHANGEFEED {}", changefeed);
        }
        if let Some(permissions) = self.permissions.as_ref() {
            write!(f, " {}", &permissions.to_string());
        }
        write!(f, "{}", STMT_END)
    }
}