//!```txt
//! RELATE @from -> @table -> @with
//! 	[ CONTENT @value
//! 	  | SET @field = @value ...
//! 	]
//! 	[ RETURN [ NONE | BEFORE | AFTER | DIFF | @projections ... ]
//! 	[ TIMEOUT @duration ]
//! 	[ PARALLEL ]
//! ;
//! ```
//!
//! ```txt
//! @author:syf20020816@outlook.com
//! @version:0.0.1
//! @date:20230610
//! ```

use serde::Serialize;
use crate::{handle_str, TimeUnit};
use super::{Statements, SQLRegion, SQLField, RegionField, Wrapper, RELATE, PARALLEL, NEXT_SEPARATOR,  TIMEOUT, RETURN, DAY, MINUTE, MILLISECOND, SECOND, HOUR, COMMON_SEPARATOR, END_SEPARATOR, NONE, DIFF, BEFORE, AFTER, SET, EQUAL_SEPARATOR, CONTENT, ContentType};


const RELATE_SEPARATOR: &str = "->";

#[derive(Debug, Clone)]
pub struct RelateWrapper {
    ///关键词
    keyword: Statements,
    ///可获取值
    available: SQLRegion,
    content_region: SQLRegion,
    from: String,
    to: String,
    with: String,
    content_type: ContentType,
    timeout_region: SQLField,
    return_region: SQLField,
    parallel: bool,
    define: bool,
}

impl RelateWrapper {
    fn get_from(&self) -> &str {
        &self.from
    }
    fn get_to(&self) -> &str {
        &self.to
    }
    fn get_with(&self) -> &str {
        &self.with
    }
    /// 设置起始的表
    /// Set starting table
    pub fn from(&mut self, from_table: &str) -> &mut Self {
        self.from = String::from(from_table);
        self
    }
    /// 设置结果存储表
    /// Set result storage table
    pub fn to(&mut self, to_table: &str) -> &mut Self {
        self.to = String::from(to_table);
        self
    }
    /// 设置关联表
    /// Set Association Table
    pub fn with(&mut self, with_table: &str) -> &mut Self {
        self.with = String::from(with_table);
        self
    }
    /// SET方式构建字段
    /// SET method for constructing fields
    pub fn set<T: Serialize>(&mut self, field_name: &'static str, value: T) -> &mut Self {
        match self.content_type {
            ContentType::CONTENT => panic!("you cannot use set and content together!"),
            ContentType::SET => (),
            ContentType::NONE => {
                self.content_type = ContentType::SET;
                self.content_region.set_keyword(SET);
            }
            _ => {
                panic!("ContentType::MERGE and ContentType::PATCH is not allowed to be used in Relate statement!");
            }
        };
        let field_value = format!("{}{}{}", field_name, EQUAL_SEPARATOR, handle_str(serde_json::to_string(&value).unwrap().as_str()));
        let field = SQLField::new(SET, &field_value);
        self.content_region.push_set(&field);
        self
    }
    /// CONTENT方式构建字段
    /// CONTENT method for constructing fields
    pub fn content<T: Serialize>(&mut self, content_obj: T) -> &mut Self {
        match self.content_type {
            ContentType::SET => panic!("you cannot use set and content together!"),
            ContentType::CONTENT => panic!("you cannot use content twice!"),
            ContentType::NONE => {
                self.content_type = ContentType::CONTENT;
                self.content_region.set_keyword(CONTENT);
                let content_value = handle_str(serde_json::to_string(&content_obj).unwrap().as_str());
                self.content_region.set_region_field(&RegionField::Single(content_value));
            }
            _ => {
                panic!("ContentType::MERGE and ContentType::PATCH is not allowed to be used in Relate statement!");
            }
        };
        self
    }
    ///返回NONE
    pub fn return_none(&mut self) -> &mut Self {
        self.return_build(NONE);
        self
    }
    ///返回DIFF
    pub fn return_diff(&mut self) -> &mut Self {
        self.return_build(DIFF);
        self
    }
    ///返回BEFORE
    pub fn return_before(&mut self) -> &mut Self {
        self.return_build(BEFORE);
        self
    }
    ///返回AFTER
    pub fn return_after(&mut self) -> &mut Self {
        self.return_build(AFTER);
        self
    }
    ///返回某几个字段
    pub fn return_field(&mut self, field_names: Vec<&str>) -> &mut Self {
        let mut fields = String::new();
        for i in 0..field_names.len() {
            fields.push_str(field_names[i]);
            if i != field_names.len() - 1 {
                fields.push_str(NEXT_SEPARATOR);
            }
        }
        self.return_build(&fields);
        self
    }
    fn return_build(&mut self, return_str: &str) {
        if self.return_region.get_field_value().is_empty() {
            self.return_region.set_field_value(return_str);
        } else {
            panic!("{}", "you cannot use return twice!");
        }
    }

    pub fn timeout(&mut self, time: usize, unit: TimeUnit) -> &mut Self {
        let mut res = "";
        match unit {
            TimeUnit::MILLISECOND => res = MILLISECOND,
            TimeUnit::SECOND => res = SECOND,
            TimeUnit::MINUTE => res = MINUTE,
            TimeUnit::HOUR => res = HOUR,
            TimeUnit::DAY => res = DAY
        };
        self.timeout_region.set_field_value(format!("{}{}", time, res).as_str());
        self
    }
    pub fn parallel(&mut self) -> &mut Self {
        self.parallel = true;
        self
    }
    pub fn define(&mut self, stmt: &str) {
        self.define = true;
        self.available.set_region_single(stmt);
    }
    fn build_from_to_with_stmt(&self) -> String {
        if self.get_from().is_empty() {
            panic!("you mut set the starting table!");
        }
        if self.get_to().is_empty() {
            panic!("you mut set the result storage table!");
        }
        if self.get_with().is_empty() {
            panic!("you mut set the association table!");
        }
        format!("{} {} {} {} {}", self.get_from(), RELATE_SEPARATOR, self.get_to(), RELATE_SEPARATOR, self.get_with())
    }
    fn build_content_stmt(&mut self) {
        let mut tmp_stmt = String::new();
        match self.content_type {
            ContentType::SET => {
                let tmp_content = self.content_region.get_region_multi();
                let field_len = self.content_region.get_region_multi().len();
                for i in 0..field_len {
                    tmp_stmt.push_str(tmp_content[i].get_field_value());
                    if i != field_len - 1 {
                        tmp_stmt.push_str(NEXT_SEPARATOR)
                    }
                }
            }
            ContentType::NONE => {
                panic!("RelateWrapper is used to building relate statement , if you just wanna check the database please use SelectWrapper!")
            }
            ContentType::CONTENT => tmp_stmt = String::from(self.content_region.get_region_single()),
            _ => {
                panic!("Cannot be used in RelateWrapper (ContentType::MERGE | ContentType::PATCH)!")
            }
        };
        self.content_region.set_region_statement(&tmp_stmt);
    }
    fn build_available(&mut self) {
        self.available_push(
            "",
            self.build_from_to_with_stmt().as_str(),
            "",
        );
        self.build_content_stmt();
        self.available_push(
            String::from(self.content_region.get_keyword()).as_str(),
            String::from(self.content_region.get_region_statement()).as_str(),
            "",
        );
        self.available_push(
            String::from(self.return_region.get_keyword()).as_str(),
            String::from(self.return_region.get_field_value()).as_str(),
            "",
        );
        self.available_push(
            String::from(self.timeout_region.get_keyword()).as_str(),
            String::from(self.timeout_region.get_field_value()).as_str(),
            "",
        );
        if self.parallel {
            self.available_push(
                "",
                PARALLEL,
                "",
            );
        }
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
}

impl Wrapper for RelateWrapper {
    fn new() -> Self {
        RelateWrapper {
            keyword: Statements::RELATE,
            available: SQLRegion::new(RegionField::Multi(Vec::new()), RELATE),
            content_region: SQLRegion::new(RegionField::Multi(Vec::new()), NONE),
            from: "".to_string(),
            to: "".to_string(),
            with: "".to_string(),
            content_type: ContentType::NONE,
            timeout_region: SQLField::new(TIMEOUT, ""),
            return_region: SQLField::new(RETURN, ""),
            parallel: false,
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
                if item.get_keyword().is_empty() {
                    res.push_str(item.get_field_value());
                } else {
                    res.push_str(format!("{}{}{}", item.get_keyword(), COMMON_SEPARATOR, item.get_field_value()).as_str());
                }
                if counter != available_list.len() {
                    res.push_str(COMMON_SEPARATOR);
                }
            }
            self.available.set_region_statement(format!("{}{}{}{}", self.available.get_keyword(), COMMON_SEPARATOR, res, END_SEPARATOR).as_str());
        } else {
            let stmt = String::from(self.available.get_region_single());
            self.available.set_region_statement(&stmt);
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