//! Core SQL Tools
//! ```txt
//! @author:syf20020816@Outlook.com
//! @date:2023/8/4
//! @version:0.0.1
//! @description:
//! ```
use std::fmt::{Display, Formatter};
use serde::{Serialize, Deserialize};
use super::constants::{AFTER, BEFORE, NONE, DIFF, MINUTE, MILLISECOND, SECOND, HOUR, DAY, TIMEOUT, RETURN, ADD_OP, MINUS_OP, DIVIDE_OP, PLUS_OP, EQ};
use crate::db::{SurrealID, ParamCombine};
use crate::db::constants::{MICROSECOND, NANOSECOND, YEAR};

/// # build Table with ID
/// If you don't want to specify the type, you can create it directly using `new_into()`
/// > `Table::<String>::new_into("temperature", "['London', 'New York']").build();`
/// ## example
/// ```rust
/// use surrealism::db::{SurrealID,SurrealValue,Table,Array,Object,Range,ParamCombine};
///     let id1 = SurrealID::RAND;
///     let id2 = SurrealID::Default;
///     let id3 = SurrealID::Str(String::from("surrealism"));
///     let id4 = SurrealID::Int(56_i64);
///     let id5 = SurrealID::Float(45.5454647_f64);
///     let id6 = SurrealID::Array(Array::from(vec![SurrealValue::Str(String::from("John")), SurrealValue::Str(String::from("Mat"))]));
///     let user = User {
///         name: "Mat",
///         age: 16,
///     };
///     let id7 = SurrealID::Object(Object::from_obj(&user));
///     let id8 = SurrealID::Range(Range::new_from_str("2", "6", true));
///     let id9 = SurrealID::from("ulid()");
///
///     let table1 = Table::new_no_arg()
///         .table("surrealism")
///         .id(id1)
///         .build();
///
///     let table2 = Table::new_no_arg()
///         .table("surrealism")
///         .id(id4)
///         .build();
///     let table3 = Table::new("surrealism", id6).combine();
///     let table4 = Table::new_into("surrealdb", "2..6").combine();
/// ```
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Table {
    name: String,
    id: SurrealID,
}

impl Default for Table {
    fn default() -> Self {
        Table {
            name: String::new(),
            id: SurrealID::default(),
        }
    }
}

impl Table {
    pub fn new(table_name: &str, table_id: SurrealID) -> Table {
        Table {
            name: String::from(table_name),
            id: table_id,
        }
    }
    /// build a table param with name and id freely
    pub fn new_into(table_name: &str, table_id: &str) -> Table {
        Table {
            name: String::from(table_name),
            id: SurrealID::String(String::from(table_id)),
        }
    }
    pub fn new_no_arg() -> Table {
        Table::default()
    }
    /// build table name
    pub fn table(&mut self, table_name: &str) -> &mut Self {
        self.name = String::from(table_name);
        self
    }
    /// build table special id
    pub fn id(&mut self, table_id: SurrealID) -> &mut Self {
        self.id = table_id;
        self
    }
    /// after appoint table name and id , this function will return a complete String like:
    /// > user:1006
    pub fn build(&self) -> String {
        let table_stmt = match self.id {
            SurrealID::Default => format!("{}", &self.name),
            _ => format!("{}:{}", &self.name, &self.id.combine())
        };
        table_stmt
    }
}

impl ParamCombine for Table {
    fn combine(&self) -> String {
        self.build()
    }
}

impl Display for Table {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.build())
    }
}

/// 数字类型
/// 主要用于直接获取Math提供的常量
pub struct MATH;

impl MATH {
    pub const E: f32 = 2.718281828459045;
    pub const FRAC_1_PI: f32 = 0.3183098861837907;
    pub const FRAC_1_SQRT_2: f32 = 0.7071067811865476;
    pub const FRAC_2_PI: f32 = 0.6366197723675814;
    pub const FRAC_2_SQRT_PI: f32 = 1.1283791670955126;
    pub const FRAC_PI_2: f32 = 1.5707963267948966;
    pub const FRAC_PI_3: f32 = 1.0471975511965979;
    pub const MFRAC_PI_4: f32 = 0.7853981633974483;
    pub const FRAC_PI_6: f32 = 0.5235987755982989;
    pub const FRAC_PI_8: f32 = 0.39269908169872414;
    pub const LN_10: f32 = 2.302585092994046;
    pub const LN_2: f32 = 0.6931471805599453;
    pub const LOG10_2: f32 = 0.3010299956639812;
    pub const LOG10_E: f32 = 0.4342944819032518;
    pub const LOG2_10: f32 = 3.321928094887362;
    pub const LOG2_E: f32 = 1.4426950408889634;
    pub const PI: f32 = 3.141592653589793;
    pub const SQRT_2: f32 = 1.4142135623730951;
    pub const TAU: f32 = 6.283185307179586;
}

/// 设置返回类型枚举
/// ## example
/// ```rust
/// use surrealism::db::{ParamCombine, SurrealismRes, ReturnType};
///     let return_none = ReturnType::None;
///     let return_before = ReturnType::Before;
///     let return_after = ReturnType::After;
///     let return_field = ReturnType::Field("name");
///     dbg!(&return_none);
///     dbg!(&return_before);
///     dbg!(&return_after);
///     dbg!(&return_field);
///     dbg!(&return_field.combine());
///     dbg!(&return_after.combine());
/// ```
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ReturnType {
    After,
    Before,
    None,
    Diff,
    Field(&'static str),
}

impl ReturnType {
    pub fn to_str(&self) -> &str {
        match self {
            ReturnType::After => AFTER,
            ReturnType::Before => BEFORE,
            ReturnType::None => NONE,
            ReturnType::Diff => DIFF,
            ReturnType::Field(target) => *target
        }
    }
}

impl ParamCombine for ReturnType {
    fn combine(&self) -> String {
        format!("{} {}", RETURN, &self.to_str())
    }
}

///在SurrealDB数据库中，Timeout子句可以用于设置查询的超时时间。它接受一个时间间隔作为参数，并支持以下单位：
/// - ns：纳秒
/// - us | µs ：微秒
/// - ms：毫秒
/// - s：秒
/// - m：分钟
/// - h：小时
/// - d：天
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum TimeUnit {
    NANOSECOND,
    MICROSECOND,
    MILLISECOND,
    SECOND,
    MINUTE,
    HOUR,
    DAY,
    Year,
}

impl Default for TimeUnit {
    fn default() -> Self {
        TimeUnit::MILLISECOND
    }
}

impl Display for TimeUnit {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_str())
    }
}

impl TimeUnit {
    pub fn to_str(&self) -> &str {
        match self {
            TimeUnit::MILLISECOND => MILLISECOND,
            TimeUnit::SECOND => SECOND,
            TimeUnit::MINUTE => MINUTE,
            TimeUnit::HOUR => HOUR,
            TimeUnit::DAY => DAY,
            TimeUnit::NANOSECOND => NANOSECOND,
            TimeUnit::MICROSECOND => MICROSECOND,
            TimeUnit::Year => YEAR
        }
    }
    pub fn get_unit(&self) -> &str {
        match self {
            TimeUnit::MILLISECOND => "ms",
            TimeUnit::SECOND => "s",
            TimeUnit::MINUTE => "m",
            TimeUnit::HOUR => "h",
            TimeUnit::DAY => "d",
            TimeUnit::NANOSECOND => "ns",
            TimeUnit::MICROSECOND => "us",
            TimeUnit::Year => "y"
        }
    }
}

/// # TimeOut for Wrapper
/// ## example
/// ```rust
/// use surrealism::db::{TimeOut,TimeUnit,ParamCombine};
///     let timeout = TimeOut::new(56_usize, TimeUnit::MINUTE);
///     dbg!(&timeout.timeout());
///     dbg!(&timeout.unit());
///     dbg!(&timeout);
///     dbg!(timeout.combine());
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TimeOut {
    timeout: Option<usize>,
    unit: TimeUnit,
}

impl Default for TimeOut {
    fn default() -> Self {
        TimeOut {
            timeout: None,
            unit: TimeUnit::MILLISECOND,
        }
    }
}

impl TimeOut {
    pub fn new(timeout: usize, unit: TimeUnit) -> Self {
        TimeOut {
            timeout: Some(timeout),
            unit,
        }
    }
    pub fn new_no_args() -> Self {
        TimeOut {
            timeout: None,
            unit: TimeUnit::SECOND,
        }
    }
    pub fn timeout(&self) -> Option<usize> {
        self.timeout
    }
    pub fn unit(&self) -> TimeUnit {
        self.unit.clone()
    }
    pub fn set_timeout(&mut self, timeout: usize) -> &mut Self {
        self.timeout = Some(timeout);
        self
    }
    pub fn set_unit(&mut self, unit: TimeUnit) -> &mut Self {
        self.unit = unit;
        self
    }
}

impl ParamCombine for TimeOut {
    fn combine(&self) -> String {
        self.to_string()
    }
}

impl Display for TimeOut {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {}{}", TIMEOUT, &self.timeout().unwrap(), &self.unit().get_unit())
    }
}

/// # Geometries
///
/// SurrealDB makes working with GeoJSON easy, with support for Point, Line, Polygon, MultiPoint, MultiLine, MultiPolygon, and Collection values. SurrealQL automatically detects GeoJSON objects converting them into a single data type.
///
/// SurrealDB使使用GeoJSON变得简单，支持Point、Line、Polygon、MultiPoint、MultiLine、MultiPolygon，以及Collection SurrealQL自动检测GeoJSON对象，将其转换为单一数据类型。
/// ```code
/// Point :	A geolocation point with latitude and longitude
/// Line :	A GeoJSON LineString value for storing a geometric path
/// Polygon :	A GeoJSON Polygon value for storing a geometric area
/// MultiPoint :	A value which contains multiple geometry points
/// MultiLine :	A value which contains multiple geometry lines
/// MultiPolygon :	A value which contains multiple geometry polygons
/// Collection :	A value which contains multiple different geometry types
/// ```
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum Geometry {
    Point,
    Line,
    Polygon,
    MultiPoint,
    MultiLine,
    MultiPolygon,
    Collection,
}

/// Operator运算符 for Create | Update ...
/// - Add : +=
/// - Minus : -=
/// - Plus : *=
/// - Divide : /=
/// - Eq : =
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum Operator {
    Add,
    Minus,
    Plus,
    Divide,
    Eq,
}

impl Operator {
    pub fn to_str(&self) -> &str {
        match self {
            Operator::Add => ADD_OP,
            Operator::Minus => MINUS_OP,
            Operator::Plus => PLUS_OP,
            Operator::Divide => DIVIDE_OP,
            Operator::Eq => EQ,
        }
    }
}