use serde::{Serialize, Deserialize};
use serde_json::Value;

use crate::util::handle_str;
use super::constants::{UUID, ULID, RAND, EQ};
use super::{SurrealValue,Object,Array,ParamCombine};
/// # ID的枚举类型
/// 通过SurrealID快速生成一个含有类型的ID
/// ## example
/// ``` rust
/// use surrealism::{Range,SurrealID,Array,SurrealValue,Object};
///    let id1 = SurrealID::RAND;
///     let id2 = SurrealID::Default;
///     let id3 = SurrealID::Str(String::from("surrealism"));
///     let id4 = SurrealID::Int(56_i32);
///     let id5 = SurrealID::Float(45.5454647_f32);
///     let id6 = SurrealID::Array(Array::from(vec![SurrealValue::Str(String::from("John")), SurrealValue::Str(String::from("Mat"))]));
///     let user = User {
///         name: "Mat",
///         age: 16,
///     };
///     let id7 = SurrealID::Object(Object::from_obj(&user));
///     let id8 = SurrealID::Range(Range::new_from_str("2", "6", true));
///     let id9 = SurrealID::from("ulid()");
/// ```
#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum SurrealID {
    Default,
    Int(i32),
    Float(f32),
    Decimal(f64),
    Str(String),
    Object(Object),
    Array(Array),
    UUID,
    ULID,
    RAND,
    Range(Range),
}

impl Default for SurrealID {
    fn default() -> Self {
        SurrealID::RAND
    }
}

impl ParamCombine for SurrealID {
    fn combine(&self) -> String {
        self.to_str()
    }
}

impl SurrealID {
    /// Convert SurrealID to String
    pub fn to_str(&self) -> String {
        match self {
            SurrealID::Default => String::new(),
            SurrealID::Int(int) => int.to_string(),
            SurrealID::Float(f) => f.to_string(),
            SurrealID::Decimal(b_f) => b_f.to_string(),
            SurrealID::Str(s) => String::from(s),
            SurrealID::Object(obj) => obj.parse(),
            SurrealID::Array(arr) => arr.parse(),
            SurrealID::ULID => ULID.to_string(),
            SurrealID::UUID => UUID.to_string(),
            SurrealID::RAND => RAND.to_string(),
            SurrealID::Range(range) => range.to_str()
        }
    }
}

impl From<i32> for SurrealID {
    fn from(value: i32) -> Self {
        SurrealID::Int(value)
    }
}

impl From<f32> for SurrealID {
    fn from(value: f32) -> Self {
        SurrealID::Float(value)
    }
}

impl From<f64> for SurrealID {
    fn from(value: f64) -> Self {
        SurrealID::Decimal(value)
    }
}

impl From<&str> for SurrealID {
    fn from(value: &str) -> Self {
        match value {
            RAND => SurrealID::RAND,
            UUID => SurrealID::UUID,
            ULID => SurrealID::ULID,
            other => SurrealID::Str(String::from(other))
        }
    }
}

impl From<String> for SurrealID {
    fn from(value: String) -> Self {
        match value.as_str() {
            RAND => SurrealID::RAND,
            UUID => SurrealID::UUID,
            ULID => SurrealID::ULID,
            other => SurrealID::Str(String::from(other))
        }
    }
}

impl From<Array> for SurrealID {
    fn from(value: Array) -> Self {
        SurrealID::Array(value)
    }
}

impl From<Object> for SurrealID {
    fn from(value: Object) -> Self {
        SurrealID::Object(value)
    }
}

impl From<Range> for SurrealID {
    fn from(value: Range) -> Self {
        SurrealID::Range(value)
    }
}

/// # Id type range
/// ## example
/// ```rust
/// use surrealism::{Range,SurrealValue};
///     let range1 = Range::new(SurrealValue::Int(1), SurrealValue::Int(32), true).to_str();
///     let range2 = Range::new_from_str("2", "23", false).to_str();
/// ```
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Range {
    eq: bool,
    beg: SurrealValue,
    end: SurrealValue,
}

impl Range {
    pub fn new(beg: SurrealValue, end: SurrealValue, is_eq: bool) -> Self {
        Range {
            eq: is_eq,
            beg,
            end,
        }
    }
    pub fn new_from_str(beg: &str, end: &str, is_eq: bool) -> Self {
        Range {
            eq: is_eq,
            beg: beg.into(),
            end: end.into(),
        }
    }
    pub fn to_str(&self) -> String {
        return if self.eq {
            format!("{}..{}{}", self.beg.to_str(), EQ, self.end.to_str())
        } else {
            format!("{}..{}", self.beg.to_str(), self.end.to_str())
        };
    }
}

#[derive(Clone, Debug)]
pub enum SurrealIDType {
    Default,
    Int,
    Float,
    Decimal,
    /// UUID,ULID,RAND is Str too
    Str,
    Object,
    Array,
    Range,
}
