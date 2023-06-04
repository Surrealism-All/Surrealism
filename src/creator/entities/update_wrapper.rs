use serde::Serialize;
use crate::handle_str;
use super::{ContentType, Statements, SQLField, SQLRegion, RegionField, Wrapper, RegionImpl, Criteria, JudgeCriteria, TimeUnit, TableId, NEXT_SEPARATOR, MERGE, PATCH, SET, CONTENT, EQUAL_SEPARATOR, TIMEOUT, DAY, WHERE, RETURN, UPDATE, IS_SEPARATOR, COMMON_SEPARATOR, END_SEPARATOR, HOUR, MINUTE, SECOND, MILLISECOND, NONE, BEFORE, AFTER, DIFF};

/// UPDATE @targets
/// 	[ CONTENT @value
/// 	  | MERGE @value
/// 	  | PATCH @value
/// 	  | SET @field = @value ...
/// 	]
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
/// @date:2023/6/4
///
/// @description:
///
///=================================================
#[derive(Debug, Clone)]
pub struct UpdateWrapper {
    ///关键字
    keyword: Statements,
    available: SQLRegion,
    ///这里多用SQLField进行构建
    /// 主要是快速简单，语句单一
    table_region: SQLField,
    content_region: SQLRegion,
    content_type: ContentType,
    where_region: SQLField,
    timeout_region: SQLField,
    return_region: SQLField,
    ///define
    define: bool,
}

impl Wrapper for UpdateWrapper {
    fn new() -> Self {
        UpdateWrapper {
            keyword: Statements::UPDATE,
            available: SQLRegion::new(RegionField::Multi(Vec::new()), UPDATE),
            table_region: SQLField::new(UPDATE, ""),
            content_region: SQLRegion::new(RegionField::Multi(Vec::new()), NONE),
            content_type: ContentType::NONE,
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
                    if item.get_keyword().is_empty() {
                        res.push_str(item.get_field_value());
                    } else {
                        res.push_str(format!("{}{}{}", item.get_keyword(), COMMON_SEPARATOR, item.get_field_value()).as_str());
                    }
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

impl UpdateWrapper {
    ///自定义Update语句
    pub fn update(&mut self, stmt: &str) -> &mut Self {
        self.define = true;
        self.available.set_region_statement(stmt);
        self
    }
    ///设置需要更新的表名
    pub fn from(&mut self, table_name: &str) -> &mut Self {
        self.table_region.set_field_value(table_name);
        self
    }
    /// 设置要更新的表的ID
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
            TableId::Fun(_) => {
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
    ///SET方式构建字段
    /// SET method for constructing fields
    pub fn set<T: Serialize>(&mut self, field_name: &'static str, value: T) -> &mut Self {
        match self.content_type {
            ContentType::SET => (),
            ContentType::NONE => {
                self.content_type = ContentType::SET;
                self.content_region.set_keyword(SET);
                self.content_region.set_region_field(&RegionField::Multi(Vec::new()));
            }
            _ => {
                panic!("you cannot use others and content together!")
            }
        };
        let field_value = format!("{}{}{}", field_name, EQUAL_SEPARATOR, handle_str(serde_json::to_string(&value).unwrap().as_str()));
        let field = SQLField::new(SET, &field_value);
        self.content_region.push_set(&field);
        self
    }
    ///CONTENT方式构建字段
    /// CONTENT method for constructing fields
    pub fn content<T: Serialize>(&mut self, content_obj: T) -> &mut Self {
        match self.content_type {
            ContentType::CONTENT => panic!("you cannot use content twice!"),
            ContentType::NONE => {
                self.content_type = ContentType::CONTENT;
                self.content_region.set_keyword(CONTENT);
                let content_value = handle_str(serde_json::to_string(&content_obj).unwrap().as_str());
                self.content_region.set_region_field(&RegionField::Single(content_value));
            }
            _ => panic!("you cannot use others and content together!"),
        };
        self
    }
    /// 当前版本不启用patch,版本号:surrealdb = "1.0.0-beta.9"
    #[cfg(not(feature = "update-patch"))]
    pub fn patch(&mut self, value: &str) -> &mut Self {
        match self.content_type {
            ContentType::PATCH => panic!("you cannot use patch twice!"),
            ContentType::NONE => {
                self.content_type = ContentType::PATCH;
                self.content_region.set_keyword(PATCH);
                let patch_value = handle_str(serde_json::to_string(&value).unwrap().as_str());
                self.content_region.set_region_field(&RegionField::Single(patch_value));
            }
            _ => panic!("you cannot use others and patch together!"),
        };
        self
    }
    /// MERGE @value：将新的文档合并到旧文档中，其中 @value 是要合并的文档。如果旧文档中存在相同的字段，则用新文档中相应字段的值来覆盖旧文档中的值。
    /// UPDATE person MERGE {
    /// 	settings: {
    /// 		marketing: true,
    /// 	},
    /// };
    pub fn merge<T: Serialize>(&mut self, key: &str, value: T) -> &mut Self {
        match self.content_type {
            ContentType::MERGE => (),
            ContentType::NONE => {
                self.content_type = ContentType::MERGE;
                self.content_region.set_keyword(MERGE);
                self.content_region.set_region_field(&RegionField::Multi(Vec::new()));
            }
            _ => panic!("you cannot use others and content together!"),
        };
        let merge_value = handle_str(serde_json::to_string(&value).unwrap().as_str());
        let merge_field = SQLField::new(key, &merge_value);

        self.content_region.push_set(&merge_field);
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
            TimeUnit::DAY => res = DAY
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
                panic!("UpdateWrapper is used to building update statement , if you just wanna check the database please use SelectWrapper!")
            }
            ContentType::PATCH => panic!("Not currently enabled in the current version!"),
            ContentType::CONTENT => tmp_stmt = String::from(self.content_region.get_region_single()),
            ContentType::MERGE => {
                let tmp_content = self.content_region.get_region_multi();
                let field_len = self.content_region.get_region_multi().len();
                for i in 0..field_len {
                    tmp_stmt.push_str(format!("{}{}{}", tmp_content[i].get_keyword(), IS_SEPARATOR, tmp_content[i].get_field_value()).as_str());
                    if i != field_len - 1 {
                        tmp_stmt.push_str(NEXT_SEPARATOR)
                    }
                }
                tmp_stmt = format!("{}{}{}", "{ ", tmp_stmt, " }");
            }
        };
        self.content_region.set_region_statement(&tmp_stmt);
    }
    fn build_available(&mut self) {
        self.available_push(
            "",
            String::from(self.table_region.get_field_value()).as_str(),
            "",
        );
        self.build_content_stmt();
        self.available_push(
            String::from(self.content_region.get_keyword()).as_str(),
            String::from(self.content_region.get_region_statement()).as_str(),
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