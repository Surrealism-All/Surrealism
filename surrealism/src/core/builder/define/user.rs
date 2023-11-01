//! # Define User
//! ```txt
//! @author:syf20020816@Outlook.com
//! @date:2023/10/31
//! @version:0.0.1
//! @description:
//! ```
use std::fmt::{Display, Formatter};
use crate::db::constants::{DEFINE_USER, ON, ROLES, STMT_END};
use crate::db::Roles;

use super::{OnType, PwdType};

#[derive(Debug, Clone)]
pub struct DefineUser<'a> {
    username: &'a str,
    on: Option<OnType<'a>>,
    password: Option<PwdType<'a>>,
    roles: Roles,
}

impl<'a> Default for DefineUser<'a> {
    fn default() -> Self {
        DefineUser {
            username: "",
            on: Some(OnType::default()),
            password: None,
            roles: Roles::default(),
        }
    }
}

impl<'a> DefineUser<'a> {
    pub fn new(name: &'a str, on: Option<OnType<'a>>, pwd: Option<PwdType<'a>>, roles: Roles) -> Self {
        if on.is_some() {
            if !on.as_ref().unwrap().on_user() {
                panic!("DEFINE USER should use OnType::ROOT | OnType::NS | OnType::DB!")
            }
        }
        DefineUser {
            username: name,
            on,
            password: pwd,
            roles,
        }
    }
    pub fn name(&mut self, name: &'a str) -> &mut Self {
        self.name = name;
        self
    }
    pub fn on(&mut self, on: OnType) -> &mut Self {
        if !on.as_ref().unwrap().on_user() {
            panic!("DEFINE USER should use OnType::ROOT | OnType::NS | OnType::DB!")
        }
        self.on.replace(on);
        self
    }
    pub fn pwd(&mut self, pwd: PwdType) -> &mut Self {
        self.password.replace(pwd);
        self
    }
    pub fn role(&mut self, roles: Roles) -> &mut Self {
        self.roles = roles;
        self
    }
    pub fn build(&self) -> String {
        DefineUser::to_string()
    }
}

impl<'a> Display for DefineUser<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {} {} {} {} {} {}{}", DEFINE_USER, self.username, ON, self.on.to_string(), self.password.to_string(), ROLES, self.roles.to_string(), STMT_END)
    }
}