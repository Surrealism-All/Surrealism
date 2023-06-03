use std::cell::RefCell;
use surrealdb::engine::remote::ws::Client;
use surrealdb::Surreal;
use serde::{Deserialize, Serialize};

///==========================构建语句所需的常量================================
pub const COMMON_SEPARATOR: &'static str = " ";
pub const IS_SEPARATOR: &'static str = ":";
pub const END_SEPARATOR: &'static str = ";";
pub const EQUAL_SEPARATOR: &'static str = " = ";
pub const NEXT_SEPARATOR: &'static str = " , ";
pub const EQ: &'static str = " = ";
pub const NEQ: &'static str = " != ";
pub const GT: &'static str = " > ";
pub const LT: &'static str = " < ";
pub const GTE: &'static str = " >= ";
pub const LTE: &'static str = " <= ";
pub const USE: &'static str = "USE";
pub const NS: &'static str = "NS";
pub const DB: &'static str = "DB";
pub const CREATE: &'static str = "CREATE";
pub const RETURN: &'static str = "RETURN";
pub const NONE: &'static str = "NONE";
pub const DIFF: &'static str = "DIFF";
pub const BEFORE: &'static str = "BEFORE";
pub const AFTER: &'static str = "AFTER";
pub const WHERE: &'static str = "WHERE";
pub const CONTENT: &'static str = "CONTENT";
pub const SET: &'static str = "SET";
pub const UUID: &'static str = "uuid()";
pub const ULID: &'static str = "ulid()";
pub const RAND: &'static str = "rand()";
pub const SELECT: &'static str = "SELECT";
pub const FROM: &'static str = "FROM";
pub const AS: &'static str = "AS";
pub const ORDER_BY: &'static str = "ORDER BY";
pub const SPLIT_AT: &'static str = "SPLIT";
pub const GROUP_BY: &'static str = "GROUP BY";
pub const LIMIT_BY: &'static str = "LIMIT";
pub const START_AT: &'static str = "START";
pub const TIMEOUT: &'static str = "TIMEOUT";
pub const FETCH: &'static str = "FETCH";
pub const AND: &'static str = "AND";
pub const OR: &'static str = "OR";
pub const MILLISECOND: &'static str = "ms";
pub const SECOND: &'static str = "s";
pub const MINUTE: &'static str = "min";
pub const HOUR: &'static str = "h";
pub const INSERT: &'static str = "INSERT";
pub const INTO: &'static str = "INTO";
pub const VALUES: &'static str = "VALUES";
pub const DELETE: &'static str = "DELETE";


///核心设计!
#[derive(Debug, Clone)]
pub struct SQLRegion {
    region_field: RegionField,
    region_statement: String,
    keyword: String,
}

impl RegionImpl for SQLRegion {
    fn combine(&mut self, stmt: &str) -> &str {
        self.region_statement = String::from(stmt);
        self.region_statement.as_str()
    }
}


impl SQLRegion {
    pub fn new(region_field: RegionField, keyword: &str) -> Self {
        SQLRegion {
            region_field,
            region_statement: String::new(),
            keyword: String::from(keyword),
        }
    }
    pub fn new_no_arg(keyword: &str) -> Self {
        SQLRegion {
            region_field: RegionField::Single(String::new()),
            region_statement: String::new(),
            keyword: String::from(keyword),
        }
    }
    pub fn get_region_field(&self) -> &RegionField {
        &self.region_field
    }
    pub fn get_region_field_mut(&mut self) -> &mut RegionField {
        &mut self.region_field
    }
    pub fn get_keyword(&self) -> &str {
        &self.keyword
    }
    pub fn get_keyword_mut(&mut self) -> &mut str {
        &mut self.keyword
    }
    pub fn get_region_statement(&self) -> &str {
        &self.region_statement
    }
    pub fn get_region_statement_mut(&mut self) -> &mut str {
        &mut self.region_statement
    }
    pub fn set_keyword(&mut self, keyword: &str) {
        self.keyword = String::from(keyword);
    }
    pub fn set_region_statement(&mut self, region_statement: &str) {
        self.region_statement = String::from(region_statement);
    }
    pub fn set_region_field(&mut self, region_field: &RegionField) {
        self.region_field = region_field.clone();
    }
    ///如果是Multi就是push，Single就是Set
    pub fn push_set(&mut self, item: &SQLField) {
        match &mut self.region_field {
            RegionField::Multi(field_list) => {
                field_list.push(item.clone())
            }
            RegionField::Single(field) => {
                *field = String::from(item.get_field_value());
            }
        };
    }
    pub fn get_region_multi(&self) -> &Vec<SQLField> {
        match self.get_region_field() {
            RegionField::Multi(fields) => {
                return fields;
            }
            RegionField::Single(_) => {
                panic!("this fn is used for get region_field(RegionField::Multi)!")
            }
        }
    }
    pub fn set_region_multi(&mut self, value: Vec<SQLField>) {
        match self.get_region_field_mut() {
            RegionField::Multi(fields) => {
                *fields = value;
            }
            RegionField::Single(_) => {
                panic!("this fn is used for get region_field(RegionField::Multi)!")
            }
        }
    }
    pub fn region_multi_push(&mut self, value: SQLField) {
        match self.get_region_field_mut() {
            RegionField::Multi( fields) => {
                fields.push(value);
            }
            RegionField::Single(_) => {
                panic!("this fn is used for get region_field(RegionField::Multi)!")
            }
        }
    }
    pub fn get_region_multi_mut(&mut self) -> &mut Vec<SQLField> {
        match self.get_region_field_mut() {
            RegionField::Multi(ref mut fields) => {
                return fields;
            }
            RegionField::Single(_) => {
                panic!("this fn is used for get region_field(RegionField::Multi)!")
            }
        }
    }
    pub fn get_region_single(&self) -> &str {
        match self.get_region_field() {
            RegionField::Multi(_) => {
                panic!("this fn is used for get region_field(RegionField::Single)!")
            }
            RegionField::Single(field) => {
                field
            }
        }
    }
    pub fn set_region_single(&mut self, value: &str) {
        match self.get_region_field_mut() {
            RegionField::Multi(_) => {
                panic!("this fn is used for get region_field(RegionField::Single)!");
            }
            RegionField::Single(field) => {
                *field = String::from(value);
            }
        }
    }
}


#[derive(Debug, Clone)]
pub enum RegionField {
    Multi(Vec<SQLField>),
    Single(String),
}


#[derive(Debug, Clone)]
pub struct SQLField {
    keyword: String,
    field_value: String,
}

impl SQLField {
    pub fn new(keyword: &str, field_value: &str) -> SQLField {
        SQLField {
            keyword: String::from(keyword),
            field_value: String::from(field_value),
        }
    }
    pub fn get_field_value(&self) -> &str {
        &self.field_value
    }
    pub fn get_keyword(&self) -> &str {
        &self.keyword
    }
    pub fn set_keyword(&mut self, keyword: &str) {
        self.keyword = String::from(keyword);
    }
    pub fn set_field_value(&mut self, value: &str) {
        self.field_value = String::from(value);
    }
    ///匹配传入的keyword，相同则返回field_value
    pub fn get(&self, keyword: &str) -> Option<&str> {
        if !&self.get_keyword().eq(keyword) {
            return None;
        }
        Some(&self.get_field_value())
    }
}



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
#[derive(Debug, Clone)]
pub enum Statements {
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
    fn get_keyword(&self) -> &Statements;
    fn get_available(&self) -> &SQLRegion;
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


pub enum TimeUnit {
    MILLISECOND,
    SECOND,
    MINUTE,
    HOUR,
}

pub trait RegionImpl {
    ///组合region_fields得到region_statement
    fn combine(&mut self, stmt: &str) -> &str;
}


///Where条件构建
#[derive(Debug, Clone)]
pub struct Criteria {
    ///比较符
    judge: JudgeCriteria,
    ///自己构建，对于如：WHERE count(->experience->organisation) > 3
    ///则需要自己构建
    define: String,
    core: String,
    comparator: String,
    complex: RefCell<Vec<String>>,
}


impl Criteria {
    pub fn new() -> Self {
        Criteria {
            judge: JudgeCriteria::NONE,
            define: String::new(),
            core: String::new(),
            comparator: String::new(),
            complex: RefCell::new(Vec::new()),
        }
    }
    pub fn combine(&self) -> String {
        let mut res = String::new();
        if self.define.is_empty() {
            let mut sign: &str = "";
            match self.judge {
                JudgeCriteria::Eq => {
                    sign = EQ;
                }
                JudgeCriteria::Neq => {
                    sign = NEQ;
                }
                JudgeCriteria::Lt => {
                    sign = LT;
                }
                JudgeCriteria::Gt => {
                    sign = GT;
                }
                JudgeCriteria::Lte => {
                    sign = LTE;
                }
                JudgeCriteria::Gte => {
                    sign = GTE;
                }
                JudgeCriteria::NONE => ()
            }
            res = format!("{}{}{}{}{}", self.core, COMMON_SEPARATOR, sign, COMMON_SEPARATOR, self.comparator);
        } else {
            res = String::from(&self.define);
        }
        res
    }
    /// 自定义写入条件
    /// 由于有些条件通过Criteria当前提供的方法无法直接构建
    /// 例如：count(->experience->organisation) > 3
    /// 此时就需要调用者直接进行手写
    pub fn define(&mut self, define_str: &str) {
        self.define = String::from(define_str);
    }
    fn build_core(&mut self, core: &str) {
        let res = self.complexBuild();
        if res.is_empty() {
            self.core = String::from(core);
        } else {
            self.core = res;
        }
    }
    /// 相等条件
    /// core = comparator
    pub fn eq(&mut self, core: &str, comparator: &str) -> &mut Self {
        self.build_core(core);
        self.judge = JudgeCriteria::Eq;
        self.comparator = String::from(comparator);
        self
    }
    /// 大于条件
    /// core > comparator
    pub fn gt(&mut self, core: &str, comparator: &str) -> &mut Self {
        self.build_core(core);
        self.judge = JudgeCriteria::Gt;
        self.comparator = String::from(comparator);
        self
    }
    pub fn lt(&mut self, core: &str, comparator: &str) -> &mut Self {
        self.build_core(core);
        self.judge = JudgeCriteria::Lt;
        self.comparator = String::from(comparator);
        self
    }
    pub fn neq(&mut self, core: &str, comparator: &str) -> &mut Self {
        self.build_core(core);
        self.judge = JudgeCriteria::Neq;
        self.comparator = String::from(comparator);
        self
    }
    pub fn lte(&mut self, core: &str, comparator: &str) -> &mut Self {
        self.build_core(core);
        self.judge = JudgeCriteria::Lte;
        self.comparator = String::from(comparator);
        self
    }
    pub fn gte(&mut self, core: &str, comparator: &str) -> &mut Self {
        self.build_core(core);
        self.judge = JudgeCriteria::Gte;
        self.comparator = String::from(comparator);
        self
    }
    pub fn and(&self, left: &str, right: &str) -> String {
        let res = format!("{}{}{}{}{}", left, COMMON_SEPARATOR, AND, COMMON_SEPARATOR, right);
        self.complex.borrow_mut().push(res.clone());
        res
    }
    pub fn or(&self, left: &str, right: &str) -> String {
        let res = format!("{}{}{}{}{}", left, COMMON_SEPARATOR, OR, COMMON_SEPARATOR, right);
        self.complex.borrow_mut().push(res.clone());
        res
    }
    pub fn complexBuild(&mut self) -> String {
        if !self.complex.borrow().is_empty() {
            let mut counter: usize = 0;
            for core_complex in &*self.complex.borrow() {
                counter += 1;

                if counter.eq(&self.complex.borrow().len()) {
                    self.core = replace_str(&self.core, core_complex);
                } else {
                    let res = replace_str(&self.core, core_complex);
                    self.core = format!("( {} )", res);
                }
            }
        }
        self.core.clone()
    }
}

///=================================================<br>
/// @params:
/// <ol>
///     <li>core:源目标</li>
///     <li>replace:替换目标</li>
/// </ol>
/// @return:<br>
/// @date:2023/5/28<br>
/// @description:将Criteria的complexBuild方法中替换`()`<br>
///=================================================
fn replace_str(core: &str, replace: &str) -> String {
    let value = core.replace("( ", "").replace(" )", "");
    let res = replace.replace(&value, core);
    res.clone()
}

///=================================================<br>
/// @params:
/// <ol>
///     <li>Eq:等于</li>
///     <li>Lt:小于</li>
///     <li>Gt:大于</li>
///     <li>Neq:不等于</li>
///     <li>Lte:小于等于</li>
///     <li>Gte:大于等于</li>
/// </ol>
/// @date:2023/5/28<br>
/// @description:JudgeCriteria判断符枚举
///=================================================
#[derive(Debug, Clone)]
pub enum JudgeCriteria {
    Eq,
    Lt,
    Gt,
    Neq,
    Lte,
    Gte,
    NONE,
}
