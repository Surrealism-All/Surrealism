//! # field for select statement
//! ```txt
//! @author:syf20020816@Outlook.com
//! @date:2023/8/27
//! @version:0.0.1
//! @description:
//! ```

use super::constants::{ALL};

pub enum Field<'f> {
    All,
    Fields(Vec<&'f str>),
}

impl<'f> Default for Field<'f> {
    fn default() -> Self {
        Field::All
    }
}

impl<'f> From<&'f str> for Field<'f> {
    fn from(value: &'f str) -> Self {
        match value {
            ALL => Field::All,
            other => Field::Fields(vec![other])
        }
    }
}

impl<'f> From<Vec<&'f str>> for Field<'f> {
    fn from(value: Vec<&'f str>) -> Self {
        Field::Fields(value)
    }
}

impl<'f> Field<'f> {
    pub fn to_str(&self) -> String {
        match self {
            Field::All => ALL.to_string(),
            Field::Fields(fields) => fields.join(" , ")
        }
    }
}