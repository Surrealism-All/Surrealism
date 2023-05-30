// use std::cell::RefCell;
// use std::collections::HashMap;
// use std::ops::Add;
// use std::sync::{Arc, Mutex};
// use super::{RegionImpl, FETCH, TIMEOUT, AND, OR, COMMON_SEPARATOR, END_SEPARATOR, EQUAL_SEPARATOR, NEXT_SEPARATOR, AS, IS_SEPARATOR, SELECT, FROM, WHERE, AvailData, Wrapper, EQ, NEQ, LT, GT, LTE, GTE, ORDER_BY, GROUP_BY, SPLIT_AT, START_AT, LIMIT_BY, TimeUnit, MILLISECOND, SECOND, HOUR, MINUTE};
// use log::error;
// use crate::{ParseSQL, SQLParser, handle_str, check_available_order};
// use serde::{Deserialize, Serialize};
//
//
// ///=================================================<br>
// /// @params:
// /// <ol>
// ///     <li>keyword:关键字</li>
// ///     <li>stmt:最终语句</li>
// ///     <li>table_region:表区域（生成FROM @TableName）</li>
// ///     <li>field_region:字段区域（生成SELECT @Fields...）</li>
// ///     <li>where_region:条件区域，where子句（生成WHERE @Condition）</li>
// ///     <li>handle_region:处理区域（生成ORDER BY，GROUP BY等子句）</li>
// /// </ol>
// /// @date:2023/5/27<br>
// /// @description:Select语句包装器,生成select语句，实现查询数据操作<br>
// /// @example:<br>
// ///=================================================
// #[derive(Debug, Clone)]
// pub struct SelectWrapper {
//     ///关键字
//     keyword: String,
//     available: Vec<AvailData>,
//     stmt: String,
//     ///表区域（生成FROM @TableName）
//     table_region: TableRegion,
//     ///字段区域（生成SELECT @Fields...）
//     field_region: FieldRegion,
//     ///条件区域，where子句（生成WHERE @Condition）
//     where_region: WhereRegion,
//     ///处理区域（生成ORDER BY，GROUP BY等子句）
//     handle_region: HandleRegion,
//
// }
//
//
// ///=================================================<br>
// /// @params:
// /// <ol>
// ///     <li>name:名称</li>
// ///     <li>as_name:别名</li>
// /// </ol>
// /// @date:2023/5/27<br>
// /// @description:字段结构体<br>
// ///=================================================
// #[derive(Debug, Clone)]
// pub struct Field {
//     name: String,
//     as_name: String,
// }
//
// impl Field {
//     pub fn new(name: &str) -> Self {
//         Field {
//             name: String::from(name),
//             as_name: String::new(),
//         }
//     }
//     ///添加字段别名<br>
//     ///@example： name AS stuName
//     pub fn as_name(&mut self, name: &str) -> &mut Self {
//         self.as_name = String::from(name);
//         self
//     }
//     pub fn get_as_name(&self) -> &str {
//         &self.as_name
//     }
//     pub fn get_name(&self) -> &str {
//         &self.name
//     }
// }
//
// ///=================================================<br>
// /// @params:
// /// <ol>
// ///     <li>field_str:字段结果</li>
// ///     <li>available_data:值存储器</li>
// ///     <li>keyword:关键字</li>
// /// </ol>
// /// @date:2023/5/27<br>
// /// @description:字段区域结构体，用于构建（生成SELECT @Fields...）<br>
// ///=================================================
// #[derive(Debug, Clone)]
// pub struct FieldRegion {
//     field_str: String,
//     available_data: Vec<AvailData>,
//     keyword: String,
// }
//
// impl RegionImpl for FieldRegion {
//     fn combine(&mut self) -> &str {
//         let mut stmt = String::new();
//         let mut counter = 0;
//         for data in &self.available_data {
//             counter += 1;
//             if counter == self.available_data.len() {
//                 stmt.push_str(format!("{}{}", COMMON_SEPARATOR, data.value()).as_str());
//             } else {
//                 stmt.push_str(format!("{}{}{}", data.value(), COMMON_SEPARATOR, NEXT_SEPARATOR).as_str());
//             }
//         }
//         self.field_str = format!("{}{}{}", self.keyword, COMMON_SEPARATOR, stmt);
//         self.field_str.as_str()
//     }
// }
//
// ///=================================================<br>
// /// @params:
// /// <ol>
// ///     <li>table:表名</li>
// ///     <li>table_str:表区域结果</li>
// ///     <li>keyword:关键字</li>
// /// </ol>
// /// @date:2023/5/27<br>
// /// @description: 表区域结构体<br>
// ///=================================================
// #[derive(Debug, Clone)]
// pub struct TableRegion {
//     table: String,
//     table_str: String,
//     keyword: String,
// }
//
// impl RegionImpl for TableRegion {
//     fn combine(&mut self) -> &str {
//         self.table_str = format!("{}{}{}", self.keyword, COMMON_SEPARATOR, self.table);
//         self.table_str.as_str()
//     }
// }
//
// #[derive(Debug, Clone)]
// pub struct WhereRegion {
//     available_data: Vec<AvailData>,
//     where_str: String,
//     keyword: String,
// }
//
// impl RegionImpl for WhereRegion {
//     fn combine(&mut self) -> &str {
//         if !self.available_data.is_empty() {
//             self.where_str = format!("{}{}{}", self.keyword, COMMON_SEPARATOR, self.available_data[0].value());
//         }
//         self.where_str.as_str()
//     }
// }
//
// #[derive(Debug, Clone)]
// pub struct HandleConditionSingle {
//     available_data: String,
//     handle_str: String,
//     keyword: String,
// }
//
// impl RegionImpl for HandleConditionSingle {
//     fn combine(&mut self) -> &str {
//         self.handle_str = format!("{}{}{}", self.keyword, COMMON_SEPARATOR, self.available_data);
//         self.handle_str.as_str()
//     }
// }
//
// ///=================================================<br>
// /// @params:
// /// <ol>
// ///     <li></li>
// /// </ol>
// /// @date:2023/5/27<br>
// /// @description: 条件子句结构体
// ///=================================================
// #[derive(Debug, Clone)]
// pub struct HandleCondition {
//     available_data: Vec<String>,
//     handle_str: String,
//     keyword: String,
// }
//
// impl RegionImpl for HandleCondition {
//     fn combine(&mut self) -> &str {
//         let mut res = String::new();
//         let mut counter: usize = 0;
//         for data in &self.available_data {
//             counter += 1;
//             res.push_str(data);
//             if !counter.eq(&self.available_data.len()) {
//                 res.push_str(format!("{}{}{}", COMMON_SEPARATOR, NEXT_SEPARATOR, COMMON_SEPARATOR).as_str());
//             }
//         }
//
//         self.handle_str = format!("{}{}{}", self.keyword, COMMON_SEPARATOR, res);
//         self.handle_str.as_str()
//     }
// }
//
// #[derive(Debug, Clone)]
// pub struct HandleOrder {
//     available_data: Vec<String>,
//     handle_str: String,
//     keyword: String,
// }
//
// impl RegionImpl for HandleOrder {
//     fn combine(&mut self) -> &str {
//         let mut res = String::new();
//         let mut counter: usize = 0;
//         for data in &self.available_data {
//             counter += 1;
//             res.push_str(data);
//             if !counter.eq(&self.available_data.len()) {
//                 res.push_str(format!("{}{}{}", COMMON_SEPARATOR, NEXT_SEPARATOR, COMMON_SEPARATOR).as_str());
//             }
//         }
//
//         self.handle_str = format!("{}{}{}", self.keyword, COMMON_SEPARATOR, res);
//         self.handle_str.as_str()
//     }
// }
//
// ///=================================================<br>
// /// @params:
// /// <ol>
// ///     <li>field:字段名</li>
// ///     <li>other:<br>
// ///     RAND()是随机排序的函数，用于返回一个随机数值，可以用于对结果集进行随机排序。<br>
// ///     COLLATE可以指定排序时使用的字符集和排序规则，用于解决不同字符集或语言环境下的排序问题。<br>
// ///     NUMERIC可以指定对数字类型的排序方式，如按数字大小排序。</li>
// ///     <li>ordered:排列（升序|降序）</li>
// /// </ol>
// /// @date:2023/5/27
// /// @description: Order By 子句结构体
// ///=================================================
// pub struct OrderCondition {
//     field: String,
//     other: String,
//     ordered: Ordered,
//     condition_str: String,
// }
//
// ///排序枚举
// /// ASC:升序|DESC:降序
// pub enum Ordered {
//     ASC,
//     DESC,
// }
//
// impl OrderCondition {
//     ///全参new方法
//     ///一般来说不需要使用到other去设置字符集，数组排序所以直接传入空字符串切片即可
//     pub fn new(field: &str, other: &str, ordered: Ordered) -> Self {
//         OrderCondition {
//             field: String::from(field),
//             other: String::from(other),
//             ordered,
//             condition_str: String::new(),
//         }
//     }
//     ///无参构造，默认使用升序ASC
//     pub fn new_no_args() -> Self {
//         OrderCondition {
//             field: String::new(),
//             other: String::new(),
//             ordered: Ordered::ASC,
//             condition_str: String::new(),
//         }
//     }
//     pub fn field(&mut self, field: &str) -> &mut Self {
//         self.field = String::from(field);
//         self
//     }
//     pub fn other(&mut self, other: &str) -> &mut Self {
//         self.other = String::from(other);
//         self
//     }
//     pub fn ordered(&mut self, ordered: Ordered) -> &mut Self {
//         self.ordered = ordered;
//         self
//     }
// }
//
// impl RegionImpl for OrderCondition {
//     fn combine(&mut self) -> &str {
//         let mut ordered = String::new();
//         match self.ordered {
//             Ordered::ASC => ordered = String::from("ASC"),
//             Ordered::DESC => ordered = String::from("DESC"),
//         };
//         if self.other.is_empty() {
//             self.condition_str = format!("{}{}{}", self.field, COMMON_SEPARATOR, ordered);
//         } else {
//             self.condition_str = format!("{}{}{}{}{}", self.field, COMMON_SEPARATOR, self.other, COMMON_SEPARATOR, ordered);
//         }
//         self.condition_str.as_str()
//     }
// }
//
// ///=================================================<br>
// /// @params:
// /// <ol>
// ///     <li></li>
// /// </ol>
// /// @date:2023/5/27<br>
// /// @description:条件子语句包括order,split.start,timeout...<br>
// ///=================================================
// #[derive(Debug, Clone)]
// pub struct HandleRegion {
//     order: HandleOrder,
//     split: HandleCondition,
//     group: HandleCondition,
//     limit: HandleConditionSingle,
//     start: HandleConditionSingle,
//     timeout: HandleConditionSingle,
//     fetch: HandleCondition,
//     handle_str: String,
// }
//
// impl HandleRegion {
//     pub fn new() -> Self {
//         HandleRegion {
//             order: HandleOrder {
//                 available_data: Vec::new(),
//                 handle_str: String::new(),
//                 keyword: String::from(ORDER_BY),
//             },
//             split: HandleCondition {
//                 available_data: Vec::new(),
//                 handle_str: String::new(),
//                 keyword: String::from(SPLIT_AT),
//             },
//             group: HandleCondition {
//                 available_data: Vec::new(),
//                 handle_str: String::new(),
//                 keyword: String::from(GROUP_BY),
//             },
//             limit: HandleConditionSingle {
//                 available_data: String::new(),
//                 handle_str: String::new(),
//                 keyword: String::from(LIMIT_BY),
//             },
//             start: HandleConditionSingle {
//                 available_data: String::new(),
//                 handle_str: String::new(),
//                 keyword: String::from(START_AT),
//             },
//             timeout: HandleConditionSingle {
//                 available_data: String::new(),
//                 handle_str: String::new(),
//                 keyword: String::from(TIMEOUT),
//             },
//             fetch: HandleCondition {
//                 available_data: Vec::new(),
//                 handle_str: String::new(),
//                 keyword: String::from(FETCH),
//             },
//             handle_str: String::new(),
//         }
//     }
// }
//
// impl RegionImpl for HandleRegion {
//     fn combine(&mut self) -> &str {
//         if !self.split.available_data.is_empty() {
//             self.handle_str.push_str(self.split.combine());
//         }
//
//         if !self.group.available_data.is_empty() {
//             self.handle_str.push_str(COMMON_SEPARATOR);
//             self.handle_str.push_str(self.group.combine());
//         }
//
//         if !self.order.available_data.is_empty() {
//             self.handle_str.push_str(COMMON_SEPARATOR);
//             self.handle_str.push_str(self.order.combine());
//         }
//
//         if !self.limit.available_data.is_empty() {
//             self.handle_str.push_str(COMMON_SEPARATOR);
//             self.handle_str.push_str(self.limit.combine());
//         }
//
//         if !self.start.available_data.is_empty() {
//             self.handle_str.push_str(COMMON_SEPARATOR);
//             self.handle_str.push_str(self.start.combine());
//         }
//
//         if !self.fetch.available_data.is_empty() {
//             self.handle_str.push_str(COMMON_SEPARATOR);
//             self.handle_str.push_str(self.fetch.combine());
//         }
//
//         if !self.timeout.available_data.is_empty() {
//             self.handle_str.push_str(COMMON_SEPARATOR);
//             self.handle_str.push_str(self.timeout.combine());
//         }
//
//         self.handle_str.as_str()
//     }
// }
//
// ///Where条件构建
// #[derive(Debug, Clone)]
// pub struct Criteria {
//     ///比较符
//     judge: JudgeCriteria,
//     ///自己构建，对于如：WHERE count(->experience->organisation) > 3
//     ///则需要自己构建
//     define: String,
//     core: String,
//     comparator: String,
//     complex: RefCell<Vec<String>>,
// }
//
// // ///构建复杂的条件语句
// // /// 例如需要AND,OR的条件语句
// // pub struct CriteriaComplex {
// //     core: String,
// //     comparator: String,
// //
// // }
//
// impl Criteria {
//     pub fn new() -> Self {
//         Criteria {
//             judge: JudgeCriteria::NONE,
//             define: String::new(),
//             core: String::new(),
//             comparator: String::new(),
//             complex: RefCell::new(Vec::new()),
//         }
//     }
//     pub fn combine(&self) -> String {
//         let mut res = String::new();
//         if self.define.is_empty() {
//             let mut sign: &str = "";
//             match self.judge {
//                 JudgeCriteria::Eq => {
//                     sign = EQ;
//                 }
//                 JudgeCriteria::Neq => {
//                     sign = NEQ;
//                 }
//                 JudgeCriteria::Lt => {
//                     sign = LT;
//                 }
//                 JudgeCriteria::Gt => {
//                     sign = GT;
//                 }
//                 JudgeCriteria::Lte => {
//                     sign = LTE;
//                 }
//                 JudgeCriteria::Gte => {
//                     sign = GTE;
//                 }
//                 JudgeCriteria::NONE => ()
//             }
//             res = format!("{}{}{}{}{}", self.core, COMMON_SEPARATOR, sign, COMMON_SEPARATOR, self.comparator);
//         } else {
//             res = String::from(&self.define);
//         }
//         res
//     }
//     /// 自定义写入条件
//     /// 由于有些条件通过Criteria当前提供的方法无法直接构建
//     /// 例如：count(->experience->organisation) > 3
//     /// 此时就需要调用者直接进行手写
//     pub fn define(&mut self, define_str: &str) {
//         self.define = String::from(define_str);
//     }
//     fn buildCore(&mut self, core: &str) {
//         let res = self.complexBuild();
//         if res.is_empty() {
//             self.core = String::from(core);
//         } else {
//             self.core = res;
//         }
//     }
//     /// 相等条件
//     /// core = comparator
//     pub fn eq(&mut self, core: &str, comparator: &str) -> &mut Self {
//         self.buildCore(core);
//         self.judge = JudgeCriteria::Eq;
//         self.comparator = String::from(comparator);
//         self
//     }
//     /// 大于条件
//     /// core > comparator
//     pub fn gt(&mut self, core: &str, comparator: &str) -> &mut Self {
//         self.buildCore(core);
//         self.judge = JudgeCriteria::Gt;
//         self.comparator = String::from(comparator);
//         self
//     }
//     pub fn lt(&mut self, core: &str, comparator: &str) -> &mut Self {
//         self.buildCore(core);
//         self.judge = JudgeCriteria::Lt;
//         self.comparator = String::from(comparator);
//         self
//     }
//     pub fn neq(&mut self, core: &str, comparator: &str) -> &mut Self {
//         self.buildCore(core);
//         self.judge = JudgeCriteria::Neq;
//         self.comparator = String::from(comparator);
//         self
//     }
//     pub fn lte(&mut self, core: &str, comparator: &str) -> &mut Self {
//         self.buildCore(core);
//         self.judge = JudgeCriteria::Lte;
//         self.comparator = String::from(comparator);
//         self
//     }
//     pub fn gte(&mut self, core: &str, comparator: &str) -> &mut Self {
//         self.buildCore(core);
//         self.judge = JudgeCriteria::Gte;
//         self.comparator = String::from(comparator);
//         self
//     }
//     pub fn and(&self, left: &str, right: &str) -> String {
//         let res = format!("{}{}{}{}{}", left, COMMON_SEPARATOR, AND, COMMON_SEPARATOR, right);
//         self.complex.borrow_mut().push(res.clone());
//         res
//     }
//     pub fn or(&self, left: &str, right: &str) -> String {
//         let res = format!("{}{}{}{}{}", left, COMMON_SEPARATOR, OR, COMMON_SEPARATOR, right);
//         self.complex.borrow_mut().push(res.clone());
//         res
//     }
//     pub fn complexBuild(&mut self) -> String {
//         if !self.complex.borrow().is_empty() {
//             let mut counter: usize = 0;
//             for core_complex in &*self.complex.borrow() {
//                 counter += 1;
//
//                 if counter.eq(&self.complex.borrow().len()) {
//                     self.core = replace_str(&self.core, core_complex);
//                 } else {
//                     let res = replace_str(&self.core, core_complex);
//                     self.core = format!("( {} )", res);
//                 }
//             }
//         }
//         self.core.clone()
//     }
// }
//
// ///=================================================<br>
// /// @params:
// /// <ol>
// ///     <li>core:源目标</li>
// ///     <li>replace:替换目标</li>
// /// </ol>
// /// @return:<br>
// /// @date:2023/5/28<br>
// /// @description:将Criteria的complexBuild方法中替换`()`<br>
// ///=================================================
// fn replace_str(core: &str, replace: &str) -> String {
//     let value = core.replace("( ", "").replace(" )", "");
//     let res = replace.replace(&value, core);
//     res.clone()
// }
//
// ///=================================================<br>
// /// @params:
// /// <ol>
// ///     <li>Eq:等于</li>
// ///     <li>Lt:小于</li>
// ///     <li>Gt:大于</li>
// ///     <li>Neq:不等于</li>
// ///     <li>Lte:小于等于</li>
// ///     <li>Gte:大于等于</li>
// /// </ol>
// /// @date:2023/5/28<br>
// /// @description:JudgeCriteria判断符枚举
// ///=================================================
// #[derive(Debug, Clone)]
// pub enum JudgeCriteria {
//     Eq,
//     Lt,
//     Gt,
//     Neq,
//     Lte,
//     Gte,
//     NONE,
// }
//
//
// impl Wrapper for SelectWrapper {
//     fn new() -> Self {
//         let mut available: Vec<AvailData> = Vec::new();
//         let tmp1 = AvailData::new(0, String::from(SELECT), String::from(SELECT), false, false);
//         let tmp2 = AvailData::new(1, "COMMON_SEPARATOR".to_string(), String::from(COMMON_SEPARATOR), true, false);
//         available.push(tmp1);
//         available.push(tmp2);
//         SelectWrapper {
//             keyword: String::from(SELECT),
//             available,
//             stmt: String::new(),
//             table_region: TableRegion {
//                 table: String::new(),
//                 table_str: String::new(),
//                 keyword: String::from(FROM),
//             },
//             field_region: FieldRegion {
//                 field_str: String::new(),
//                 available_data: vec![],
//                 keyword: String::from(SELECT),
//             },
//             where_region: WhereRegion {
//                 available_data: vec![],
//                 where_str: String::new(),
//                 keyword: String::from(WHERE),
//             },
//             handle_region: HandleRegion::new(),
//         }
//     }
//
//
//     fn commit(&mut self) -> &str {
//         if self.stmt.is_empty() {
//             self.stmt = format!("{}{}{}{}{}{}{}{}", self.field_region.combine(), COMMON_SEPARATOR, self.table_region.combine(), COMMON_SEPARATOR, self.where_region.combine(), COMMON_SEPARATOR, self.handle_region.combine(), END_SEPARATOR);
//         }
//         &self.stmt
//     }
//
//     fn get_keyword(&self) -> &str {
//         &self.keyword
//     }
//
//     fn get_available(&self) -> &Vec<AvailData> {
//         &self.available
//     }
// }
//
// impl SelectWrapper {
//     ///=================================================<br>
//     /// @params:
//     /// <ol>
//     ///     <li>query:查询语句,全写</li>
//     /// </ol>
//     /// @return:<br>
//     /// @date:2023/5/28<br>
//     /// @description:通用查询<br>
//     ///=================================================
//     pub fn select(&mut self, query: &str) -> &mut Self {
//         // let len = self.get_available().len();
//         // let tmp = AvailData::new(len, String::from("query"), String::from(query), false, false);
//         // self.available.push(tmp);
//         self.stmt = String::from(query);
//         self
//     }
//     ///=================================================<br>
//     /// @params:
//     /// <ol>
//     ///     <li>fields:字段</li>
//     /// </ol>
//     /// @return:SelectWrapper<br>
//     /// @date:2023/5/28<br>
//     /// @description:查询字段<br>
//     ///=================================================
//     pub fn select_fields(&mut self, fields: &Vec<Field>) -> &mut Self {
//         for field in fields {
//             self.select_field(field);
//         }
//         self
//     }
//     ///=================================================<br>
//     /// @params:
//     /// <ol>
//     ///     <li>fields:字段</li>
//     /// </ol>
//     /// @return:<br>
//     /// @date:2023/5/28<br>
//     /// @description:查询字段-单个查询<br>
//     ///=================================================
//     pub fn select_field(&mut self, field: &Field) -> &mut Self {
//         let len = self.get_available().len();
//         let mut field_stmt = String::new();
//         if field.get_as_name().is_empty() {
//             field_stmt = format!("{}", field.get_name());
//         } else {
//             field_stmt = format!("{}{}{}{}{}", field.get_name(), COMMON_SEPARATOR, AS, COMMON_SEPARATOR, field.get_as_name());
//         }
//         let value = AvailData::new(len, String::from("field"), field_stmt, false, false);
//         self.field_region.available_data.push(value);
//         self
//     }
//     ///=================================================<br>
//     /// @params:
//     /// <ol>
//     ///     <li>table_name:表名</li>
//     /// </ol>
//     /// @return:<br>
//     /// @date:2023/5/28<br>
//     /// @description:
//     /// 设置查询的表<br />
//     /// set table name which you wanna select<br>
//     ///=================================================
//     pub fn from(&mut self, table_name: &str) -> &mut Self {
//         self.table_region.table = String::from(table_name);
//         self
//     }
//     /// 构建where子句
//     /// build a where statement
//     pub fn where_condition(&mut self, condition: &Criteria) -> &mut Self {
//         let len = self.get_available().len();
//         let value = AvailData::new(len, String::from("where_criteria"), condition.combine(), false, false);
//         self.where_region.available_data.push(value);
//         self
//     }
//     ///构建OrderBy子句
//     /// build an OrderBy statement
//     pub fn order_by(&mut self, conditions: &mut Vec<OrderCondition>) -> &mut Self {
//         for condition in conditions {
//             let res = condition.combine();
//             self.handle_region.order.available_data.push(String::from(res));
//         }
//         self
//     }
//     ///构建GroupBy子句
//     ///
//     pub fn group_by(&mut self, conditions: &Vec<&str>) -> &mut Self {
//         for group_condition in conditions.clone() {
//             self.handle_region.group.available_data.push(String::from(group_condition))
//         }
//         self
//     }
//     /// 构建SplitAt子句
//     pub fn split_at(&mut self, conditions: &Vec<&str>) -> &mut Self {
//         for split_condition in conditions.clone() {
//             self.handle_region.split.available_data.push(String::from(split_condition))
//         }
//         self
//     }
//     ///构建Fetch子句
//     pub fn fetch(&mut self, fields: &Vec<&str>) -> &mut Self {
//         for field in fields.clone() {
//             self.handle_region.fetch.available_data.push(String::from(field))
//         }
//         self
//     }
//     ///构建延时Timeout子句
//     pub fn timeout(&mut self, time: usize, unit: TimeUnit) -> &mut Self {
//         let mut res = String::new();
//         match unit {
//             TimeUnit::MILLISECOND => res = String::from(MILLISECOND),
//             TimeUnit::SECOND => res = String::from(SECOND),
//             TimeUnit::MINUTE => res = String::from(MINUTE),
//             TimeUnit::HOUR => res = String::from(HOUR)
//         };
//         self.handle_region.timeout.available_data = format!("{}{}", time, res);
//         self
//     }
//     ///构建limit子句
//     pub fn limit_by(&mut self, pieces: usize) -> &mut Self {
//         self.handle_region.limit.available_data = format!("{}", pieces);
//         self
//     }
//     ///构建Start子句
//     pub fn start_at(&mut self, pieces: usize) -> &mut Self {
//         self.handle_region.start.available_data = format!("{}", pieces);
//         self
//     }
// }
//
//
