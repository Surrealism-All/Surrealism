//! # Surreal CONTENT SET
//! 1. Content
//! 2. Set
//! ## example
//! ```rust
//! use surrealism::{SurrealValue,ContentSet,Object};
//! use std::collections::HashMap;
//!
//!     let value1 = SurrealValue::None;
//!     dbg!(value1.is_none());
//!     let mut map: HashMap<&str, SurrealValue> = HashMap::new();
//!     let _ = map.insert("name", SurrealValue::Str(String::from("Mat")));
//!     let _ = map.insert("age", SurrealValue::Int(16));
//!     let _ = map.insert("address", SurrealValue::from("China - Shanghai"));
//!     let _ = map.insert("male", SurrealValue::Bool(true));
//!     let c_set1 = ContentSet::new_set(map);
//!     dbg!(c_set1.set());
//!     let user = User {
//!         name: "joker",
//!         age: 12,
//!     };
//!     let content = ContentSet::new_content(Object::from_obj(&user));
//!     dbg!(content.content());
//! ```
//! ```txt
//! @author:syf20020816@Outlook.com
//! @date:2023/8/21
//! @version:0.0.1
//! @description:
//! ```

use std::collections::HashMap;
use serde::Serialize;
use super::{SurrealValue, Object, ParamCombine, CONTENT, SET};

/// # SurrealContent
/// ContentSet is used to create wrapper `content | set` param
#[derive(Debug)]
pub enum ContentSet<'a> {
    Content(Object),
    Set(HashMap<&'a str, SurrealValue>),
}

impl<'a> ContentSet<'a> {
    /// new ContentSet::Content
    pub fn new_content(data: Object) -> Self {
        ContentSet::Content(data)
    }
    /// new ContentSet::Set
    pub fn new_set(data: HashMap<&'a str, SurrealValue>) -> Self {
        ContentSet::Set(data)
    }
    // pub fn from_content()->Self{
    //
    // }
    /// judge ContentSet is SET or CONTENT
    pub fn is_content(&self) -> bool {
        match self {
            ContentSet::Content(_) => true,
            ContentSet::Set(_) => false
        }
    }
    pub fn is_set(&self) -> bool {
        !self.is_content()
    }
    /// return ref Object from ContentSet
    pub fn content(&self) -> Option<&Object> {
        match self {
            ContentSet::Content(res) => Some(res),
            ContentSet::Set(_) => None
        }
    }
    /// return ref HashMap<&'a str, SurrealValue> from ContentSet
    pub fn set(&self) -> Option<&HashMap<&'a str, SurrealValue>> {
        match self {
            ContentSet::Content(_) => None,
            ContentSet::Set(res) => Some(res)
        }
    }
    /// return deref SurrealValue::Object(Option) from ContentSet and consume ContentSet
    pub fn content_deref(self) -> Option<SurrealValue> {
        match self {
            ContentSet::Content(res) => Some(SurrealValue::Object(res)),
            ContentSet::Set(_) => None
        }
    }
    /// return deref SurrealValue(Option) from ContentSet and consume ContentSet
    pub fn set_deref(self) -> Option<SurrealValue> {
        match self {
            ContentSet::Content(_) => None,
            ContentSet::Set(res) => Some(SurrealValue::Object(Object::from(res)))
        }
    }
    /// add K-V to ContentSet::Set
    /// ## example
    /// ```rust
    /// use std::collections::HashMap;
    /// use surrealism::{SurrealValue,Array,ContentSet};
    ///     let mut map: HashMap<&str, SurrealValue> = HashMap::new();
    ///     let _ = map.insert("name", SurrealValue::Str(String::from("Mat")));
    ///     let _ = map.insert("age", SurrealValue::Int(16));
    ///     let _ = map.insert("address", SurrealValue::from("China - Shanghai"));
    ///     let _ = map.insert("male", SurrealValue::Bool(true));
    ///     let mut c_set1 = ContentSet::new_set(map);
    ///     let mut arr = Array::new();
    ///     let _ = arr.push(SurrealValue::Str(String::from("cook")))
    ///         .push(SurrealValue::Str("author".to_string()));
    ///     c_set1.add("works", SurrealValue::Array(arr));
    /// ```
    pub fn add(&mut self, field: &'a str, value: SurrealValue) {
        match self {
            ContentSet::Content(_) => {}
            ContentSet::Set(res) => { res.insert(field, value); }
        }
    }
}

impl<'a> ParamCombine for ContentSet<'a> {
    fn combine(&self) -> String {
        fn build(map: &HashMap<&str, SurrealValue>) -> String {
            map.iter()
                .map(|(k, v)| format!("{} = {}", k, v.to_str()))
                .collect::<Vec<String>>()
                .join(" , ")
        }
        match self {
            ContentSet::Content(content) => {
                format!("{} {}", CONTENT, content.parse())
            }
            ContentSet::Set(set) => {
                // convert to SurrealValue
                format!("{} {}", SET, build(set))
            }
        }
    }
}

