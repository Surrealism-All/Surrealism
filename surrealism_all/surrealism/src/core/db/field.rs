//! # field for select statement
//! ```txt
//! @author:syf20020816@Outlook.com
//! @date:2023/8/27
//! @version:0.0.1
//! @description:
//! ```

use crate::db::ParamCombine;
use super::constants::{ALL,DIFF};

#[derive(Debug, Clone)]
pub enum Field<'f> {
    All,
    Diff,
    Fields(Vec<(&'f str, &'f str)>),
}

impl<'f> Default for Field<'f> {
    fn default() -> Self {
        Field::All
    }
}

impl<'f> From<(&'f str, &'f str)> for Field<'f> {
    fn from(value: (&'f str, &'f str)) -> Self {
        Field::Fields(vec![value])
    }
}

impl<'f> From<&'f str> for Field<'f> {
    fn from(value: &'f str) -> Self {
        match value {
            ALL => Field::All,
            DIFF => Field::Diff,
            "diff" => Field::Diff,
            other => Field::Fields(vec![(other, "")])
        }
    }
}

impl<'f> From<Vec<&'f str>> for Field<'f> {
    fn from(value: Vec<&'f str>) -> Self {
        let value = value.iter().map(|x| (*x, "")).collect::<Vec<(&str, &str)>>();
        Field::from(value)
    }
}

impl<'f> From<Vec<(&'f str, &'f str)>> for Field<'f> {
    fn from(value: Vec<(&'f str, &'f str)>) -> Self {
        Field::Fields(value)
    }
}

impl<'f> Field<'f> {
    pub fn to_str(&self) -> String {
        match self {
            Field::All => ALL.to_string(),
            Field::Fields(fields) => {
                fields.iter().map(|(name, as_name)| {
                    if as_name.is_empty() {
                        name.to_string()
                    } else {
                        format!("{} AS {}", name, as_name)
                    }
                }).collect::<Vec<String>>()
                    .join(" , ")
            }
            Field::Diff => DIFF.to_string()
        }
    }
    pub fn push(&mut self, item: &'f str, as_name: Option<&'f str>) -> () {
        match self {
            Field::All|Field::Diff => (),
            Field::Fields(ref mut field) => {
                if as_name.is_some() {
                    field.push((item, as_name.as_ref().unwrap()))
                } else {
                    field.push((item, ""))
                }
            }
        }
    }
}

impl<'f> ParamCombine for Field<'f> {
    fn combine(&self) -> String {
        self.to_str()
    }
}