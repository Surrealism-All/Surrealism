//!
//! ```txt
//! @author:syf20020816@Outlook.com
//! @date:2023/8/23
//! @version:0.0.1
//! @description:
//! ```
use crate::{Object, ParamCombine, Set};
use super::{Patch};
use super::constants::{SET, CONTENT, MERGE, PATCH};
use serde::{Serialize, Deserialize};


/// # strategy for CreateWrapper
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CreateStrategy {
    Content(Object),
    Set(Vec<Set>),
}

impl CreateStrategy {
    pub fn push(&mut self, item: Set) -> &mut Self {
        match self {
            CreateStrategy::Content(_) => panic!("{}", "push function can just use in CreateStrategy::Set"),
            CreateStrategy::Set(ref mut set) => set.push(item)
        }
        self
    }
    pub fn get(&self, index: usize) -> &Set {
        match self {
            CreateStrategy::Content(_) => panic!("{}", "get function can just use in CreateStrategy::Set"),
            CreateStrategy::Set(ref set) => &set[index]
        }
    }
}

impl From<Object> for CreateStrategy {
    fn from(value: Object) -> Self {
        CreateStrategy::Content(value)
    }
}

impl From<Vec<Set>> for CreateStrategy {
    fn from(value: Vec<Set>) -> Self {
        CreateStrategy::Set(value)
    }
}

impl ParamCombine for CreateStrategy {
    fn combine(&self) -> String {
        match self {
            CreateStrategy::Content(content) => format!("{} {}", CONTENT, content.parse()),
            CreateStrategy::Set(set) => format!("{} {}", SET, set.iter().map(|x| x.combine()).collect::<Vec<String>>().join(" , ")),
        }
    }
}

/// # strategy for UpdateWrapper
#[derive(Debug, Clone, Serialize)]
pub enum UpdateStrategy<'u> {
    Content(Object),
    Set(Vec<Set>),
    Merge(Object),
    Patch(Patch<'u>),
}

impl<'u> UpdateStrategy<'u> {
    pub fn from_content(value: Object) -> Self {
        UpdateStrategy::Content(value)
    }
    pub fn from_merge(value: Object) -> Self {
        UpdateStrategy::Merge(value)
    }
    pub fn push(&mut self, item: Set) -> &mut Self {
        match self {
            UpdateStrategy::Set(ref mut set) => set.push(item),
            _ => panic!("{}", "push function can just use in CreateStrategy::Set"),
        }
        self
    }
    pub fn get(&self, index: usize) -> &Set {
        match self {
            UpdateStrategy::Set(ref set) => &set[index],
            _ => panic!("{}", "get function can just use in CreateStrategy::Set"),
        }
    }
    pub fn build(&self) -> String {
        match self {
            UpdateStrategy::Content(content) => format!("{} {}", CONTENT, content.parse()),
            UpdateStrategy::Set(set) => format!("{} {}", SET, set.iter().map(|x| x.combine()).collect::<Vec<String>>().join(" , ")),
            UpdateStrategy::Merge(merge) => format!("{} {}{}", MERGE, "settings:", merge.parse()),
            UpdateStrategy::Patch(patch) => format!("{} [ {} ]", PATCH, patch.combine())
        }
    }
}

impl<'u> From<Vec<Set>> for UpdateStrategy<'u> {
    fn from(value: Vec<Set>) -> Self {
        UpdateStrategy::Set(value)
    }
}

impl<'u> From<Patch<'u>> for UpdateStrategy<'u> {
    fn from(value: Patch<'u>) -> Self {
        UpdateStrategy::Patch(value)
    }
}

impl<'u> ParamCombine for UpdateStrategy<'u> {
    fn combine(&self) -> String {
        self.build()
    }
}

/// # strategy for InsertWrapper
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum InsertStrategy {
    Content(Vec<Object>),
    Set(Vec<Set>),
    Stmt(String),
}

impl InsertStrategy {
    pub fn push_content(&mut self, item: Object) -> &mut Self {
        match self {
            InsertStrategy::Content(ref mut content) => content.push(item),
            _ => panic!("{}", "push_content function can just use in InsertStrategy::Content"),
        }
        self
    }
    pub fn push_set(&mut self, item: Set) -> &mut Self {
        match self {
            InsertStrategy::Set(ref mut set) => set.push(item),
            _ => panic!("{}", "push_set function can just use in InsertStrategy::Set"),
        }
        self
    }
    pub fn push(&mut self, stmt: &str) -> &mut Self {
        match self {
            InsertStrategy::Stmt(s) => *s = String::from(stmt),
            _ => panic!("{}", "push_set function can just use in InsertStrategy::Set"),
        }
        self
    }
    pub fn get_set(&self, index: usize) -> &Set {
        match self {
            InsertStrategy::Set(ref set) => &set[index],
            _ => panic!("{}", "get_set function can just use in InsertStrategy::Set"),
        }
    }
}

impl From<Vec<Object>> for InsertStrategy {
    fn from(value: Vec<Object>) -> Self {
        InsertStrategy::Content(value)
    }
}

impl From<Vec<Set>> for InsertStrategy {
    fn from(value: Vec<Set>) -> Self {
        InsertStrategy::Set(value)
    }
}

impl From<&str> for InsertStrategy {
    fn from(value: &str) -> Self {
        InsertStrategy::Stmt(value.to_string())
    }
}

impl From<String> for InsertStrategy {
    fn from(value: String) -> Self {
        InsertStrategy::Stmt(value)
    }
}

impl ParamCombine for InsertStrategy {
    fn combine(&self) -> String {
        match self {
            InsertStrategy::Content(content) => {

                return if 1.eq(&content.len()) {
                    format!("{} {}", CONTENT, content[0].parse())
                } else {
                    format!("{} [ {} ]", CONTENT, content.iter().map(|x| x.parse()).collect::<Vec<String>>().join(" , "))
                };
            }
            InsertStrategy::Set(set) => format!("{} {}", SET, set.iter().map(|x| x.combine()).collect::<Vec<String>>().join(" , ")),
            InsertStrategy::Stmt(stmt) => format!("( {} )", stmt)
        }
    }
}