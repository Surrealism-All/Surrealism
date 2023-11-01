//! # Define Function
//!
//! DEFINE FUNCTION fn::@name(
//! 	[ @argument: @type ... ]
//! ) {
//! 	[ @query ... ]
//! 	[ RETURN @returned ]
//! }
//!
//! ```txt
//! @author:syf20020816@Outlook.com
//! @date:2023/10/31
//! @version:0.0.1
//! @description:
//! ```



use std::fmt::{Display, Formatter};
use crate::db::constants::{DEFINE_FUNCTION, LEFT_BRACE, RETURN, RIGHT_BRACE, STMT_END};

#[derive(Debug, Clone)]
pub struct DefineFunction<'a> {
    name: &'a str,
    args: Option<Vec<&'a str>>,
    query: Option<&'a str>,
    returned: Option<&'a str>,
}

impl<'a> Default for DefineFunction<'a> {
    fn default() -> Self {
        DefineFunction {
            name: "",
            args: None,
            query: None,
            returned: None,
        }
    }
}

impl<'a> DefineFunction<'a> {
    pub fn new(
        name: &'a str,
        args: Option<Vec<&'a str>>,
        query: Option<&'a str>,
        returned: Option<&'a str>,
    ) -> Self {
        DefineFunction {
            name,
            args,
            query,
            returned,
        }
    }
    pub fn name(&mut self, name: &'a str) -> &mut Self {
        self.name = name;
        self
    }
    pub fn args(&mut self, args: Vec<&'a str>) -> &mut Self {
        self.args.replace(args);
        self
    }
    pub fn query(&mut self, query: &'a str) -> &mut Self {
        self.query.replace(query);
        self
    }
    pub fn returned(&mut self, returned: &'a str) -> &mut Self {
        self.returned.replace(returned);
        self
    }
    pub fn build(&self) -> String {
        self.to_string()
    }
}

impl<'a> Display for DefineFunction<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} fn::{}(", DEFINE_FUNCTION, self.name);
        if let Some(args) = self.args.as_ref() {
            write!(f, "{}) {}", args.join(", "), LEFT_BRACE);
        }
        if let Some(query) = self.query {
            write!(f, "{}", query);
        }
        if let Some(returned) = self.returned {
            write!(f, " {}", returned);
        }
        write!(f, "{}{}", RIGHT_BRACE, STMT_END)
    }
}