use std::cell::RefCell;
use std::collections::HashMap;
use std::ops::Add;
use std::sync::{Arc, Mutex};
use super::{RegionImpl, AND, OR, COMMON_SEPARATOR, END_SEPARATOR, EQUAL_SEPARATOR, NEXT_SEPARATOR, AS, IS_SEPARATOR, SELECT, FROM, WHERE, AvailData, Wrapper, EQ, NEQ, LT, GT, LTE, GTE, ORDER_BY, GROUP_BY, SPLIT_AT, START_AT, LIMIT_BY};
use log::error;
use crate::{ParseSQL, SQLParser, handle_str, check_available_order};
use serde::{Deserialize, Serialize};


///Select语句包装器
/// 生成select语句，实现查询数据操作
/// example:
#[derive(Debug, Clone)]
pub struct SelectWrapper {
    ///关键字
    pub keyword: String,
    pub available: Vec<AvailData>,
    pub stmt: String,
    ///表区域（生成FROM @TableName）
    pub table_region: TableRegion,
    ///字段区域（生成SELECT @Fields...）
    pub field_region: FieldRegion,
    ///条件区域，where子句（生成WHERE @Condition）
    pub where_region: WhereRegion,
    ///处理区域（生成ORDER BY，GROUP BY等子句）
    pub handle_region: Vec<HandleRegion>,

}

///字段结构体
///name:名称
///as_name:别买那个
#[derive(Debug, Clone)]
pub struct Field {
    pub name: String,
    pub as_name: String,
}

impl Field {
    pub fn new(name: &str) -> Self {
        Field {
            name: String::from(name),
            as_name: String::new(),
        }
    }
    pub fn as_name(&mut self, name: &str) -> &mut Self {
        self.as_name = String::from(name);
        self
    }
    pub fn get_as_name(&self) -> &str {
        &self.as_name
    }
    pub fn get_name(&self) -> &str {
        &self.name
    }
}

#[derive(Debug, Clone)]
pub struct FieldRegion {
    field_str: String,
    available_data: Vec<AvailData>,
    keyword: String,
}

impl RegionImpl for FieldRegion {
    fn combine(&mut self) -> &str {
        let mut stmt = String::new();
        let mut counter = 0;
        for data in &self.available_data {
            counter += 1;
            if counter == self.available_data.len() {
                stmt.push_str(data.value());
            } else {
                stmt.push_str(data.value());
                stmt.push_str(NEXT_SEPARATOR);
            }
        }
        self.field_str = format!("{}{}{}", self.keyword, COMMON_SEPARATOR, stmt);
        self.field_str.as_str()
    }
}

#[derive(Debug, Clone)]
pub struct TableRegion {
    table: String,
    table_str: String,
    keyword: String,
}

impl RegionImpl for TableRegion {
    fn combine(&mut self) -> &str {
        self.table_str = format!("{}{}{}", self.keyword, COMMON_SEPARATOR, self.table);
        self.table_str.as_str()
    }
}

#[derive(Debug, Clone)]
pub struct WhereRegion {
    available_data: Vec<AvailData>,
    where_str: String,
    keyword: String,
}

impl RegionImpl for WhereRegion {
    fn combine(&mut self) -> &str {
        self.where_str = format!("{}{}{}", self.keyword, COMMON_SEPARATOR, self.available_data[0].value());
        self.where_str.as_str()
    }
}

#[derive(Debug, Clone)]
pub struct HandleRegion {
    available_data: Vec<AvailData>,
    handle_str: String,
    keyword: String,
}

impl HandleRegion {
    pub fn new() -> Self {
        HandleRegion {
            available_data: Vec::new(),
            handle_str: String::new(),
            keyword: String::new(),
        }
    }
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

// ///构建复杂的条件语句
// /// 例如需要AND,OR的条件语句
// pub struct CriteriaComplex {
//     core: String,
//     comparator: String,
//
// }

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
    fn buildCore(&mut self, core: &str) {
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
        self.buildCore(core);
        self.judge = JudgeCriteria::Eq;
        self.comparator = String::from(comparator);
        self
    }
    /// 大于条件
    /// core > comparator
    pub fn gt(&mut self, core: &str, comparator: &str) -> &mut Self {
        self.buildCore(core);
        self.judge = JudgeCriteria::Gt;
        self.comparator = String::from(comparator);
        self
    }
    pub fn lt(&mut self, core: &str, comparator: &str) -> &mut Self {
        self.buildCore(core);
        self.judge = JudgeCriteria::Lt;
        self.comparator = String::from(comparator);
        self
    }
    pub fn neq(&mut self, core: &str, comparator: &str) -> &mut Self {
        self.buildCore(core);
        self.judge = JudgeCriteria::Neq;
        self.comparator = String::from(comparator);
        self
    }
    pub fn lte(&mut self, core: &str, comparator: &str) -> &mut Self {
        self.buildCore(core);
        self.judge = JudgeCriteria::Lte;
        self.comparator = String::from(comparator);
        self
    }
    pub fn gte(&mut self, core: &str, comparator: &str) -> &mut Self {
        self.buildCore(core);
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

fn replace_str(core: &str, replace: &str) -> String {
    let value = core.replace("( ", "").replace(" )", "");
    let res = replace.replace(&value, core);
    res.clone()
}

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

// impl Criteria {
//     pub fn new(judge: JudgeCriteria, core: &str, comparator: &str) -> Self {
//         Criteria {
//             judge,
//             define: String::new(),
//             core: String::from(core),
//             comparator: String::from(comparator),
//         }
//     }
//     pub fn define(&mut self, df: &str) -> &mut Self {
//         self.define = String::from(df);
//         self
//     }
// }

impl Wrapper for SelectWrapper {
    fn new() -> Self {
        let mut available: Vec<AvailData> = Vec::new();
        let tmp1 = AvailData::new(0, String::from(SELECT), String::from(SELECT), false, false);
        let tmp2 = AvailData::new(1, "COMMON_SEPARATOR".to_string(), String::from(COMMON_SEPARATOR), true, false);
        available.push(tmp1);
        available.push(tmp2);
        SelectWrapper {
            keyword: String::from(SELECT),
            available,
            stmt: String::new(),
            table_region: TableRegion {
                table: String::new(),
                table_str: String::new(),
                keyword: String::from(FROM),
            },
            field_region: FieldRegion {
                field_str: String::new(),
                available_data: vec![],
                keyword: String::from(SELECT),
            },
            where_region: WhereRegion {
                available_data: vec![],
                where_str: String::new(),
                keyword: String::from(WHERE),
            },
            handle_region: vec![],
        }
    }


    fn commit(&mut self) -> &str {
        self.stmt = format!("{}{}{}{}{}{}", self.field_region.combine(), COMMON_SEPARATOR, self.table_region.combine(), COMMON_SEPARATOR, self.where_region.combine(), END_SEPARATOR);
        &self.stmt
    }

    fn get_keyword(&self) -> &str {
        &self.keyword
    }

    fn get_available(&self) -> &Vec<AvailData> {
        &self.available
    }
}

impl SelectWrapper {
    ///通用查询
    pub fn select(&mut self, query: &str) -> &mut Self {
        let len = self.get_available().len();
        let tmp = AvailData::new(len, String::from("query"), String::from(query), false, false);
        self.available.push(tmp);
        self
    }
    ///查询字段
    pub fn select_fields(&mut self, fields: &Vec<Field>) -> &mut Self {
        for field in fields {
            self.select_field(field);
        }
        self
    }
    ///查询字段
    pub fn select_field(&mut self, field: &Field) -> &mut Self {
        let len = self.get_available().len();
        let mut field_stmt = String::new();
        if field.get_as_name().is_empty() {
            field_stmt = format!("{}", field.get_name());
        } else {
            field_stmt = format!("{}{}{}{}{}", field.get_name(), COMMON_SEPARATOR, AS, COMMON_SEPARATOR, field.get_as_name());
        }
        let value = AvailData::new(len, String::from("field"), field_stmt, false, false);
        self.field_region.available_data.push(value);
        self
    }
    ///from子句
    pub fn from(&mut self, table_name: &str) -> &mut Self {
        self.table_region.table = String::from(table_name);
        self
    }
    ///where子句
    pub fn where_condition(&mut self, condition: &Criteria) -> &mut Self {
        let len = self.get_available().len();
        let value = AvailData::new(len, String::from("where_criteria"), condition.combine(), false, false);
        self.where_region.available_data.push(value);
        self
    }
    pub fn order_by(&mut self, conditions: &Vec<&str>) -> &mut Self {
        // let mut tmp = HandleRegion::new();
        // for condition in *conditions {
        //     //TODO:
        //     tmp.keyword = String::from(ORDER_BY);
        // }
        self
    }
    pub fn group_by(&mut self, condition: &str) -> &mut Self {
        self
    }
    pub fn limit_by(&mut self, condition: &str) -> &mut Self {
        self
    }
    pub fn start_at(&mut self, condition: &str) -> &mut Self {
        self
    }
}


