//!
//! ```txt
//! @author:syf20020816@Outlook.com
//! @date:2023/8/25
//! @version:0.0.1
//! @description:
//! ```
use crate::{ParamCombine, SurrealValue};
use serde::{Serialize, Deserialize};
use super::{Operator};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Set {
    column: String,
    value: SurrealValue,
    sign: Operator,
}

impl Set {
    pub fn new(column: &str, value: SurrealValue, sign: Operator) -> Self {
        Self {
            column: String::from(column),
            value,
            sign,
        }
    }
    pub fn build(&self) -> String {
        format!("{} {} {}", &self.column, self.sign.to_str(), self.value.to_str())
    }
    /// get column and value
    pub fn get(&self) -> (&str, &SurrealValue) {
        (&self.column, &self.value)
    }
}

impl ParamCombine for Set {
    fn combine(&self) -> String {
        self.build()
    }
}

