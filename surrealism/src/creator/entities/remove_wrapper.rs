//! REMOVE STMT
//! ```txt
//! REMOVE [
//! 	NAMESPACE @name
//! 	| DATABASE @name
//! 	| LOGIN @name ON [ NAMESPACE | DATABASE ]
//! 	| TOKEN @name ON [ NAMESPACE | DATABASE ]
//! 	| SCOPE @name
//! 	| TABLE @name
//! 	| EVENT @name ON [ TABLE ] @table
//! 	| FUNCTION fn::@name
//! 	| FIELD @name ON [ TABLE ] @table
//! 	| INDEX @name ON [ TABLE ] @table
//! 	| PARAM $@name
//! ]
//! ```
//! ```txt
//! @author:syf20020816@outlook.com
//! @version:0.0.1
//! @date:20230610
//! ```
use super::{Statements, Wrapper, SQLRegion, RegionField, END_SEPARATOR, REMOVE, NAMESPACE, DATABASE, LOGIN, FIELD, ON, TOKEN, SCOPE, TABLE, EVENT, FUNCTION, INDEX, PARAM};

#[derive(Debug, Clone)]
pub struct RemoveWrapper {
    keyword: Statements,
    available: SQLRegion,
}

impl RemoveWrapper {
    pub fn namespace(&mut self, name: &str) {
        self.available.set_region_single(format!("{} {}", NAMESPACE, name).as_str());
    }
    pub fn database(&mut self, name: &str) {
        self.available.set_region_single(format!("{} {}", DATABASE, name).as_str());
    }
    pub fn login_ns(&mut self, name: &str) {
        self.available.set_region_single(format!("{} {} {} {}", LOGIN, name, ON, NAMESPACE).as_str());
    }
    pub fn login_db(&mut self, name: &str) {
        self.available.set_region_single(format!("{} {} {} {}", LOGIN, name, ON, DATABASE).as_str());
    }
    pub fn token_ns(&mut self, name: &str) {
        self.available.set_region_single(format!("{} {} {} {}", TOKEN, name, ON, NAMESPACE).as_str());
    }
    pub fn token_db(&mut self, name: &str) {
        self.available.set_region_single(format!("{} {} {} {}", TOKEN, name, ON, DATABASE).as_str());
    }
    pub fn scope(&mut self, name: &str) {
        self.available.set_region_single(format!("{} {}", SCOPE, name).as_str());
    }
    pub fn table(&mut self, name: &str) {
        self.available.set_region_single(format!("{} {}", TABLE, name).as_str());
    }
    pub fn event(&mut self, name: &str, table: &str) {
        self.available.set_region_single(format!("{} {} {} {} {}", EVENT, name, ON, TABLE, table).as_str());
    }
    pub fn function(&mut self, name: &str) {
        self.available.set_region_single(format!("{} fn::{}", FUNCTION, name).as_str());
    }
    pub fn field(&mut self, name: &str, table: &str) {
        self.available.set_region_single(format!("{} {} {} {} {}", FIELD, name, ON, TABLE, table).as_str());
    }
    pub fn index(&mut self, name: &str, table: &str) {
        self.available.set_region_single(format!("{} {} {} {} {}", INDEX, name, ON, TABLE, table).as_str());
    }
    pub fn param(&mut self, name: &str) {
        self.available.set_region_single(format!("{} {}", PARAM, name).as_str());
    }
    pub fn define(&mut self, stmt: &str) {
        self.available.set_region_single(stmt);
    }
}

impl Wrapper for RemoveWrapper {
    fn new() -> Self {
        RemoveWrapper {
            keyword: Statements::REMOVE,
            available: SQLRegion::new(RegionField::Single(String::new()), REMOVE),
        }
    }

    fn commit(&mut self) -> &str {
        let stmt = format!("{} {}{}", self.available.get_keyword(), self.available.get_region_single(), END_SEPARATOR);
        self.available.set_region_statement(&stmt);
        self.available.get_region_statement()
    }

    fn get_keyword(&self) -> &Statements {
        &self.keyword
    }

    fn get_available(&self) -> &SQLRegion {
        &self.available
    }
}