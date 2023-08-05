//! Core SQL Tools
//! ```txt
//! @author:syf20020816@Outlook.com
//! @date:2023/8/4
//! @version:0.0.1
//! @description:
//! ```
use super::{AFTER, BEFORE, NONE, DIFF, UUID, ULID, RAND};
use serde::{Serialize, Deserialize};

/// # build Table with ID
/// If you don't want to specify the type, you can create it directly using `new_into()`
/// > `Table::<String>::new_into("temperature", "['London', 'New York']").build();`
/// ## example
/// ```code
///     let table1 = Table::new("test", SurrealID::<String>::Str("surrealdb".to_string())).build();
///     let table2 = Table::new_no_arg().table("temperature").id(SurrealID::<IDNumber>::Number(IDNumber::Int(17493))).build();
///     let table3 = Table::<String>::new_into("temperature", "['London', 'New York']").build();
///     let table4 = Table::new("user", SurrealID::<String>::RAND).build();
/// ```
#[derive(Debug, Serialize, Deserialize)]
pub struct Table<T: Serialize> {
    name: String,
    id: SurrealID<T>,
}

impl<T: Serialize> Table<T> {
    pub fn new(table_name: &str, table_id: SurrealID<T>) -> Table<T> {
        Table {
            name: String::from(table_name),
            id: table_id,
        }
    }
    /// build a table param with name and id freely
    pub fn new_into(table_name: &str, table_id: &str) -> Table<String> {
        Table {
            name: String::from(table_name),
            id: SurrealID::Str(String::from(table_id)),
        }
    }
    pub fn new_no_arg() -> Table<T> {
        Table {
            name: String::new(),
            id: SurrealID::Default,
        }
    }
    /// build table name
    pub fn table(&mut self, table_name: &str) -> &mut Self {
        self.name = String::from(table_name);
        self
    }
    /// build table special id
    pub fn id(&mut self, table_id: SurrealID<T>) -> &mut Self {
        self.id = table_id;
        self
    }
    /// after appoint table name and id , this function will return a complete String like:
    /// > user:1006
    pub fn build(&self) -> String {
        let mut table_stmt = String::new();
        match self.id {
            SurrealID::Default => table_stmt = format!("{}", &self.name),
            _ => table_stmt = format!("{}:{}", &self.name, &self.id.to_str())
        };
        table_stmt
    }
}

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
#[derive(Debug, Serialize, Deserialize)]
pub enum SurrealID<T: Serialize> {
    Default,
    Number(IDNumber),
    Str(String),
    Object(T),
    Array(Vec<T>),
    UUID,
    ULID,
    RAND,
}

impl<T: Serialize> SurrealID<T> {
    /// Convert SurrealID to String
    pub fn to_str(&self) -> String {
        match self {
            SurrealID::Default => String::new(),
            SurrealID::Number(num) => {
                num.to_str()
            }
            SurrealID::Str(s) => String::from(s),
            SurrealID::Object(obj) => serde_json::to_string(obj).unwrap(),
            SurrealID::Array(arr) => serde_json::to_string(arr).unwrap(),
            SurrealID::ULID => ULID.to_string(),
            SurrealID::UUID => UUID.to_string(),
            SurrealID::RAND => RAND.to_string(),
        }
    }
}

/// SurrealID‘s Number Type Enum
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