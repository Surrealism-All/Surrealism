//! # field for select statement
//! - 0.3.0 -> surrealdb 1.0.0 Add OMIT
//! ```txt
//! @author:syf20020816@Outlook.com
//! @date:2023/8/27
//! @version:0.0.1
//! @description:
//! ```

use std::fmt::{Display, Formatter};
use crate::db::ParamCombine;
use super::constants::{ALL, DIFF};

/// # Field for STMT
///
/// ## example
/// ```rust
/// use surrealism::DefaultRes;
/// use surrealism::db::Field;
///
/// // [tests\src\main.rs:12] field_all = "*"
/// // [tests\src\main.rs:13] field_diff = "DIFF"
/// // [tests\src\main.rs:14] field = "username AS name"
/// #[tokio::main]
/// async fn main() -> DefaultRes<()> {
///     let field_all = Field::all().to_string();
///     let field_diff = Field::diff().to_string();
///     let field = Field::new_field("username").as_field("name").to_string();
///     Ok(())
/// }
/// ```
#[derive(Debug, Clone)]
pub struct Field<'f> {
    field: &'f str,
    as_field: Option<&'f str>,
}

impl<'f> Default for Field<'f> {
    fn default() -> Self {
        Field {
            field: "",
            as_field: None,

        }
    }
}

impl<'f> Field<'f> {
    pub fn new() -> Self {
        Field::default()
    }
    pub fn all() -> Self {
        Field {
            field: ALL,
            as_field: None,

        }
    }
    pub fn diff() -> Self {
        Field {
            field: DIFF,
            as_field: None,

        }
    }
    pub fn new_field(field: &'f str) -> Self {
        Field {
            field,
            as_field: None,

        }
    }
    pub fn field(&mut self, field: &'f str) -> &mut Self {
        self.field = field;
        self
    }
    pub fn as_field(&mut self, as_field: &'f str) -> &mut Self {
        self.as_field.replace(as_field);
        self
    }

    pub fn get_field(&self) -> &'f str {
        self.field
    }
    pub fn get_as_field(&self) -> Option<&'f str> {
        self.as_field
    }
}

impl<'f> From<&'f str> for Field<'f> {
    fn from(value: &'f str) -> Self {
        match value {
            ALL => Field::all(),
            DIFF => Field::diff(),
            _ => Field::new_field(value)
        }
    }
}

impl<'f> From<(&'f str, &'f str)> for Field<'f> {
    fn from(value: (&'f str, &'f str)) -> Self {
        Field {
            field: value.0,
            as_field: Some(value.1),

        }
    }
}


impl<'f> From<Vec<&'f str>> for Field<'f> {
    fn from(value: Vec<&'f str>) -> Self {
        let len = value.len();
        match len {
            0 => Field::all(),
            1 => Field::new_field(value[0]),
            2 => Field::from((value[0], value[1])),
            _ => panic!("Invalid field length!")
        }
    }
}


impl<'f> ParamCombine for Field<'f> {
    fn combine(&self) -> String {
        self.to_string()
    }
}

impl<'f> Display for Field<'f> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut res = self.get_field().to_string();
        let as_field = self.get_as_field();
        if as_field.is_some() {
            res.push_str(" AS ");
            res.push_str(as_field.unwrap());
        }
        write!(f, "{}", res)
    }
}