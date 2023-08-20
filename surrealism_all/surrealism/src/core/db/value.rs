use std::collections::{BTreeMap, HashMap};
use serde::{Serialize, Deserialize};
use serde_json::Value;
use crate::{handle_str, remove_format_half};
use super::{NULL, NULL_DOWN, NONE_DOWN, NONE, TRUE, TRUE_STR, FALSE, FALSE_STR, LEFT_BRACE, RIGHT_BRACE, COMMA};

/// SurrealDB对应的值类型
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum SurrealValue {
    None,
    Null,
    Bool(bool),
    Int(i32),
    Float(f32),
    Decimal(f64),
    Str(String),
    Object(Object),
    Array(Array),
}

impl SurrealValue {
    ///将SurrealValue转换为String
    pub fn to_str(&self) -> String {
        match self {
            SurrealValue::None => format!("'{}'", NONE),
            SurrealValue::Null => format!("'{}'", NULL),
            SurrealValue::Bool(b) => b.to_string(),
            SurrealValue::Int(int) => int.to_string(),
            SurrealValue::Float(float) => float.to_string(),
            SurrealValue::Decimal(decimal) => decimal.to_string(),
            SurrealValue::Str(s) => handle_str(serde_json::to_string(s).unwrap().as_str()),
            SurrealValue::Object(obj) => obj.parse(),
            SurrealValue::Array(arr) => arr.parse()
        }
    }

    pub fn from_str(value: &str) -> SurrealValue {
        let value_str: Value = serde_json::from_str(value).unwrap();
        let res: SurrealValue = value_str.into();
        res
    }
}

///将serde的Value类型转为为SurrealValue
impl From<Value> for SurrealValue {
    fn from(value: Value) -> Self {
        match value {
            Value::Null => SurrealValue::Null,
            Value::Bool(b) => SurrealValue::Bool(b),
            Value::Number(n) => {
                if n.is_f64() {
                    SurrealValue::Decimal(n.as_f64().unwrap())
                } else {
                    let num = n.as_u64().unwrap();
                    SurrealValue::Int(num as i32)
                }
            }
            Value::String(s) => {
                let value: SurrealValue = s.as_str().into();
                value
            }
            Value::Array(arr) => {
                let mut values = vec![];
                for item in arr {
                    let value: SurrealValue = item.into();
                    values.push(value);
                }
                SurrealValue::Array(Array::from(values))
            }
            Value::Object(obj) => {
                let value: BTreeMap<String, SurrealValue> = obj.into_iter().map(|(k, v)| (k, v.into())).collect();
                SurrealValue::Object(Object::from(value))
            }
        }
    }
}

impl From<&str> for SurrealValue {
    fn from(value: &str) -> Self {
        match value {
            NONE => SurrealValue::None,
            NONE_DOWN => SurrealValue::None,
            NULL => SurrealValue::Null,
            NULL_DOWN => SurrealValue::Null,
            _ => SurrealValue::Str(String::from(value))
        }
    }
}

impl From<bool> for SurrealValue {
    fn from(value: bool) -> Self {
        SurrealValue::Bool(value)
    }
}

impl From<i32> for SurrealValue {
    fn from(value: i32) -> Self {
        SurrealValue::Int(value)
    }
}

impl From<u32> for SurrealValue {
    fn from(value: u32) -> Self {
        SurrealValue::Int(value as i32)
    }
}

impl From<usize> for SurrealValue {
    fn from(value: usize) -> Self {
        SurrealValue::Int(value as i32)
    }
}

impl From<isize> for SurrealValue {
    fn from(value: isize) -> Self {
        SurrealValue::Int(value as i32)
    }
}

impl From<f32> for SurrealValue {
    fn from(value: f32) -> Self {
        SurrealValue::Float(value)
    }
}

impl From<f64> for SurrealValue {
    fn from(value: f64) -> Self {
        SurrealValue::Decimal(value)
    }
}

impl From<String> for SurrealValue {
    fn from(value: String) -> Self {
        let value: SurrealValue = value.as_str().into();
        value
    }
}

impl From<Vec<SurrealValue>> for SurrealValue {
    fn from(value: Vec<SurrealValue>) -> Self {
        SurrealValue::Array(Array::from(value))
    }
}


/// Surreal对应的对象类型，使用B-Tree
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Object(BTreeMap<String, SurrealValue>);

impl Object {
    ///将BTreeMap<String, SurrealValue>转为String
    pub fn parse(&self) -> String {
        let mut res = String::from(LEFT_BRACE);
        let mut count: usize = 0;
        for (key, value) in &self.0 {
            count += 1;
            res += format!("{} : {}", key, value.to_str()).as_str();
            if self.0.len().ne(&count) {
                res += COMMA
            }
        }
        res += RIGHT_BRACE;
        res
    }
    ///将可序列化struct转为Surrealism::Object
    pub fn from_obj<T: Serialize>(t: &T) -> Object {
        //序列化为String
        let obj_str: Value = serde_json::to_value(t).unwrap();
        let res: SurrealValue = obj_str.into();
        match res {
            SurrealValue::Object(obj) => obj,
            _ => panic!("parse SurrealValue::Object failed"),
        }
    }
}

impl From<BTreeMap<String, SurrealValue>> for Object {
    fn from(v: BTreeMap<String, SurrealValue>) -> Self {
        Self(v)
    }
}

impl From<HashMap<String, SurrealValue>> for Object {
    fn from(v: HashMap<String, SurrealValue>) -> Self {
        Self(v.into_iter().collect())
    }
}

/// Surreal对应的数组类型
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Array(Vec<SurrealValue>);

impl Array {
    /// New A Empty Vector<SurrealValue>
    pub fn new() -> Self {
        Array(vec![])
    }
    pub fn from(value: Vec<SurrealValue>) -> Self {
        Array(value)
    }
    pub fn push(&mut self, value: SurrealValue) -> &mut Self {
        self.0.push(value);
        self
    }
    /// convert Vec<SurrealValue> -> String like:
    /// ``` code
    /// [{Int:1},'None',{Bool:true}] -> [1,'None',true]
    /// ```
    pub fn parse(&self) -> String {
        let mut res = vec![];
        for item in &self.0 {
            let item_str = item.to_str();
            res.push(item_str);
        }
        remove_format_half(format!("{:?}", res))
    }
}

impl From<Vec<SurrealValue>> for Array {
    fn from(value: Vec<SurrealValue>) -> Self {
        Array(value)
    }
}