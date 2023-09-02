//! # SurrealDB对应值的类型
//! ## example
//! ```rust
//! use std::collections::HashMap;
//! use surrealism::{SurrealismRes, SurrealID, handle_str, Array, Object, SurrealValue};
//! use serde::{Serialize, Deserialize};
//!
//! #[derive(Serialize, Deserialize)]
//! struct User<'a> {
//!     name: &'a str,
//!     age: u32,
//! }
//!
//! #[tokio::main]
//! async fn main() -> SurrealismRes<()> {
//!     let mut id1 = Array::new();
//!     let _ = id1.push(SurrealValue::Int(1));
//!     let _ = id1.push(SurrealValue::None);
//!     let _ = id1.push(SurrealValue::Bool(true));
//!     let ss = id1.parse();
//!     dbg!(ss);
//!     let user = User { name: "Joe", age: 12 };
//!     let mut item = HashMap::new();
//!     item.insert("a".to_string(), SurrealValue::Array(id1));
//!     item.insert("b".to_string(), SurrealValue::Int(2));
//!     let mut id2 = Object::from(item);
//!     dbg!(id2.parse());
//!     let res = Object::from_obj(&user);
//!     dbg!(res.parse());
//!     Ok(())
//! }
//! ```
//! ```txt
//! @author:syf20020816@Outlook.com
//! @date:2023/8/21
//! @version:0.0.1
//! @description:
//! ```

use std::any::{Any, TypeId};
use std::collections::{BTreeMap, HashMap};

use serde::{Serialize, Deserialize};
use serde_json::Value;
use surrealdb::sql::{Datetime, Duration};
use crate::{Condition};
use crate::core::db::constants::{BLANK};
use crate::util::{remove_format_half, handle_str};
use super::constants::{NULL, NULL_DOWN, NONE_DOWN, NONE, LEFT_BRACE, RIGHT_BRACE, COMMA};

/// SurrealDB对应的值类型
/// Geometry类型当前版本不支持，预计版本 > 0.2.1
/// - feature
/// - point
/// - line
/// - polygon
/// - multipoint
/// - multiline
/// - multipolygon
/// - collection
/// 0.2.2版本后将消除SurrealValue 与 ValueType之间的隔阂 （这里的设计并不合理）
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum SurrealValue {
    DateTime(Datetime),
    Duration(Duration),
    Record(String),
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
    pub fn value_type(&self) -> ValueType {
        match self {
            SurrealValue::DateTime(_) => ValueType::DateTime,
            SurrealValue::Duration(_) => ValueType::Duration,
            SurrealValue::Record(_) => ValueType::Record,
            SurrealValue::Bool(_) => ValueType::Bool,
            SurrealValue::Int(_) => ValueType::Number,
            SurrealValue::Float(_) => ValueType::Number,
            SurrealValue::Decimal(_) => ValueType::Number,
            SurrealValue::Str(_) => ValueType::String,
            SurrealValue::Object(_) => ValueType::Object,
            SurrealValue::Array(_) => ValueType::Array,
            _ => ValueType::String
        }
    }
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
            SurrealValue::Array(arr) => arr.parse(),
            SurrealValue::DateTime(time) => time.to_string(),
            SurrealValue::Duration(duration) => duration.to_string(),
            SurrealValue::Record(record) => format!("record({})", record),
        }
    }
    ///从json-str进行推测，转换为serde::Value再转为SurrealValue
    /// ## example
    /// ```rust
    /// use surrealism::{SurrealValue};
    /// let v = SurrealValue::from_str("{ \"address\": \"China - Shanghai\"}");
    /// /*[tests\src\main.rs:41] v = Object(
    ///     Object(
    ///         {
    ///             "address": Str(
    ///                 "China - Shanghai",
    ///             ),
    ///         },
    ///     ),
    /// )*/
    /// ```
    pub fn from_str(value: &str) -> SurrealValue {
        let value_str: Value = serde_json::from_str(value).unwrap();
        let res: SurrealValue = value_str.into();
        res
    }
    pub fn is_none(&self) -> bool {
        match self {
            SurrealValue::None => true,
            _ => false
        }
    }
    pub fn is_null(&self) -> bool {
        match self {
            SurrealValue::Null => true,
            _ => false
        }
    }
    pub fn is_bool(&self) -> bool {
        match self {
            SurrealValue::Bool(_) => true,
            _ => false
        }
    }
    pub fn is_int(&self) -> bool {
        match self {
            SurrealValue::Int(_) => true,
            _ => false
        }
    }
    pub fn is_float(&self) -> bool {
        match self {
            SurrealValue::Float(_) => true,
            _ => false
        }
    }
    pub fn is_decimal(&self) -> bool {
        match self {
            SurrealValue::Decimal(_) => true,
            _ => false
        }
    }
    pub fn is_str(&self) -> bool {
        match self {
            SurrealValue::Str(_) => true,
            _ => false
        }
    }
    pub fn is_object(&self) -> bool {
        match self {
            SurrealValue::Object(_) => true,
            _ => false
        }
    }
    pub fn is_array(&self) -> bool {
        match self {
            SurrealValue::Array(_) => true,
            _ => false
        }
    }
    /// try to from each Type to SurrealValue
    pub fn from_each<T>(value: T) -> Self where T: Serialize {
        let value = serde_json::to_value(&value).unwrap();
        SurrealValue::from(value)
    }
    pub fn from_vec<T>(value: Vec<T>) -> Vec<SurrealValue> where T: Serialize {
        value.iter().map(|x| SurrealValue::from_each(x)).collect::<Vec<SurrealValue>>()
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

impl From<Vec<&str>> for SurrealValue {
    fn from(value: Vec<&str>) -> Self {
        SurrealValue::Array(Array::from_lower(value))
    }
}


/// Surreal对应的对象类型，使用B-Tree
/// ## example (expect HashMap<&str,SurrealValue>)
/// ```rust
/// use std::collections::HashMap;
/// use surrealism::{SurrealValue};
///     let mut map: HashMap<&str, SurrealValue> = HashMap::new();
///     let _ = map.insert("name", SurrealValue::Str(String::from("Mat")));
///     let _ = map.insert("age", SurrealValue::Int(16));
///     let _ = map.insert("address", SurrealValue::from("China - Shanghai"));
///     let _ = map.insert("male", SurrealValue::Bool(true));
///     let res = Object::from(map);
/// ```
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

impl<'a> From<HashMap<&'a str, SurrealValue>> for Object {
    fn from(value: HashMap<&'a str, SurrealValue>) -> Self {
        let value: HashMap<String, SurrealValue> = value
            .into_iter()
            .map(|(k, v)| {
                (k.to_string(), v)
            })
            .collect();
        Object::from(value)
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
    pub fn from_lower(value: Vec<&str>) -> Self {
        // &str -> SurrealValue
        let res = value.iter().map(|x| SurrealValue::from(*x)).collect::<Vec<SurrealValue>>();
        Array(res)
    }
}

impl From<Vec<SurrealValue>> for Array {
    fn from(value: Vec<SurrealValue>) -> Self {
        Array(value)
    }
}

/// # Value Type
/// 未来计划移除
///     ///当您明确不想指定字段的数据类型时，请使用此选项。该字段将允许SurrealDB支持的任何数据类型。
///     Any,
///     Array,
///     Bool,
///     ///一种符合ISO 8601的数据类型，用于存储带有时间和时区的日期。
///     Datetime,
///     ///使用BigDecimal以任意精度存储任何真实的。
///     Decimal,
///     ///存储表示时间长度的值。可以从日期时间或其他持续时间中添加或减去。
///     Duration,
///     ///将值存储在64位浮点数中。
///     Float,
///     ///将值存储为64位整数。
///     Int,
///     ///存储数字而不指定类型。SurrealDB将检测数字的类型，并使用最小的字节数存储它。对于以字符串形式传入的数字，此字段将数字存储在BigDecimal中。
///     Number,
///     ///存储包含任何受支持类型的值的格式化对象，对对象深度或嵌套没有限制。
///     Object,
///     String,
///     ///存储对另一个记录的引用。该值必须是记录ID。
///     Record,
///     ///RFC 7946 兼容的数据类型，用于在GeoJson格式.
///     Geometry(Geometry),
#[derive(Debug, Clone, PartialEq)]
pub enum ValueType {
    Any,
    Bool,
    Array,
    DateTime,
    Duration,
    Float,
    Int,
    Number,
    Object,
    String,
    Record,
    Geometry,
}

impl ValueType {
    pub fn to_str(&self) -> &str {
        match self {
            ValueType::Any => "any",
            ValueType::Bool => "bool",
            ValueType::Array => "array",
            ValueType::DateTime => "datetime",
            ValueType::Duration => "duration",
            ValueType::Float => "float",
            ValueType::Int => "int",
            ValueType::Number => "number",
            ValueType::Object => "object",
            ValueType::String => "string",
            ValueType::Record => "record",
            ValueType::Geometry => "geometry"
        }
    }
}

/// 用于构造Define Field 语句
/// 生成有模式的表
#[derive(Debug, Clone)]
pub struct ValueConstructor {
    value_type: ValueType,
    default_value: Option<SurrealValue>,
    assert: Option<Condition>,
}

impl ValueConstructor {
    pub fn new(value_type: ValueType, default_value: Option<SurrealValue>, assert: Option<Condition>) -> Self {
        //try to confirm value type == value's type
        if default_value.is_some() {
            if default_value.as_ref().unwrap().value_type().ne(&value_type) {
                panic!("value type match error!")
            }
        }
        ValueConstructor {
            value_type,
            default_value,
            assert,
        }
    }
    pub fn build(&self) -> String {
        let mut res = format!("TYPE {}", self.value_type.to_str());
        if self.default_value.is_some() {
            res.push_str(BLANK);
            res.push_str(format!("VALUE $value OR {}", self.default_value.as_ref().unwrap().to_str()).as_str());
        }
        if self.assert.is_some() {
            res.push_str(BLANK);
            res.push_str(format!("ASSERT {}", self.assert.as_ref().unwrap().build()).as_str());
        }
        res
    }
}