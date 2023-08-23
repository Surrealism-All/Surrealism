//! # where condition
//! ```txt
//! @author:syf20020816@Outlook.com
//! @date:2023/8/23
//! @version:0.0.1
//! @description:
//! ```
use crate::SurrealValue;
use super::{ParamCombine};
use super::constants::{EQ, LT, GT, GTE, LTE, LINK, NEQ};

#[derive(Debug, Clone, PartialEq)]
pub struct Condition {
    sign: CompareSign,
    left: ConditionUnit,
    right: ValueUnit,
}

impl Condition {
    /// new condition : left sign right (name='Mat')
    pub fn new(left: ConditionUnit, right: ValueUnit, sign: CompareSign) -> Self {
        Condition {
            left,
            right,
            sign,
        }
    }
    /// new a empty condition : "" = ""
    pub fn new_no_args() -> Self {
        Condition {
            sign: CompareSign::Eq,
            left: ConditionUnit::Column(String::new()),
            right: ValueUnit::Value(SurrealValue::Str(String::new())),
        }
    }
    /// build left
    pub fn left(&mut self, left: ConditionUnit) -> &mut Self {
        self.left = left;
        self
    }
    /// build left from str -> ConditionUnit::Column
    pub fn left_from_str(&mut self, left: &str) -> &mut Self {
        self.left = ConditionUnit::from(left);
        self
    }
    /// build left from Vec<&str> -> ConditionUnit::Links
    pub fn left_from_vec(&mut self, left: Vec<&str>) -> &mut Self {
        self.left = ConditionUnit::from(left);
        self
    }
    /// add item to left which is used in ConditionUnit::Links
    pub fn add_to_left(&mut self, item: &str) -> &mut Self {
        match self.left {
            ConditionUnit::Column(_) => { panic!("{}", "add_to_left() can just use in ConditionUnit::Links"); }
            ConditionUnit::Links(ref mut link) => {
                link.push(String::from(item));
            }
        };
        self
    }
    /// build right
    pub fn right(&mut self, right: ValueUnit) -> &mut Self {
        self.right = right;
        self
    }
    /// build right from str -> ValueUnit
    pub fn right_from_str(&mut self, right: &str, is_value: bool) -> &mut Self {
        if is_value {
            self.right = ValueUnit::Value(SurrealValue::from(right));
        } else {
            self.right = ValueUnit::Unit(ConditionUnit::from(right));
        }
        self
    }
}

/// value unit for condition right
/// - value is basic
/// - unit(ConditionUnit) is for complex condition
#[derive(Debug, Clone, PartialEq)]
pub enum ValueUnit {
    Value(SurrealValue),
    Unit(ConditionUnit),
}


/// condition unit for condition left
/// - column is basic
/// - links means Multiple Table Connection
/// ## example
/// ```rust
/// use surrealism::{ConditionUnit,ParamCombine};
///     // [tests\src\main.rs:29] u1 = "name"
///     // [tests\src\main.rs:30] u2 = "person"
///     // [tests\src\main.rs:31] u3 = "->user->person->job->"
///     let u1 = ConditionUnit::from("name").combine();
///     let u2 = ConditionUnit::from("person".to_string()).combine();
///     let u3 = ConditionUnit::from(vec!["user","person","job"]).combine();
/// ```
#[derive(Debug, Clone, PartialEq)]
pub enum ConditionUnit {
    Column(String),
    Links(Vec<String>),
}

impl ParamCombine for ConditionUnit {
    fn combine(&self) -> String {
        match self {
            ConditionUnit::Column(column) => column.to_string(),
            ConditionUnit::Links(list) => {
                // 长度小于2，不满足生成条件
                if 2_usize.gt(&list.len()) {
                    panic!("{}", "Multiple Table Connection need length >= 2")
                } else {
                    // like: ->knows->person->(knows WHERE influencer = true)
                    let mut res = String::from(LINK);
                    for item in list {
                        res.push_str(item.as_str());
                        res.push_str(LINK);
                    }
                    res
                }
            }
        }
    }
}

impl From<&str> for ConditionUnit {
    fn from(value: &str) -> Self {
        ConditionUnit::Column(String::from(value))
    }
}

impl From<String> for ConditionUnit {
    fn from(value: String) -> Self {
        ConditionUnit::Column(value)
    }
}

impl From<Vec<String>> for ConditionUnit {
    fn from(value: Vec<String>) -> Self {
        ConditionUnit::Links(value)
    }
}

impl From<Vec<&str>> for ConditionUnit {
    fn from(value: Vec<&str>) -> Self {
        let value: Vec<String> = value.into_iter().map(|x| String::from(x)).collect();
        ConditionUnit::Links(value)
    }
}

/// # compare sign
/// - Eq:等于
/// - Lt:小于
/// - Gt:大于
/// - Neq:不等于
/// - Lte:小于等于
/// - Gte:大于等于
/// - Link:连接`->`
#[derive(Debug, Clone, PartialEq)]
pub enum CompareSign {
    Eq,
    Lt,
    Gt,
    Neq,
    Lte,
    Gte,
    Link,
}

impl CompareSign {
    pub fn to_str(&self) -> &str {
        match self {
            CompareSign::Eq => EQ,
            CompareSign::Lt => LT,
            CompareSign::Gt => GT,
            CompareSign::Neq => NEQ,
            CompareSign::Lte => LTE,
            CompareSign::Gte => GTE,
            CompareSign::Link => LINK,
        }
    }
}

impl From<&str> for CompareSign {
    fn from(value: &str) -> Self {
        match value {
            EQ => CompareSign::Eq,
            LT => CompareSign::Lt,
            GT => CompareSign::Gt,
            LTE => CompareSign::Lte,
            GTE => CompareSign::Gte,
            NEQ => CompareSign::Neq,
            LINK => CompareSign::Link,
            _ => panic!("{}", "can not match compare sign")
        }
    }
}

impl From<String> for CompareSign {
    fn from(value: String) -> Self {
        match value.as_str() {
            EQ => CompareSign::Eq,
            LT => CompareSign::Lt,
            GT => CompareSign::Gt,
            LTE => CompareSign::Lte,
            GTE => CompareSign::Gte,
            NEQ => CompareSign::Neq,
            LINK => CompareSign::Link,
            _ => panic!("{}", "can not match compare sign")
        }
    }
}