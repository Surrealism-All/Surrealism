use std::collections::HashMap;
use std::ops::Add;
use std::sync::{Arc, Mutex};
use super::{COMMON_SEPARATOR, END_SEPARATOR, EQUAL_SEPARATOR, NEXT_SEPARATOR, AS, IS_SEPARATOR, SELECT, FROM, AvailData, Wrapper};
use log::error;
use crate::{ParseSQL, SQLParser, handle_str, check_available_order};
use serde::{Deserialize, Serialize};


///Select语句包装器
/// 生成select语句，实现查询数据操作
/// keywords:关键词
/// available:参数存储器
/// stmt:具体语句
/// field_type:设置构建类型(SET,CONTENT)
/// return_type:返回类型
/// example:
#[derive(Debug, Clone)]
pub struct SelectWrapper {
    pub keyword: String,
    pub available: Vec<AvailData>,
    pub stmt: String,
    pub fieldList: Vec<Field>,
    pub criteriaList: Vec<Criteria>,

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
pub struct Criteria {
    ///比较符
    judge: JudgeCriteria,
    ///自己构建，对于如：WHERE count(->experience->organisation) > 3
    ///则需要自己构建
    define: String,
    core: String,
    comparator: String,
}
#[derive(Debug, Clone)]
pub enum JudgeCriteria {
    Eq,
    Lt,
    Gt,
    Neq,
    Lte,
    Gte,
}

impl Criteria {
    pub fn new(judge: JudgeCriteria, core: &str, comparator: &str) -> Self {
        Criteria {
            judge,
            define: String::new(),
            core: String::from(core),
            comparator: String::from(comparator),
        }
    }
    pub fn define(&mut self, df: &str) -> &mut Self {
        self.define = String::from(df);
        self
    }
}

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
            fieldList: Vec::new(),
            criteriaList: Vec::new()
        }
    }

    // fn and(&mut self) -> &mut Self {
    //     let len = self.get_available().len();
    //     let tmp = AvailData::new(len, "COMMON_SEPARATOR".to_string(), String::from(COMMON_SEPARATOR), true, false);
    //     self.available.push(tmp);
    //     self
    // }
    //
    // fn build(&mut self) -> &mut Self {
    //     let len = self.get_available().len();
    //     let tmp = AvailData::new(len, "END_SEPARATOR".to_string(), END_SEPARATOR.to_string(), false, true);
    //     self.available.push(tmp);
    //     self
    // }

    fn commit(&mut self) -> &str {
        let tmp = self.get_available().clone();
        if check_available_order(&tmp) {
            let len = tmp.len();
            for t in tmp {
                self.stmt.push_str(t.value());
            }
        }
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
        // for field in fields {
        //     match self.fields {
        //         true => {
        //             let len = self.get_available().len();
        //             let tmp = AvailData::new(len, "NEXT_SEPARATOR".to_string(), String::from(NEXT_SEPARATOR), true, false);
        //             self.available.push(tmp);
        //         }
        //         false => {
        //             self.fields = true;
        //         }
        //     }
        //     self.select_field(field);
        // }

        self
    }
    ///查询字段
    pub fn select_field(&mut self, field: &Field) -> &mut Self {
        let len = self.get_available().len();
        if field.get_as_name() != "" {
            let tmp1 = AvailData::new(len, String::from("field"), String::from(field.get_name()), false, false);
            let tmp2 = AvailData::new(len + 1, "COMMON_SEPARATOR".to_string(), String::from(COMMON_SEPARATOR), true, false);
            let tmp3 = AvailData::new(len + 2, String::from(AS), String::from(AS), true, false);
            let tmp4 = AvailData::new(len + 3, "COMMON_SEPARATOR".to_string(), String::from(COMMON_SEPARATOR), true, false);
            let tmp5 = AvailData::new(len + 4, String::from("as_field"), String::from(field.get_as_name()), false, false);
            self.available.push(tmp1);
            self.available.push(tmp2);
            self.available.push(tmp3);
            self.available.push(tmp4);
            self.available.push(tmp5);
        } else {
            let tmp = AvailData::new(len, String::from("field"), String::from(field.get_name()), false, false);
            self.available.push(tmp);
        }

        self
    }
    pub fn from(&mut self, table_name: &str) -> &mut Self {
        let len = self.get_available().len();
        let tmp1 = AvailData::new(len, String::from(FROM), String::from(FROM), false, false);
        let tmp2 = AvailData::new(len + 1, "COMMON_SEPARATOR".to_string(), String::from(COMMON_SEPARATOR), true, false);
        let tmp3 = AvailData::new(len + 2, String::from("table"), String::from(table_name), false, false);
        self.available.push(tmp1);
        self.available.push(tmp2);
        self.available.push(tmp3);
        self
    }
}


