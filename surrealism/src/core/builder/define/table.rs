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

use crate::Table::constants::{DEFINE_NS, STMT_END};

#[derive(Debug, Clone)]
pub struct DefineTable<'a> {
    name: &'a str,
    drop: bool,
    changefeed: Option<&'a str>,
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
    pub fn drop(&mut self) -> &mut self {
        self.drop = true;
        self
    }
    pub fn changefeed(&mut self, duration: u32, unit: TimeUnit) -> &mut self {
        self.changefeed = Some(format!("{} {}", duration, unit.to_str()).as_str());
        self
    }
    pub fn schemafull(&mut self) -> &mut self {
        self.schema.replace(Schema::SCHEMAFULL);
        self
    }
    pub fn schemaless(&mut self) -> &mut self {
        self.schema.replace(Schema::SCHEMALESS);
        self
    }
    pub fn as_expression(&mut self, expression: &'a str) -> &mut self {
        self.as_expression.replace(expression);
        self
    }
    pub fn permission(&mut self, permission: Permissions) -> &mut self {
        self.permissions.replace(permission);
        self
    }
    pub fn build(&self) -> String {
        DefineTable::to_string()
    }
}

impl<'a> Display for DefineTable<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut res = format!("{} {}", DEFINE_TABLE, name);
        if *self.drop {
            res.push_str(BLANK);
            res.push_str(DROP);
        }
        if self.schema.is_some() {
            res.push_str(BLANK);
            res.push_str(self.schema.as_ref().unwrap().to_string().as_str())
        }
        if self.as_expression.is_some() {
            res.push_str(" AS ");
            res.push_str(self.as_expression.as_ref().unwrap())
        }
        if self.changefeed.is_some() {
            res.push_str(" CHANGEFEED ");
            res.push_str(self.changefeed.as_ref().unwrap())
        }
        if self.permissions.is_some() {
            res.push_str(BLANK);
            res.push_str(&self.permissions.as_ref().unwrap().to_string())
        }
        res.push_str(STMT_END);

        write!(f, "{}", res)
    }
}