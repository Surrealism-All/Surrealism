//! # Define Field
//! ```txt
//! @author:syf20020816@Outlook.com
//! @date:2023/10/31
//! @version:0.0.1
//! @description:
//! ```

use std::fmt::{Display, Formatter};
use crate::db::constants::{DEFINE_FIELD, ON_TABLE, STMT_END};
use crate::db::ValueConstructor;
use super::{OnType, Permissions};


#[derive(Debug, Clone)]
pub struct DefineField<'a> {
    name: &'a str,
    on: OnType<'a>,
    value: Option<ValueConstructor>,
    permissions: Option<Permissions>,

}

impl<'a> Default for DefineField<'a> {
    fn default() -> Self {
        DefineField {
            name: "",
            on: OnType::TABLE(""),
            value: None,
            permissions: None,
        }
    }
}

impl<'a> DefineField<'a> {
    pub fn new(
        name: &'a str,
        table: &'a str,
        value: Option<ValueConstructor>,
        permissions: Option<Permissions>,
    ) -> Self {
        DefineField {
            name,
            on: OnType::TABLE(table),
            value,
            permissions,
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
    pub fn value(&mut self, value: ValueConstructor) -> &mut Self {
        self.value.replace(value);
        self
    }
    pub fn permissions(&mut self, permissions: Permissions) -> &mut Self {
        self.permissions.replace(permissions);
        self
    }
    pub fn build(&self) -> String {
        self.to_string()
    }
}

impl<'a> Display for DefineField<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {} {} {}", DEFINE_FIELD, self.name, ON_TABLE, &self.on.to_string());
        if let Some(value) = self.value.as_ref() {
            write!(f, " {}", &value.to_string());
        }
        if let Some(permissions) = self.permissions.as_ref() {
            write!(f, " {}", &permissions.to_string());
        }
        write!(f, "{}", STMT_END)
    }
}