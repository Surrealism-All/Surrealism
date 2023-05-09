use std::collections::HashMap;
use std::ops::Add;
use surrealdb::engine::remote::ws::Client;
use surrealdb::Surreal;
use std::sync::{Arc, Mutex};

const COMMON_SEPERATOR: &'static str = " ";
const END_SEPERATOR: &'static str = ";";
const NS: &'static str = "NS";
const DB: &'static str = "DB";


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

///USE语句包装器
///
#[derive(Debug)]
pub struct UseWrapper {
    pub keyword: String,
    pub available: Arc<Mutex<HashMap<&'static str, String>>>,
    pub stmt: String,
}

pub trait Wrapper {
    fn new() -> Self;
    fn and(&mut self) -> &mut Self;
    fn build(&mut self) -> &mut Self;
    fn commit(&self) -> &str;
    fn get_keyword(&self) -> &str;
    fn get_available(&self) -> Arc<Mutex<HashMap<&'static str, String>>>;
}

impl UseWrapper {
    pub fn use_ns(&mut self, namespace: &str) -> &mut Self {
        let stmt = format!("{}{}{}{}", self.stmt, NS, COMMON_SEPERATOR, namespace);
        self.stmt = stmt;
        self.available.lock().unwrap().insert(NS, namespace.to_string());
        self
    }
    pub fn use_db(&mut self, database: &str) -> &mut Self {
        let stmt = format!("{}{}{}{}", self.stmt, DB, COMMON_SEPERATOR, database);
        self.stmt = stmt;
        self.available.lock().unwrap().insert(DB,database.to_string());
        self
    }
}

impl Wrapper for UseWrapper {
    fn new() -> Self {
        UseWrapper {
            keyword: "USE".to_string(),
            available: Arc::new(Mutex::new(HashMap::new())),
            stmt: String::from("USE").add(COMMON_SEPERATOR),
        }
    }

    fn and(&mut self) -> &mut Self {
        self.stmt = format!("{}{}", self.stmt, COMMON_SEPERATOR);

        self
    }
    fn build(&mut self) -> &mut Self {
        self.stmt = format!("{}{}", self.stmt, END_SEPERATOR);
        self
    }
    fn commit(&self) -> &str {
        &self.stmt
    }
    fn get_keyword(&self) -> &str {
        &self.keyword
    }
    fn get_available(&self) -> Arc<Mutex<HashMap<&'static str,String>>> {
        self.available.clone()
    }
}

