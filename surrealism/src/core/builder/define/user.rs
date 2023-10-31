//! # Define User
//! ```txt
//! @author:syf20020816@Outlook.com
//! @date:2023/10/31
//! @version:0.0.1
//! @description:
//! ```
use std::fmt::{Display, Formatter};
use crate::db::constants::{DEFINE_USER, ON, ROLES, STMT_END};
use super::{OnType, PwdType, Role};

#[derive(Debug, Clone)]
pub struct DefineUser<'a> {
    username: &'a str,
    on: OnType<'a>,
    password: PwdType<'a>,
    role: Role,
}

impl<'a> Default for DefineUser<'a> {
    fn default() -> Self {
        DefineUser {
            username: "",
            on: OnType::default(),
            password: PwdType::Pwd(""),
            role: Role::default(),
        }
    }
}

impl<'a> DefineUser<'a> {
    pub fn new(name: &'a str, on: OnType<'a>, pwd: PwdType<'a>, role: Role) -> Self {
        DefineUser {
            username: name,
            on,
            password: pwd,
            role,
        }
    }
    pub fn name(&mut self, name: &'a str) -> &mut Self {
        self.name = name;
        self
    }
    pub fn on(&mut self, on: OnType) -> &mut Self {
        self.on = on;
        self
    }
    pub fn pwd(&mut self,pwd:PwdType)->&mut Self{
        self.password = pwd;
        self
    }
    pub fn role(&mut self, role: Role)->&mut Self{
        self.role = role;
        self
    }
    pub fn build(&self) -> String {
        DefineUser::to_string()
    }
}

impl<'a> Display for DefineUser<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {} {} {} {} {} {}{}", DEFINE_USER, self.username, ON, self.on.to_string(), self.password.to_string(), ROLES, self.role.to_string(), STMT_END)
    }
}