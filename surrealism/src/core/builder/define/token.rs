//! # Define Token
//! ```txt
//! @author:syf20020816@Outlook.com
//! @date:2023/10/31
//! @version:0.0.1
//! @description:

use super::{OnType, TokenType};

/// DEFINE TOKEN @name ON [ NAMESPACE | DATABASE | SCOPE @scope ] TYPE @type VALUE @value
#[derive(Debug, Clone)]
pub struct DefineToken<'a> {
    name: &'a str,
    on: OnType<'a>,
    token_type: TokenType,
    value: &'a str,
}

impl<'a> Default for DefineToken<'a>{
    fn default() -> Self {
        DefineToken{
            name: "",
            on: OnType::default(),
            token_type: TokenType::defa,
            value: ""
        }
    }
}


impl<'a> DefineToken<'a> {
    pub fn new(
        name: &'a str,
        on: OnType<'a>,
        token_type: TokenType,
        value: &'a str,
    ) -> Self {
        DefineToken{
            name,
            on,
            token_type,
            value
        }
    }
}