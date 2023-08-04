//!
//! ```txt
//! @author:syf20020816@Outlook.com
//! @date:2023/8/4
//! @version:0.0.1
//! @description:
//! ```
use super::{AFTER, BEFORE, NONE, DIFF};
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Table<T:Serialize> {
    name: String,
    id: SurrealID<T>,
}

impl<T:Serialize> Table<T> {
    pub fn new(table_name: &str, table_id: SurrealID<T>) -> Table<T> {
        Table {
            name: String::from(table_name),
            id: table_id,
        }
    }
    pub fn new_no_arg() -> Table<T> {
        Table {
            name: String::new(),
            id: SurrealID::Default,
        }
    }
    pub fn table(&mut self, table_name: &str) -> &mut Self {
        self.name = String::from(table_name);
        self
    }
    pub fn id(&mut self, table_id: SurrealID<T>) -> &mut Self {
        self.id = table_id;
        self
    }
    pub fn build(&mut self) -> String {
        let mut table_stmt = String::new();
        match self.id {
            SurrealID::Default => table_stmt = format!("{}", &self.name),
            _ => table_stmt = format!("{}:{}", &self.name, &self.id.to_str())
        };
        table_stmt
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub enum SurrealID<T:Serialize> {
    Default,
    Number(IDNumber),
    Str(String),
    Object(T),
    Array(Vec<T>),
}

impl<T: Serialize> SurrealID<T> {
    pub fn to_str(&self) -> String {
        match self {
            SurrealID::Default => String::new(),
            SurrealID::Number(num) => {
                num.to_str()
            }
            SurrealID::Str(s) => String::from(s),
            SurrealID::Object(obj) => serde_json::to_string(obj).unwrap(),
            SurrealID::Array(arr) => serde_json::to_string(arr).unwrap()
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub enum IDNumber {
    Int(i32),
    Float(f32),
    Decimal(f64),
}

impl IDNumber {
    pub fn to_str(&self) -> String {
        match self {
            IDNumber::Int(int_num) => int_num.to_string(),
            IDNumber::Float(float_num) => float_num.to_string(),
            IDNumber::Decimal(decimal_num) => decimal_num.to_string()
        }
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

pub enum ReturnType<'a> {
    After,
    Before,
    None,
    Diff,
    Field(&'a str),
}

impl<'a> ReturnType<'a> {
    fn to_str(&self) -> &str {
        match self {
            ReturnType::After => AFTER,
            ReturnType::Before => BEFORE,
            ReturnType::None => NONE,
            ReturnType::Diff => DIFF,
            ReturnType::Field(target) => *target
        }
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
pub enum Geometry {
    Point,
    Line,
    Polygon,
    MultiPoint,
    MultiLine,
    MultiPolygon,
    Collection,
}