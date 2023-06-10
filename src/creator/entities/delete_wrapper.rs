use serde::Serialize;
use crate::creator::entities::RegionImpl;
use super::{Statements, SQLField, SQLRegion, RegionField, Wrapper, Criteria,  TableId, TimeUnit, HOUR, DAY, COMMON_SEPARATOR, END_SEPARATOR, MINUTE, SECOND, MILLISECOND, IS_SEPARATOR, DELETE, RETURN, WHERE, TIMEOUT, NONE, BEFORE, AFTER, DIFF};

///DELETE @targets
/// 	[ WHERE @condition ]
/// 	[ RETURN [ NONE | BEFORE | AFTER | DIFF | @projections ... ]
/// 	[ TIMEOUT @duration ]
/// 	[ PARALLEL ]
/// ;
///=================================================
///
/// @params:
/// <ol>
///     <li></li>
/// </ol>
/// @date:2023/6/3
///
/// @description:
///
///=================================================
#[derive(Debug, Clone)]
pub struct DeleteWrapper {
    ///关键字
    keyword: Statements,
    available: SQLRegion,
    ///这里多用SQLField进行构建
    /// 主要是快速简单，语句单一
    table_region: SQLField,
    where_region: SQLField,
    timeout_region: SQLField,
    return_region: SQLField,
    ///define
    define: bool,
}

impl Wrapper for DeleteWrapper {
    fn new() -> Self {
        DeleteWrapper {
            keyword: Statements::DELETE,
            available: SQLRegion::new(RegionField::Multi(Vec::new()), DELETE),
            table_region: SQLField::new(DELETE, ""),
            where_region: SQLField::new(WHERE, ""),
            timeout_region: SQLField::new(TIMEOUT, ""),
            return_region: SQLField::new(RETURN, ""),
            define: false,
        }
    }

    fn commit(&mut self) -> &str {
        if !self.define {
            self.build_available();
            let stmt_fn = || -> String{
                let mut res = String::new();
                let available_list = self.available.get_region_multi();
                let mut counter = 0;
                for item in available_list {
                    counter += 1;
                    res.push_str(format!("{}{}{}", item.get_keyword(), COMMON_SEPARATOR, item.get_field_value()).as_str());
                    if counter != available_list.len() {
                        res.push_str(COMMON_SEPARATOR);
                    }
                }
                res
            };
            let mut available_copy = self.available.clone();
            let complete_stmt = available_copy.combine(&stmt_fn());
            self.available.set_region_statement(format!("{}{}{}{}", self.available.get_keyword(), COMMON_SEPARATOR, complete_stmt, END_SEPARATOR).as_str());
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

impl DeleteWrapper {
    ///自定义DELETE语句
    pub fn delete(&mut self, stmt: &str) -> &mut Self {
        self.define = true;
        self.available.set_region_statement(stmt);
        self
    }
    ///设置需要删除的表名
    pub fn from(&mut self, table_name: &str) -> &mut Self {
        self.table_region.set_field_value(table_name);
        self
    }
    /// 设置要删除的表的ID
    pub fn id<T: Serialize>(&mut self, table_id: TableId<T>) -> &mut Self {
        let mut tmp_res = String::new();
        match table_id {
            TableId::Array(arr) => {
                //序列化
                let res = serde_json::to_string(&arr).unwrap();
                tmp_res = res;
            }
            TableId::Object(obj) => {
                let res = serde_json::to_string(&obj).unwrap();
                tmp_res = res;
            }
            TableId::Fun(fun_type) => {
                panic!("DeleteWrapper is not allowed : TableId::Fun!")
            }
            TableId::Range { min, max } => {
                let min_str = serde_json::to_string(&min).unwrap();
                let max_str = serde_json::to_string(&max).unwrap();
                tmp_res = format!("{}{}{}", min_str, "..", max_str);
            }
            TableId::Num(n) => {
                tmp_res = n.to_string();
            }
            TableId::Str(s) => {
                tmp_res = String::from(s);
            }
        }
        self.table_region.set_field_value(format!("{}{}{}", self.table_region.get_field_value(), IS_SEPARATOR, tmp_res).as_str());
        self
    }
    /// 构建where子句
    /// build a where statement
    pub fn where_condition(&mut self, condition: &Criteria) -> &mut Self {
        let condition_value = condition.combine();
        self.where_region.set_field_value(&condition_value);
        self
    }
    ///构建延时Timeout子句
    pub fn timeout(&mut self, time: usize, unit: TimeUnit) -> &mut Self {
        let mut res = "";
        match unit {
            TimeUnit::MILLISECOND => res = MILLISECOND,
            TimeUnit::SECOND => res = SECOND,
            TimeUnit::MINUTE => res = MINUTE,
            TimeUnit::HOUR => res = HOUR,
            TimeUnit::DAY => res = DAY,
        };
        self.timeout_region.set_field_value(format!("{}{}", time, &res).as_str());
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
    ///返回某个字段
    pub fn return_field(&mut self, field_name: &str) -> &mut Self {
        self.return_build(field_name);
        self
    }
    fn return_build(&mut self, return_str: &str) {
        if self.return_region.get_field_value().is_empty() {
            self.return_region.set_field_value(return_str);
        } else {
            panic!("{}", "you cannot use return twice!");
        }
    }
    fn build_available(&mut self) {
        self.available_push(
            "",
            String::from(self.table_region.get_field_value()).as_str(),
            "",
        );
        self.available_push(
            String::from(self.where_region.get_keyword()).as_str(),
            String::from(self.where_region.get_field_value()).as_str(),
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