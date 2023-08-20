use serde::{Serialize, Deserialize};
use super::{UUID, ULID, RAND, EQ, SurrealValue, Object, Array};

/// # ID的枚举类型
/// 通过SurrealID快速生成一个含有类型的ID
/// ## example
/// ``` code
///     let n1 = IDNumber::Int(56).to_str();
///     let sn1 = SurrealID::<String>::Default.to_str();
///     let sn2 = SurrealID::<String>::Str("Joe".to_string()).to_str();
///     let sn3 = SurrealID::<User>::Array(vec![User { name: "Joe", age: 16 }, User { name: "Mark", age: 25 }]);
///     let sn4 = SurrealID::<f32>::Number(IDNumber::Float(23.56546_f32)).to_str();
///     let sn5 = SurrealID::<User>::Object(User { name: "Mary", age: 23 });
///     let sn6 =  SurrealID::<String>::UUID;
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

impl SurrealID {
    /// Convert SurrealID to String
    pub fn to_str(&self) -> String {
        match self {
            SurrealID::Default => String::new(),
            SurrealID::Int(int) => int.to_string(),
            SurrealID::Float(f) => f.to_string(),
            SurrealID::Decimal(b_f) => b_f.to_string(),
            SurrealID::Str(s) => String::from(s),
            SurrealID::Object(obj) => serde_json::to_string(obj).unwrap(),
            SurrealID::Array(arr) => serde_json::to_string(arr).unwrap(),
            SurrealID::ULID => ULID.to_string(),
            SurrealID::UUID => UUID.to_string(),
            SurrealID::RAND => RAND.to_string(),
            SurrealID::Range(range) => range.to_str()
        }
    }
}

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