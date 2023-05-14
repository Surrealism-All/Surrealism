use std::collections::HashMap;
use std::ops::Add;
use std::sync::{Arc, Mutex};
use super::{Wrapper, DB, NS, COMMON_SEPARATOR, END_SEPARATOR, USE, AvailData};
use crate::check_available_order;

///USE语句包装器
/// keywords:关键词
/// available:参数存储器
/// stmt:具体语句
#[derive(Debug, Clone)]
pub struct UseWrapper {
    pub keyword: String,
    pub available: Vec<AvailData>,
    pub stmt: String,
}

impl UseWrapper {
    pub fn use_ns(&mut self, namespace: &str) -> &mut Self {
        let len = self.get_available().len();
        let tmp1 = AvailData::new(len, String::from(NS), String::from(NS), false, false);
        let tmp2 = AvailData::new(len+1, "COMMON_SEPARATOR".to_string(), String::from(COMMON_SEPARATOR), true, false);
        let tmp3 = AvailData::new(len+2, String::from("namespace"), String::from(namespace), false, false);
        self.available.push(tmp1);
        self.available.push(tmp2);
        self.available.push(tmp3);
        self
    }
    pub fn use_db(&mut self, database: &str) -> &mut Self {
        let len = self.get_available().len();
        let tmp1 = AvailData::new(len, String::from(DB), String::from(NS), false, false);
        let tmp2 = AvailData::new(len+1, "COMMON_SEPARATOR".to_string(), String::from(COMMON_SEPARATOR), true, false);
        let tmp3 = AvailData::new(len+2, String::from("database"), String::from(database), false, false);
        self.available.push(tmp1);
        self.available.push(tmp2);
        self.available.push(tmp3);
        self
    }
}

impl Wrapper for UseWrapper {
    fn new() -> Self {
        let mut available = Vec::new();
        let tmp1 = AvailData::new(0, String::from(USE), String::from(USE), false, false);
        let tmp2 = AvailData::new(1, "COMMON_SEPARATOR".to_string(), String::from(COMMON_SEPARATOR), true, false);
        available.push(tmp1);
        available.push(tmp2);
        UseWrapper {
            keyword: String::from(USE),
            available,
            stmt: String::new(),
        }
    }

    fn and(&mut self) -> &mut Self {
        let len = self.get_available().len();
        let tmp = AvailData::new(len, "COMMON_SEPARATOR".to_string(), COMMON_SEPARATOR.to_string(), true, false);
        self.available.push(tmp);
        self
    }
    fn build(&mut self) -> &mut Self {
        let len = self.get_available().len();
        let tmp = AvailData::new(len, "END_SEPARATOR".to_string(), END_SEPARATOR.to_string(), false, true);
        self.available.push(tmp);
        self
    }
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
