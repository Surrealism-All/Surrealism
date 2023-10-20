use std::fmt::{Display, Formatter};
use serde::{Serialize, Deserialize};

use super::constants::{UUID, ULID, RAND, EQ};
use super::{SurrealValue, Object, Array, ParamCombine};

/// # ID的枚举类型
/// 通过SurrealID快速生成一个含有类型的ID
/// ## example
/// ``` rust
/// use surrealism::DefaultRes;
/// use surrealism::db::{Range, SurrealID, Array, SurrealValue, Object};
/// use serde::{Serialize,Deserialize};
///
/// #[derive(Debug,Serialize,Deserialize)]
/// struct User<'a>{
///     name: &'a str,
///     age: u8,
/// }
///
/// // [tests\src\main.rs:27] id1.to_string() = "rand()"
/// // [tests\src\main.rs:28] id2.to_string() = ""
/// // [tests\src\main.rs:29] id3.to_string() = "surrealism"
/// // [tests\src\main.rs:30] id4.to_string() = "56"
/// // [tests\src\main.rs:31] id5.to_string() = "45.5454647"
/// // [tests\src\main.rs:32] id6.to_string() = "['John', 'Matt']"
/// // [tests\src\main.rs:33] id7.to_string() = "{ age : 16 , name : 'Mat' }"
/// // [tests\src\main.rs:34] id8.to_string() = "'2'..='6'"
/// // [tests\src\main.rs:35] id9.to_string() = "ulid()"
/// #[tokio::main]
/// async fn main() -> DefaultRes<()> {
///     let id1 = SurrealID::RAND;
///     let id2 = SurrealID::Default;
///     let id3 = SurrealID::from("surrealism");
///     let id4 = SurrealID::Int(56_i64);
///     let id5 = SurrealID::Float(45.5454647_f64);
///     let id6 = SurrealID::Array(Array::from(vec!["John".into(), "Matt".into()]));
///     let user = User {
///         name: "Mat",
///         age: 16,
///     };
///     let id7 = SurrealID::Object(Object::from_obj(&user));
///     let id8 = SurrealID::Range(Range::new_from_str("2", "6", true));
///     let id9 = SurrealID::from("ulid()");
///     dbg!(id1.to_string());
///     dbg!(id2.to_string());
///     dbg!(id3.to_string());
///     dbg!(id4.to_string());
///     dbg!(id5.to_string());
///     dbg!(id6.to_string());
///     dbg!(id7.to_string());
///     dbg!(id8.to_string());
///     dbg!(id9.to_string());
///     Ok(())
/// }
/// ```
#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum SurrealID {
    Default,
    Int(i64),
    Float(f64),
    String(String),
    Object(Object),
    Array(Array),
    UUID,
    ULID,
    RAND,
    Range(Range),
}

impl Default for SurrealID {
    fn default() -> Self {
        SurrealID::Default
    }
}

impl ParamCombine for SurrealID {
    fn combine(&self) -> String {
        self.to_string()
    }
}

impl Display for SurrealID {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let res = match self {
            SurrealID::Default => String::new(),
            SurrealID::Int(int) => int.to_string(),
            SurrealID::Float(f) => f.to_string(),
            SurrealID::String(s) => String::from(s),
            SurrealID::Object(obj) => obj.parse(),
            SurrealID::Array(arr) => arr.parse(),
            SurrealID::ULID => ULID.to_string(),
            SurrealID::UUID => UUID.to_string(),
            SurrealID::RAND => RAND.to_string(),
            SurrealID::Range(range) => range.to_string()
        };
        write!(f, "{}", res)
    }
}

impl From<i64> for SurrealID {
    fn from(value: i64) -> Self {
        SurrealID::Int(value)
    }
}

impl From<i32> for SurrealID {
    fn from(value: i32) -> Self {
        SurrealID::Int(value as i64)
    }
}

impl From<f32> for SurrealID {
    fn from(value: f32) -> Self {
        SurrealID::Float(value as f64)
    }
}

impl From<f64> for SurrealID {
    fn from(value: f64) -> Self {
        SurrealID::Float(value)
    }
}

impl From<&str> for SurrealID {
    fn from(value: &str) -> Self {
        match value {
            RAND => SurrealID::RAND,
            UUID => SurrealID::UUID,
            ULID => SurrealID::ULID,
            other => SurrealID::String(String::from(other))
        }
    }
}

impl From<String> for SurrealID {
    fn from(value: String) -> Self {
        match value.as_str() {
            RAND => SurrealID::RAND,
            UUID => SurrealID::UUID,
            ULID => SurrealID::ULID,
            other => SurrealID::String(String::from(other))
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
/// use surrealism::db::{Range,SurrealValue};
///     let range1 = Range::new(SurrealValue::Int(1), SurrealValue::Int(32), true).to_string();
///     let range2 = Range::new_from_str("2", "23", false).to_string();
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
}

impl Display for Range {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let res = if self.eq {
            format!("{}..{}{}", self.beg.to_string(), EQ, self.end.to_string())
        } else {
            format!("{}..{}", self.beg.to_string(), self.end.to_string())
        };
        write!(f, "{}", res)
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
