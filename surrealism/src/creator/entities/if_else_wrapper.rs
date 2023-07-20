//!IF @condition THEN
//! 	@expression
//! [ ELSE IF @condition THEN
//! 	@expression ... ]
//! [ ELSE
//! 	@expression ]
//! END

use crate::creator::entities::Statements;
use super::{Wrapper, RegionField, SQLField, SQLRegion, IF, ELSE, ELSE_IF, THEN, END, COMMON_SEPARATOR, Criteria};
use crate::handle_str;

#[derive(Clone, Debug)]
pub struct IfElseWrapper {
    ///关键字
    keyword: Statements,
    available: SQLRegion,
    if_condition: Condition,
    else_if_condition: Vec<Condition>,
    else_condition: Condition,
    ///define
    define: bool,
}

#[derive(Clone, Debug)]
struct Condition {
    keyword: String,
    condition: String,
    then: String,
}

impl Condition {
    pub fn new(keyword: &str) -> Self {
        Condition {
            keyword: String::from(keyword),
            condition: String::new(),
            then: String::new(),
        }
    }
    pub fn get_condition(&self) -> &str {
        &self.condition
    }
    pub fn get_then(&self) -> &str {
        &self.then
    }
    pub fn get_keyword(&self) -> &str {
        &self.keyword
    }
    pub fn set_condition(&mut self, condition: &str) {
        self.condition = String::from(condition);
    }
    pub fn set_then(&mut self, then: &str) {
        self.then = String::from(then);
    }
    pub fn combine(&self) -> String {
        if self.get_then().is_empty() {
            return "".to_string();
        }
        if self.get_condition().is_empty() {
            return format!("{}{}{}{}", self.get_keyword(), COMMON_SEPARATOR, self.get_then(), COMMON_SEPARATOR);
        }
        format!("{}{}{}{}{} {} {}", self.get_keyword(), COMMON_SEPARATOR, self.get_condition(), COMMON_SEPARATOR, THEN, self.get_then(), COMMON_SEPARATOR)
    }
}


impl IfElseWrapper {
    pub fn if_else(&mut self, stmt: &str) {
        self.available.set_region_statement(stmt);
    }
    pub fn if_condition(&mut self, criteria: &Criteria, then: &mut impl Wrapper) -> &mut Self {
        self.if_condition.set_condition(&criteria.combine());
        self.if_condition.set_then(then.commit());
        self
    }
    pub fn else_if_condition(&mut self, criteria: &Criteria, then: &mut impl Wrapper) -> &mut Self {
        let mut tmp = Condition::new(ELSE_IF);
        tmp.set_condition(&criteria.combine());
        tmp.set_then(then.commit());
        self.else_if_condition.push(tmp);
        self
    }
    pub fn else_condition(&mut self, then: &mut impl Wrapper) -> &mut Self {
        self.else_condition.set_then(then.commit());
        self
    }
    pub fn if_condition_str(&mut self, criteria: &Criteria, then: &str) -> &mut Self {
        self.if_condition.set_condition(&criteria.combine());
        self.if_condition.set_then(handle_str(serde_json::to_string(then).unwrap().as_str()).as_str());
        self
    }
    pub fn else_if_condition_str(&mut self, criteria: &Criteria, then: &str) -> &mut Self {
        let mut tmp = Condition::new(ELSE_IF);
        tmp.set_condition(&criteria.combine());
        tmp.set_then(handle_str(serde_json::to_string(then).unwrap().as_str()).as_str());
        self.else_if_condition.push(tmp);
        self
    }

    pub fn else_condition_str(&mut self, then: &str) -> &mut Self {
        self.else_condition.set_then(handle_str(serde_json::to_string(then).unwrap().as_str()).as_str());
        self
    }
    fn available_push(&mut self, keyword: &str, stmt: &str, msg: &str) {
        if !stmt.is_empty() {
            let push_stmt = SQLField::new(keyword, stmt);
            self.available.push_set(&push_stmt);
        } else {
            if !msg.is_empty() {
                panic!("{}", msg);
            }
        }
    }

    fn build_available(&mut self) {
        self.available_push(
            String::from(self.if_condition.get_keyword()).as_str(),
            &self.if_condition.combine(),
            "IF ELSE Statement need if condition!",
        );
        for item in self.else_if_condition.clone() {
            self.available_push(
                String::from(item.get_keyword()).as_str(),
                &item.combine(),
                "",
            );
        }
        self.available_push(
            String::from(self.else_condition.get_keyword()).as_str(),
            &self.else_condition.combine(),
            "",
        )
    }
}

impl Wrapper for IfElseWrapper {
    fn new() -> Self {
        IfElseWrapper {
            keyword: Statements::IF_ELSE,
            available: SQLRegion::new(RegionField::Multi(Vec::new()), IF),
            if_condition: Condition::new(IF),
            else_if_condition: vec![],
            else_condition: Condition::new(ELSE),
            define: false,
        }
    }

    fn commit(&mut self) -> &str {
        if !self.define {
            self.build_available();
            let mut res = String::new();
            let available_list = self.available.get_region_multi();
            let mut counter = 0;
            for item in available_list {
                counter += 1;
                res.push_str(item.get_field_value());
                if counter != available_list.len() {
                    res.push_str(COMMON_SEPARATOR);
                }
            }
            res.push_str(END);
            self.available.set_region_statement(&res);
        }
        self.available.get_region_statement()
    }

    fn get_keyword(&self) -> &Statements {
        &self.keyword
    }

    fn get_available(&self) -> &SQLRegion {
        &self.available
    }
}