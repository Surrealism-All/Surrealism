use surrealdb::engine::remote::ws::Client;
use surrealdb::Surreal;
use serde::{Deserialize, Serialize};

///==========================构建语句所需的常量================================
pub const COMMON_SEPARATOR: &'static str = " ";
pub const IS_SEPARATOR: &'static str = ":";
pub const END_SEPARATOR: &'static str = ";";
pub const EQUAL_SEPARATOR: &'static str = "=";
pub const NEXT_SEPARATOR: &'static str = ",";
pub const EQ:&'static str = "=";
pub const NEQ:&'static str = "!=";
pub const GT:&'static str = ">";
pub const LT:&'static str = "<";
pub const GTE:&'static str = ">=";
pub const LTE:&'static str = "<=";

pub const USE: &'static str = "USE";
pub const NS: &'static str = "NS";
pub const DB: &'static str = "DB";
pub const CREATE: &'static str = "CREATE";
pub const RETURN: &'static str = "RETURN";
pub const NONE: &'static str = "NONE";
pub const DIFF: &'static str = "DIFF";
pub const BEFORE: &'static str = "BEFORE";
pub const AFTER: &'static str = "AFTER";
pub const CONTENT: &'static str = "CONTENT";
pub const SET: &'static str = "SET";
pub const UUID: &'static str = "uuid()";
pub const ULID: &'static str = "ulid()";
pub const RAND: &'static str = "rand()";
pub const SELECT: &'static str = "SELECT";
pub const FROM: &'static str = "FROM";
pub const AS: &'static str = "AS";
pub const WHERE: &'static str = "WHERE";

///SurrealCore是应用核心结构体，连接使用的是Surreal<Client>
/// operator: SurrealOperator 暂时并未有任何具体有用实现
pub struct SurrealCore {
    pub cn: Surreal<Client>,
    pub operator: SurrealOperator,
}

impl SurrealCore {
    pub fn new(cn: Surreal<Client>) -> Self {
        SurrealCore {
            cn,
            operator: SurrealOperator::new(),
        }
    }
}

/// 语句枚举
/// 考虑结合包装器,也许可以将包装器的keyword字段使用Statements枚举
enum Statements {
    USE,
    LET,
    BEGIN,
    CANCEL,
    COMMIT,
    IF_ELSE,
    SELECT,
    INSERT,
    CREATE,
    UPDATE,
    RELATE,
    DELETE,
    DEFINE,
    REMOVE,
    INFO,
    SLEEP,
}

pub struct SurrealOperator {}

impl SurrealOperator {
    pub fn new() -> Self {
        SurrealOperator {}
    }
}


///所有包装器都需要实现这个顶级包装器trait
/// new:创建一个新包装器
/// and:语句连接,当语句进入一个新区域时,使用and(),例如: USE NS xxx DB xxx; => UseWrapper::new().use_ns("xxx")`.and()下个方法进入新区域`.use_db("xxx")
/// build:当所有构建结束后使用build进行完整的构建,否则导致语句不完整
/// commit:语句提交
/// get_keyword:获取keyword
/// get_available:获取可用参数Map
pub trait Wrapper {
    fn new() -> Self;
    fn commit(&mut self) -> &str;
    fn get_keyword(&self) -> &str;
    fn get_available(&self) -> &Vec<AvailData>;
}

///语句可用参数
/// order:语句顺序
/// key:语句键
/// value:语句值
/// repeat:是否可重复
/// end:结束标识,end = true 说明语句结束
#[derive(Debug, Clone)]
pub struct AvailData {
    order: usize,
    key: String,
    value: String,
    repeat: bool,
    end: bool,
}

///语句可用参数方法实现
impl AvailData {
    pub fn new(
        order: usize,
        key: String,
        value: String,
        repeat: bool,
        end: bool,
    ) -> Self {
        AvailData {
            order,
            key,
            value,
            repeat,
            end,
        }
    }

    pub fn new_no_args() -> Self {
        AvailData {
            order: 0,
            key: String::new(),
            value: String::new(),
            repeat: true,
            end: false,
        }
    }
    pub fn order(&self) -> usize {
        self.order
    }
    pub fn key(&self) -> &str {
        &self.key
    }
    pub fn value(&self) -> &str {
        &self.value
    }
    pub fn repeat(&self) -> bool {
        self.repeat
    }
    pub fn end(&self) -> bool {
        self.end
    }
    pub fn set_order(&mut self, order: usize) {
        self.order = order;
    }
    pub fn set_key(&mut self, key: String) {
        self.key = key;
    }
    pub fn set_value(&mut self, value: String) {
        self.value = value;
    }
    pub fn set_repeat(&mut self, repeat: bool) {
        self.repeat = repeat;
    }
    pub fn set_end(&mut self, end: bool) {
        self.end = end;
    }
}


///TableID枚举
///当使用CreateWrapper中的id()方法,该方法的入参需要使用TableID enum进行指定
/// Num: 数字类型
/// Str: 字符串类型
/// Object: 对象类型
/// Array: 数组类型
/// Range: 范围类型(使用IdRange进行指定)
/// Fun: 对应Surreal的内置生成ID的方法,包含:rand(),uuid(),ulid()三种
#[derive(Debug, Clone, Serialize)]
pub enum TableId<T: Serialize> {
    Num(isize),
    Str(String),
    Object(T),
    Array(Vec<T>),
    Range {
        min: IdRange<T>,
        max: IdRange<T>,
    },
    Fun(IdFunction),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum IdFunction {
    UUID,
    ULID,
    RAND,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum IdRange<T> {
    Num(isize),
    Arr(Vec<T>),
}


pub trait RegionImpl{
    fn combine(&mut self)->&str;
}

