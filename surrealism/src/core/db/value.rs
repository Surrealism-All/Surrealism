//! # SurrealDB对应值的类型
//! ## example
//! ```rust
//! use std::collections::HashMap;
//! use surrealism::db::{ SurrealID, handle_str, Array, Object, SurrealValue};
//! use surrealism::surreal::SurrealismRes;
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
//!
use std::collections::{BTreeMap, HashMap};
use std::fmt::{Display, Formatter};
use std::ops::Deref;
use num_traits::cast::FromPrimitive;
use std::str::FromStr;
use serde::{Serialize, Deserialize};
use serde_json::Value;
use surrealdb::sql::{Duration, Datetime};
use crate::db::{Condition, Geometries, Decimal, DurationAdapter, DatetimeAdapter};
use crate::core::db::constants::{BLANK};
use crate::db::constants::SET_DOWN;
use crate::util::{remove_format_half, handle_str};
use super::constants::{OWNER, EDITOR, VIEWER, NULL, NULL_DOWN, NONE_DOWN, NONE, LEFT_BRACE, RIGHT_BRACE, COMMA, ANY, BOOL, ARRAY, DATETIME, DURATION, NUMBER, INT, FLOAT, STRING, OBJECT, GEOMETRY, RECORD, DECIMAL};

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
    Int(i64),
    Float(f64),
    Decimal(Decimal),
    Option(Option<Box<SurrealValue>>),
    String(String),
    Object(Object),
    Array(Array),
    Set(Array),
    Geometries(Geometries),
}

impl Display for SurrealValue {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let res = match self {
            SurrealValue::None => NONE.to_string(),
            SurrealValue::Null => NULL.to_string(),
            SurrealValue::Bool(b) => b.to_string(),
            SurrealValue::Int(n) => n.to_string(),
            SurrealValue::Float(f) => f.to_string(),
            SurrealValue::Decimal(d) => d.to_string(),
            SurrealValue::String(s) => handle_str(serde_json::to_string(s).unwrap().as_str()),
            SurrealValue::Object(obj) => obj.parse(),
            SurrealValue::Array(arr) => arr.parse(),
            SurrealValue::Set(set) => set.parse(),
            SurrealValue::DateTime(time) => time.to_string(),
            SurrealValue::Duration(duration) => duration.to_string(),
            SurrealValue::Record(record) => format!("record({})", record),
            SurrealValue::Geometries(geo) => geo.to_string(),
            _ => String::new()
        };
        write!(f, "{}", res)
    }
}

impl SurrealValue {
    /// # build Geo
    /// SurrealDB使使用GeoJSON变得简单，支持Point、Line、Polygon、MultiPoint、MultiLine、MultiPolygon、Collection
    ///
    /// 在Surrealism中会进行第一道检测，但若遇到大量数据请使用后缀为unchecked方法
    ///
    /// SurrealQL自动检测GeoJSON对象，将其转换为单一数据类型。
    /// ## example
    /// ```rust
    /// use surrealism::DefaultRes;
    /// use surrealism::db::{Geometries, AdapterToValue, Point, SurrealValue};
    /// #[tokio::main]
    /// async fn main() -> DefaultRes<()> {
    ///     let geo_point = SurrealValue::geo().point(12.0, 16.6).to_value();
    ///     let geo_line = SurrealValue::geo()
    ///         .line(vec![Point::new(10.0, 11.2), Point::from((11.0, 11.2))])
    ///         .unwrap();
    ///     let geo_polygon = SurrealValue::geo()
    ///         .polygon(vec![Point::new(10.0, 11.2), Point::from((11.0, 11.2)), Point::new(10.0, 11.2)])
    ///         .unwrap()
    ///         .to_value();
    ///     let geo_multi_points = SurrealValue::geo()
    ///         .multi_point(vec![Point::new(10.0, 11.2), Point::from((11.0, 11.2))])
    ///         .to_value();
    ///     let geo_multi_line = SurrealValue::geo().multi_line(
    ///         vec![
    ///             vec![Point::from((10.0, 11.2)), Point::from((10.5, 11.9))],
    ///             vec![Point::from((11.0, 12.2)), Point::from((11.5, 12.9)), Point::from((12.0, 13.0))],
    ///         ]
    ///     ).unwrap();
    ///     let geo_multi_polygon = SurrealValue::geo().multi_polygon(
    ///         vec![
    ///             vec![Point::from((9.0, 11.2)), Point::from((10.5, 11.9)), Point::from((10.3, 13.0)), Point::from((9.0, 11.2))],
    ///             vec![Point::from((10.0, 11.2)), Point::from((10.5, 11.9)), Point::from((10.8, 12.0)), Point::from((10.0, 11.2))],
    ///         ]
    ///     ).unwrap().to_value();
    ///     let collection = SurrealValue::geo().collections(
    ///         vec![geo_line.clone(),geo_multi_line.clone()]
    ///     ).to_value();
    ///     Ok(())
    /// }
    /// ```
    pub fn geo() -> Geometries {
        Geometries::Default
    }
    /// # build Datetime , GMT 0
    /// format just like : `2023-09-10T23:13:23.520847500Z`
    /// ## example
    /// use this way you will get SurrealValue::DateTime()
    ///
    /// `let datetime = SurrealValue::datetime().default().to_value();`
    pub fn datetime() -> DatetimeAdapter {
        DatetimeAdapter
    }
    /// # build Duration (surreal::sql::Duration)
    /// 通过调用该方法获取一个DurationAdapter帮助使用者间接构建Duration!👍
    /// ## example
    /// ```rust
    /// use surrealism::DefaultRes;
    /// use surrealism::db::{SurrealValue};
    ///
    /// // [tests\src\main.rs:11] dur_day.to_string() = "1d"
    /// // [tests\src\main.rs:12] dur_weak.to_string() = "12w"
    /// // [tests\src\main.rs:13] dur_min.to_string() = "20m"
    /// #[tokio::main]
    /// async fn main() -> DefaultRes<()> {
    ///     let dur_day = SurrealValue::duration().from_days(1);
    ///     let dur_weak = SurrealValue::duration().from_weeks(12);
    ///     let dur_min  = SurrealValue::duration().from_mins(20);
    ///     dbg!(dur_day.to_string());
    ///     dbg!(dur_weak.to_string());
    ///     dbg!(dur_min.to_string());
    ///     Ok(())
    /// }
    /// ```
    pub fn duration() -> DurationAdapter {
        DurationAdapter
    }
    /// # build None
    /// 值可以专门设置为NONE 在SurrealDB中从记录中删除字段
    /// ## example
    /// ```rust
    /// use surrealism::DefaultRes;
    /// use surrealism::db::{SurrealValue};
    ///
    /// // [tests\src\main.rs:10] none1.to_string() = "NONE"
    /// // [tests\src\main.rs:11] none2.to_string() = "NONE"
    /// // [tests\src\main.rs:12] none3.to_string() = "NONE"
    /// #[tokio::main]
    /// async fn main() -> DefaultRes<()> {
    ///     let none1 = SurrealValue::none();
    ///     let none2 = SurrealValue::from("None");
    ///     let none3:SurrealValue = "NONE".into();
    ///     dbg!(none1.to_string());
    ///     dbg!(none2.to_string());
    ///     dbg!(none3.to_string());
    ///     Ok(())
    /// }
    /// ```
    pub fn none() -> Self {
        SurrealValue::None
    }
    /// # build Null
    /// 值可以专门设置为NULL 在SurrealDB中表示已设置但没有值的字段。
    /// ## example
    /// ```rust
    /// use surrealism::DefaultRes;
    /// use surrealism::db::{SurrealValue};
    ///
    /// // [tests\src\main.rs:12] null1.to_string() = "NULL"
    /// // [tests\src\main.rs:13] null2.to_string() = "NULL"
    /// // [tests\src\main.rs:14] null3.to_string() = "NULL"
    /// #[tokio::main]
    /// async fn main() -> DefaultRes<()> {
    ///     let null1 = SurrealValue::null();
    ///     let null2 = SurrealValue::from("Null");
    ///     let null3:SurrealValue = "NULL".into();
    ///     dbg!(null1.to_string());
    ///     dbg!(null2.to_string());
    ///     dbg!(null3.to_string());
    ///     Ok(())
    /// }
    /// ```
    pub fn null() -> Self { SurrealValue::Null }
    /// # build Bool
    /// 布尔值可用于标记字段是否为true或false
    /// ## example
    /// ```rust
    ///use surrealism::DefaultRes;
    /// use surrealism::db::{SurrealValue};
    ///
    /// // [tests\src\main.rs:10] bool1.to_string() = "true"
    /// // [tests\src\main.rs:11] bool2.to_string() = "false"
    /// // [tests\src\main.rs:12] bool3.to_string() = "true"
    /// #[tokio::main]
    /// async fn main() -> DefaultRes<()> {
    ///     let bool1 = SurrealValue::Bool(true);
    ///     let bool2:SurrealValue = false.into();
    ///     let bool3 = SurrealValue::bool(true);
    ///     Ok(())
    /// }
    /// ```
    pub fn bool(b: bool) -> Self {
        SurrealValue::Bool(b)
    }
    /// # build String
    /// 字符串可用于存储文本值。所有文本字段都可以包含Unicode值、表情符号以及表格和多行分隔符。
    /// 使用该方法不会涉及隐式转换，但若您希望使用一种省力的方式可以采用from方法或into方法，请注意from和into会涉及隐式转换
    /// ## example
    /// ```rust
    /// use surrealism::DefaultRes;
    /// use surrealism::db::{SurrealValue};
    /// #[tokio::main]
    /// async fn main() -> DefaultRes<()> {
    ///     // 请注意使用string方法创建的只可能是字符串
    ///     // 而是用from或from_json方法创建的可能会进行隐式转换
    ///     let s1 = SurrealValue::string("Lorem ipsum dolor sit amet");
    ///     let s2 = SurrealValue::from("I ❤️ SurrealDB");
    ///     //这里则会隐式转换为Int
    ///     let s3 =  SurrealValue::from("89");
    ///     // 这里会隐式转换为Object
    ///     let s4 = SurrealValue::from_json("{ \"address\": \"China - Shanghai\"}");
    ///     Ok(())
    /// }
    /// ```
    pub fn string(s: &str) -> Self {
        SurrealValue::String(String::from(s))
    }
    /// # build number int
    /// ## example
    /// ```rust
    /// use surrealism::DefaultRes;
    /// use surrealism::db::{SurrealValue};
    ///
    /// // [tests\src\main.rs:13] int_num1.to_string() = "2022"
    /// // [tests\src\main.rs:14] int_num2.to_string() = "2023"
    /// // [tests\src\main.rs:15] int_num3.to_string() = "2024"
    /// // [tests\src\main.rs:16] float_num1.to_string() = "41.5"
    /// // [tests\src\main.rs:17] float_num2.to_string() = "56.23"
    /// // [tests\src\main.rs:18] decimal_num1.to_string() = "99.99dec"
    /// // [tests\src\main.rs:19] decimal_num2.to_string() = "564.22dec"
    /// #[tokio::main]
    /// async fn main() -> DefaultRes<()> {
    ///     let int_num1 = SurrealValue::Int(2022);
    ///     let int_num2 = SurrealValue::int(2023);
    ///     let int_num3:SurrealValue = 2024.into();
    ///     let float_num1 = SurrealValue::float(41.5);
    ///     let float_num2:SurrealValue = 56.23.into();
    ///     let decimal_num1 = SurrealValue::decimal(99.99);
    ///     let decimal_num2 = SurrealValue::decimal_str("564.22");
    ///     Ok(())
    /// }
    ///```
    pub fn int(num: i64) -> Self {
        SurrealValue::Int(num)
    }
    /// # build number float
    /// ## example
    /// ```code
    ///     let float_num1 = SurrealValue::float(41.5);
    ///     let float_num2:SurrealValue = 56.23.into();
    /// ```
    pub fn float(num: f64) -> Self {
        SurrealValue::Float(num)
    }
    /// # build number decimal
    ///
    /// recommend : decimal_str()
    ///
    /// 请注意decimal 无法使用into()进行推测
    ///
    /// decimal can not use into()
    pub fn decimal(num: f64) -> Self {
        SurrealValue::Decimal(Decimal::new(num.to_string().as_str()))
    }
    /// 更推荐创建decimal的方式
    pub fn decimal_str(value: &str) -> Self {
        SurrealValue::Decimal(Decimal::new(value))
    }
    pub fn record(id: &str) -> Self {
        SurrealValue::Record(id.to_string())
    }
    pub fn value_type(&self) -> ValueType {
        match self {
            SurrealValue::DateTime(_) => ValueType::DateTime,
            SurrealValue::Duration(_) => ValueType::Duration,
            SurrealValue::Record(_) => ValueType::Record,
            SurrealValue::Bool(_) => ValueType::Bool,
            SurrealValue::Int(_) => ValueType::Int,
            SurrealValue::Float(_) => ValueType::Float,
            SurrealValue::Decimal(_) => ValueType::Decimal,
            SurrealValue::String(_) | SurrealValue::None | SurrealValue::Null => ValueType::String,
            SurrealValue::Object(_) => ValueType::Object,
            SurrealValue::Array(_) => ValueType::Array,
            SurrealValue::Geometries(_) => ValueType::Geometry,
            SurrealValue::Option(_) => ValueType::Any,
            SurrealValue::Set(_) => ValueType::Set,
        }
    }

    /// # build from json str
    ///从json-str进行推测，转换为serde::Value再转为SurrealValue，请注意该方法会涉及隐式转换
    /// ## example
    /// ```rust
    /// use surrealism::DefaultRes;
    /// use surrealism::db::{SurrealValue};
    /// #[tokio::main]
    /// async fn main() -> DefaultRes<()> {
    ///     // 这里会隐式转换为Object
    ///     let s = SurrealValue::from_json("{ \"address\": \"China - Shanghai\"}");
    ///     Ok(())
    /// }
    /// ```
    pub fn from_json(value: &str) -> SurrealValue {
        let value_str: Value = serde_json::from_str(value).unwrap();
        let res: SurrealValue = value_str.into();
        res
    }
    /// # build Object
    /// SurrealDB记录可以存储对象，对任何嵌套对象或值的深度没有限制。
    /// 这意味着对象和数组可以相互存储，以便对复杂的数据场景进行建模。
    /// 对象可以存储其中存储的任何值，并且可以在同一对象中存储不同的值类型。
    /// ## example
    /// ```rust
    /// use surrealism::DefaultRes;
    /// use surrealism::db::{SurrealValue, Object};
    /// use serde::{Serialize, Deserialize};
    ///
    /// #[derive(Debug, Clone, Serialize, Deserialize)]
    /// struct User<'a> {
    ///     name: &'a str,
    ///     age: u8,
    /// }
    ///
    /// impl<'a> User<'a> {
    ///     pub fn new() -> Self { User { name: "Matt", age: 16 } }
    /// }
    ///
    /// // [tests\src\main.rs:20] obj1.to_string() = "{ age : 16 , name : 'Matt' }"
    /// // [tests\src\main.rs:21] obj2.to_string() = "{ age : 16 , name : 'Matt' }"
    /// #[tokio::main]
    /// async fn main() -> DefaultRes<()> {
    ///     let user = User::new();
    ///     let obj1 = SurrealValue::object(&user);
    ///     let obj2 = SurrealValue::from(Object::from_obj(&user));
    ///     dbg!(obj1.to_string());
    ///     dbg!(obj2.to_string());
    ///     Ok(())
    /// }
    /// ```
    pub fn object<T>(obj: &T) -> Self where T: Serialize {
        Object::to_value(obj)
    }
    /// # build Array
    /// SurrealDB中的记录可以存储值的数组，对数组的深度没有限制。
    /// 数组可以存储其中存储的任何值，并且可以在同一数组中存储不同的值类型。
    /// > 采用surrealism::db::Array;可以创建更加灵活的构建方式！
    /// ## example
    /// ```rust
    /// use surrealism::DefaultRes;
    /// use surrealism::db::{SurrealValue, Object, AdapterToValue};
    /// use serde::{Serialize, Deserialize};
    ///
    /// #[derive(Debug, Clone, Serialize, Deserialize)]
    /// struct User<'a> {
    ///     name: &'a str,
    ///     age: u8,
    /// }
    ///
    /// #[tokio::main]
    /// async fn main() -> DefaultRes<()> {
    ///     let arr1 = SurrealValue::array(vec![2,3,5,6]);
    ///     let arr2 = SurrealValue::array(
    ///         vec![
    ///             User{name:"John",age:1},
    ///             User{name:"Matt",age:13},
    ///         ]
    ///     );
    ///     Ok(())
    /// }
    /// ```
    pub fn array<T>(arr: Vec<T>) -> Self where T: Serialize {
        SurrealValue::Array(Array::from(SurrealValue::from_vec(arr)))
    }
    pub fn is_none(&self) -> bool {
        match self {
            SurrealValue::None => true,
            _ => false
        }
    }
    pub fn is_duration(&self) -> bool {
        match self {
            SurrealValue::Duration(_) => true,
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
            SurrealValue::String(_) => true,
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
    /// get str from SurrealValue::Str
    pub fn inner_str(&self) -> Option<String> {
        match self {
            SurrealValue::String(s) => Some(s.to_string()),
            _ => Some(self.to_string())
        }
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
                    SurrealValue::from(n.as_f64().unwrap())
                } else {
                    let num = n.as_u64().unwrap();
                    SurrealValue::from(num as i32)
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

impl From<Object> for SurrealValue {
    fn from(value: Object) -> Self {
        SurrealValue::Object(value)
    }
}

impl From<Geometries> for SurrealValue {
    fn from(value: Geometries) -> Self {
        SurrealValue::Geometries(value)
    }
}

impl From<&str> for SurrealValue {
    fn from(value: &str) -> Self {
        // 优先转化为Float
        // 考虑转化后再次尝试转化为Int
        match value.clone().parse::<f64>() {
            Ok(f) => {
                if f.fract() == 0.0 {
                    if let Some(i) = i64::from_f64(f) {
                        return SurrealValue::Int(i);
                    }
                }
                return SurrealValue::Float(f);
            }
            Err(_) => {
                //无法转换的情况
                match value.to_lowercase().as_str() {
                    NONE_DOWN => SurrealValue::None,
                    NULL_DOWN => SurrealValue::Null,
                    _ => SurrealValue::String(String::from(value))
                }
            }
        }
    }
}


impl From<bool> for SurrealValue {
    fn from(value: bool) -> Self {
        SurrealValue::Bool(value)
    }
}


impl From<i64> for SurrealValue {
    fn from(value: i64) -> Self {
        SurrealValue::Int(value)
    }
}

impl From<i32> for SurrealValue {
    fn from(value: i32) -> Self {
        SurrealValue::from(value as i64)
    }
}

impl From<u32> for SurrealValue {
    fn from(value: u32) -> Self {
        SurrealValue::from(value as i64)
    }
}

impl From<usize> for SurrealValue {
    fn from(value: usize) -> Self {
        SurrealValue::from(value as i64)
    }
}

impl From<isize> for SurrealValue {
    fn from(value: isize) -> Self {
        SurrealValue::from(value as i64)
    }
}

impl From<f32> for SurrealValue {
    fn from(value: f32) -> Self {
        SurrealValue::from(value as f64)
    }
}

impl From<f64> for SurrealValue {
    fn from(value: f64) -> Self {
        SurrealValue::Float(value)
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
/// use surrealism::db::{SurrealValue};
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
            res += format!("{} : {}", key, value.to_string()).as_str();
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
        // let obj_str: Value = serde_json::to_value(t).unwrap();
        // let res: SurrealValue = obj_str.into();
        match Object::to_value(t) {
            SurrealValue::Object(obj) => obj,
            _ => panic!("parse SurrealValue::Object failed"),
        }
    }
    pub fn to_value<T: Serialize>(t: &T) -> SurrealValue {
        let obj_str: Value = serde_json::to_value(t).unwrap();
        let res: SurrealValue = obj_str.into();
        res
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
            let item_str = item.to_string();
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
    Set,
    DateTime,
    Duration,
    Decimal,
    Float,
    Int,
    Number,
    Object,
    String,
    Record,
    Geometry,
    Option(Box<ValueType>),
}

impl ValueType {
    pub fn option(value_type: ValueType) -> Self {
        ValueType::Option(Box::new(value_type))
    }
    pub fn to_string(&self) -> String {
        let res = match self {
            ValueType::Any => ANY,
            ValueType::Bool => BOOL,
            ValueType::Array => ARRAY,
            ValueType::DateTime => DATETIME,
            ValueType::Duration => DURATION,
            ValueType::Float => FLOAT,
            ValueType::Int => INT,
            ValueType::Number => NUMBER,
            ValueType::Object => OBJECT,
            ValueType::String => STRING,
            ValueType::Record => RECORD,
            ValueType::Geometry => GEOMETRY,
            ValueType::Decimal => DECIMAL,
            ValueType::Option(v) => {
                return format!("option<{}>", v.to_string());
            }
            ValueType::Set => SET_DOWN
        };
        res.to_string()
    }
}

/// 用于构造Define Field 语句
/// 生成有模式的表
#[derive(Debug, Clone)]
pub struct ValueConstructor {
    flexible: bool,
    value_type: ValueType,
    default_value: Option<SurrealValue>,
    value: Option<SurrealValue>,
    assert: Option<Condition>,
}

impl ValueConstructor {
    pub fn new(value_type: ValueType, default_value: Option<SurrealValue>, value: Option<SurrealValue>, assert: Option<Condition>, flexible: bool) -> Self {
        //try to confirm value type == value's type
        if default_value.is_some() {
            if default_value.as_ref().unwrap().value_type().ne(&value_type) {
                panic!("value type match error!")
            }
        }
        ValueConstructor {
            flexible,
            value_type,
            default_value,
            value,
            assert,
        }
    }
    /// You do not need to specify value type , this function will help to infer value type by default value
    /// ## example
    /// ```code
    ///     let define_field3 = SQLBuilderFactory::define()
    ///         .field("countrycode", "user",
    ///         ValueConstructor::new_infer(Some(SurrealValue::from(true)),None),None).build();
    /// ```
    pub fn new_infer(default_value: Option<SurrealValue>, assert: Option<Condition>) -> Self {
        if default_value.is_some() {
            //guess type
            Self::new(default_value.as_ref().unwrap().value_type(), default_value, None, assert, false)
        } else {
            panic!("if you wanna use None | Null to specify value , you should use new function!")
        }
    }
    pub fn build(&self) -> String {
        let mut res = if self.flexible {
            String::from("FLEXIBLE ")
        } else { String::new() };
        res.push_str(format!("TYPE {}", self.value_type.to_string()).as_str());
        if self.default_value.is_some() {
            res.push_str(BLANK);
            res.push_str(format!("DEFAULT {}", self.default_value.as_ref().unwrap().to_string()).as_str());
        }
        if self.value.is_some() {
            res.push_str(BLANK);
            res.push_str(format!("VALUE {}", self.value.as_ref().unwrap().to_string()).as_str());
        }
        if self.assert.is_some() {
            res.push_str(BLANK);
            res.push_str(format!("ASSERT {}", self.assert.as_ref().unwrap().build()).as_str());
        }
        res
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum Number {
    Int(i32),
    Float(f32),
    Decimal(f64),
}

impl Number {
    /// convert Number to string
    pub fn parse(&self) -> String {
        match self {
            Number::Int(i) => i.to_string(),
            Number::Float(f) => f.to_string(),
            Number::Decimal(d) => d.to_string(),
        }
    }
    pub fn is_int(&self) -> bool {
        match self {
            Number::Int(_) => true,
            _ => false
        }
    }
    pub fn is_float(&self) -> bool {
        match self {
            Number::Float(_) => true,
            _ => false
        }
    }
    pub fn is_decimal(&self) -> bool {
        match self {
            Number::Decimal(_) => true,
            _ => false
        }
    }
}

impl From<i32> for Number {
    fn from(value: i32) -> Self {
        Number::Int(value)
    }
}

impl From<f32> for Number {
    fn from(value: f32) -> Self {
        Number::Float(value)
    }
}

impl From<f64> for Number {
    fn from(value: f64) -> Self {
        Number::Decimal(value)
    }
}

///# 角色
///目前，只有内置角色OWNER、EDITOR 和VIEWER 都是可用的。
///作用 	描述
/// OWNER 	可以查看和编辑用户级别或以下的任何资源，包括用户和令牌（IAM）资源。它还为支持`PERMISSIONS`子句的子资源（表、字段等）赠款完全权限。
/// EDITOR 	可以查看和编辑用户级别或更低级别的任何资源，但不能查看和编辑用户或令牌（IAM）资源它还为支持`PERMISSIONS`子句的子资源（表、字段等）赠款完全权限。
/// VIEWER 	授予赠款以查看用户级别或更低级别的任何资源，但不能进行编辑。它还为支持`PERMISSIONS`子句的子资源（表、字段等）赠款查看权限。
#[derive(Debug, Clone)]
pub enum Roles {
    OWNER,
    EDITOR,
    VIEWER,
}

impl Default for Roles {
    fn default() -> Self {
        Role::OWNER
    }
}

impl Display for Roles {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            Role::OWNER => OWNER,
            Role::EDITOR => EDITOR,
            Role::VIEWER => VIEWER,
        })
    }
}