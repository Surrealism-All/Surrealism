use std::collections::HashMap;
use std::ops::Add;
use std::sync::{Arc, Mutex};
use super::{Wrapper,DB,NS,COMMON_SEPARATOR,END_SEPARATOR};

///USE语句包装器
/// keywords:关键词
/// available:参数存储器
/// stmt:具体语句
#[derive(Debug,Clone)]
pub struct UseWrapper {
    pub keyword: String,
    pub available: Arc<Mutex<HashMap<&'static str, String>>>,
    pub stmt: String,
}

impl UseWrapper {
    pub fn use_ns(&mut self, namespace: &str) -> &mut Self {
        let stmt = format!("{}{}{}{}", self.stmt, NS, COMMON_SEPARATOR, namespace);
        self.stmt = stmt;
        self.available.lock().unwrap().insert(NS, namespace.to_string());
        self
    }
    pub fn use_db(&mut self, database: &str) -> &mut Self {
        let stmt = format!("{}{}{}{}", self.stmt, DB, COMMON_SEPARATOR, database);
        self.stmt = stmt;
        self.available.lock().unwrap().insert(DB, database.to_string());
        self
    }
}

impl Wrapper for UseWrapper {
    fn new() -> Self {
        UseWrapper {
            keyword: "USE".to_string(),
            available: Arc::new(Mutex::new(HashMap::new())),
            stmt: String::from("USE").add(COMMON_SEPARATOR),
        }
    }

    fn and(&mut self) -> &mut Self {
        self.stmt = format!("{}{}", self.stmt, COMMON_SEPARATOR);
        self
    }
    fn build(&mut self) -> &mut Self {
        self.stmt = format!("{}{}", self.stmt, END_SEPARATOR);
        self
    }
    fn commit(&mut self) -> &str {
        let stmt =  &self.stmt;
        // self = &mut Self::new();
        stmt
    }
    fn get_keyword(&self) -> &str {
        &self.keyword
    }
    fn get_available(&self) -> Arc<Mutex<HashMap<&'static str, String>>> {
        self.available.clone()
    }


}
