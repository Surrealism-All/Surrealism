use std::collections::HashMap;
use std::ops::Add;
use std::sync::{Arc, Mutex};
use super::{COMMON_SEPARATOR, SET, END_SEPARATOR, CREATE, EQUAL_SEPARATOR, NEXT_SEPARATOR, IS_SEPARATOR, RETURN, NONE, DIFF, AFTER, BEFORE, RAND, ULID, UUID, CONTENT, AvailData, Wrapper, TableId, IdRange, IdFunction};
use log::error;
use crate::{ParseSQL, SQLParser, handle_str, check_available_order};
use serde::{Deserialize, Serialize};


///create语句包装器
/// 生成create语句，实现添加数据操作
/// keywords:关键词
/// available:参数存储器
/// stmt:具体语句
/// field_type:设置构建类型(SET,CONTENT)
/// return_type:返回类型
/// example:
/// use surrealism::{CreateWrapper,TableId,IdFunction}
/// let mut create_table = CreateWrapper::new();
///
///     create_table.create("user")
///         .id(TableId::<IdFunction>::Fun(IdFunction::RAND))
///         .and()
///         .set("name","zhangsan")
///         .set("email","syf2002@out.com")
///         .return_field("name")
///         .build();
///
/// let res = db.commit(create_table).await?;
///
#[derive(Debug, Clone)]
pub struct CreateWrapper {
    pub keyword: String,
    pub available: Vec<AvailData>,
    pub stmt: String,
    pub field_type: FieldType,
    pub return_type: ReturnType,
}

#[derive(Debug, Clone)]
pub enum FieldType {
    SET,
    CONTENT,
    NONE,
}

///Create语句返回类型
#[derive(Debug, Clone)]
pub enum ReturnType {
    NONE,
    BEFORE,
    AFTER,
    DIFF,
    FIELD(String),
}

impl Wrapper for CreateWrapper {
    fn new() -> Self {
        let mut available: Vec<AvailData> = Vec::new();
        let tmp1 = AvailData::new(0, String::from(CREATE), String::from(CREATE), false, false);
        let tmp2 = AvailData::new(1, "COMMON_SEPARATOR".to_string(), String::from(COMMON_SEPARATOR), true, false);
        available.push(tmp1);
        available.push(tmp2);
        CreateWrapper {
            keyword: String::from(CREATE),
            available,
            stmt: String::new(),
            field_type: FieldType::NONE,
            return_type: ReturnType::AFTER,
        }
    }

    fn and(&mut self) -> &mut Self {
        let len = self.get_available().len();
        let tmp = AvailData::new(len, "COMMON_SEPARATOR".to_string(), String::from(COMMON_SEPARATOR), true, false);
        self.available.push(tmp);
        self
    }

    fn build(&mut self) -> &mut Self {

        let len = self.get_available().len();
        let tmp = AvailData::new(len, "END_SEPARATOR".to_string(), END_SEPARATOR.to_string(), false, true);
        self.available.push(tmp);
        self
    }

    fn commit(&mut self) -> &str {//匹配解析SET或CONTENT
        match self.field_type {
            FieldType::NONE => error!("{}","you must use SET or CONTENT to create the table"),
            _=>(),
        }

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

impl CreateWrapper {
    ///创建表名称
    /// create table with name
    pub fn create(&mut self, table_name: &str) -> &mut Self {
        let len = self.get_available().len();
        let tmp1 = AvailData::new(len, String::from("table"), String::from(table_name), false, false);
        self.available.push(tmp1);
        self
    }
    /// 创建表的ID , ID使用TableId进行构建!
    /// create table with id , use TableId enum to create!
    pub fn id<'a, T: Serialize>(&mut self, table_id: TableId<'a, T>) -> &mut Self {
        let len = self.get_available().len();
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
                match fun_type {
                    IdFunction::ULID => {
                        tmp_res = String::from(ULID);
                    }
                    IdFunction::UUID => {
                        tmp_res = String::from(UUID);
                    }
                    IdFunction::RAND => {
                        tmp_res = String::from(RAND);
                    }
                }
            }
            TableId::Range { min, max } => {
                let min_str = serde_json::to_string(&min).unwrap();
                let max_str = serde_json::to_string(&max).unwrap();
                tmp_res = format!("{}{}{}", min_str, "..", max_str);
            }
            TableId::Num(n) => {
                tmp_res =n.to_string();
            }
            TableId::Str(s) => {
                tmp_res = String::from(s);
            }
            _ => {
                panic!("{}", "Cannot find type in TableId enum")
            }
        }
        let tmp1 = AvailData::new(len, "IS_SEPARATOR".to_string(), String::from(IS_SEPARATOR), false, false);
        let tmp2 = AvailData::new(len + 1, String::from("table"), tmp_res, false, false);
        // let tmp3 = AvailData::new(len + 2, "COMMON_SEPARATOR".to_string(), String::from(COMMON_SEPARATOR), true, false);
        self.available.push(tmp1);
        self.available.push(tmp2);
        // self.available.push(tmp3);
        self
    }
    ///SET方式构建字段
    pub fn set<T: Serialize>(&mut self, field_name: &'static str, value: T) -> &mut Self {
        let len = self.get_available().len();
        match self.field_type {
            FieldType::NONE => {
                self.field_type = FieldType::SET;
                let tmp1 = AvailData::new(len, String::from(SET), String::from(SET), false, false);
                let tmp2 = AvailData::new(len+1, "COMMON_SEPARATOR".to_string(), String::from(COMMON_SEPARATOR), false, false);
                self.available.push(tmp1);
                self.available.push(tmp2);
            }
            FieldType::SET => {
                let tmp = AvailData::new(len, "NEXT_SEPARATOR".to_string(), String::from(NEXT_SEPARATOR), false, false);
                self.available.push(tmp);
            },
            FieldType::CONTENT => panic!("you cannot use set and content together!")
        };
        let len = self.get_available().len();
        let tmp1 = AvailData::new(len, String::from("field_name"), String::from(field_name), false, false);
        let tmp2 = AvailData::new(len + 1, "EQUAL_SEPARATOR".to_string(), String::from(EQUAL_SEPARATOR), true, false);
        let tmp3 = AvailData::new(len + 2, String::from("field_value"), handle_str(serde_json::to_string(&value).unwrap().as_str()), true, false);
        self.available.push(tmp1);
        self.available.push(tmp2);
        self.available.push(tmp3);
        self
    }
    ///CONTENT方式构建字段
    pub fn content<T: Serialize + SQLParser>(&mut self, content_obj: T) -> &mut Self {
        match self.field_type {
            FieldType::NONE => self.field_type = FieldType::CONTENT,
            FieldType::SET => panic!("you cannot use set and content together!"),
            FieldType::CONTENT => panic!("you cannot use content twice!"),
        };

        let obj_str = content_obj.parse_sql();
        let len = self.get_available().len();
        let tmp1 = AvailData::new(len, String::from(CONTENT), String::from(CONTENT), false, false);
        let tmp2 = AvailData::new(len + 1, "COMMON_SEPARATOR".to_string(), String::from(COMMON_SEPARATOR), true, false);
        let tmp3 = AvailData::new(len + 2, String::from("content_value"), obj_str, true, false);
        self.available.push(tmp1);
        self.available.push(tmp2);
        self.available.push(tmp3);
        self
    }
    ///返回NONE
    pub fn return_none(&mut self) -> &mut Self {
        if self.check_return() {
            panic!("{}", "you cannot use return twice!");
        } else {
            self.return_type = ReturnType::NONE;
            let len = self.get_available().len();
            let tmp1 = AvailData::new(len, String::from(RETURN), String::from(RETURN), false, false);
            let tmp2 = AvailData::new(len + 1, "COMMON_SEPARATOR".to_string(), String::from(COMMON_SEPARATOR), true, false);
            let tmp3 = AvailData::new(len + 2, String::from("return_type"), String::from(NONE), true, false);
            self.available.push(tmp1);
            self.available.push(tmp2);
            self.available.push(tmp3);
        }
        self
    }
    ///返回DIFF
    pub fn return_diff(&mut self) -> &mut Self {
        if self.check_return() {
            panic!("{}", "you cannot use return twice!");
        } else {
            self.return_type = ReturnType::DIFF;
            let len = self.get_available().len();
            let tmp1 = AvailData::new(len, String::from(RETURN), String::from(RETURN), false, false);
            let tmp2 = AvailData::new(len + 1, "COMMON_SEPARATOR".to_string(), String::from(COMMON_SEPARATOR), true, false);
            let tmp3 = AvailData::new(len + 2, String::from("return_type"), String::from(DIFF), true, false);
            self.available.push(tmp1);
            self.available.push(tmp2);
            self.available.push(tmp3);
        }
        self
    }
    ///返回BEFORE
    pub fn return_before(&mut self) -> &mut Self {
        if self.check_return() {
            panic!("{}", "you cannot use return twice!");
        } else {
            self.return_type = ReturnType::BEFORE;
            let len = self.get_available().len();
            let tmp1 = AvailData::new(len, String::from(RETURN), String::from(RETURN), false, false);
            let tmp2 = AvailData::new(len + 1, "COMMON_SEPARATOR".to_string(), String::from(COMMON_SEPARATOR), true, false);
            let tmp3 = AvailData::new(len + 2, String::from("return_type"), String::from(BEFORE), true, false);
            self.available.push(tmp1);
            self.available.push(tmp2);
            self.available.push(tmp3);
        }
        self
    }
    ///返回AFTER
    pub fn return_after(&mut self) -> &mut Self {
        if self.check_return() {
            panic!("{}", "you cannot use return twice!");
        } else {
            self.return_type = ReturnType::AFTER;
            let len = self.get_available().len();
            let tmp1 = AvailData::new(len, String::from(RETURN), String::from(RETURN), false, false);
            let tmp2 = AvailData::new(len + 1, "COMMON_SEPARATOR".to_string(), String::from(COMMON_SEPARATOR), true, false);
            let tmp3 = AvailData::new(len + 2, String::from("return_type"), String::from(AFTER), true, false);
            self.available.push(tmp1);
            self.available.push(tmp2);
            self.available.push(tmp3);
        }
        self
    }
    ///返回某个字段
    pub fn return_field(&mut self, field_name: &'static str) -> &mut Self {
        if self.check_return() {
            panic!("{}", "you cannot use return twice!");
        } else {
            self.return_type = ReturnType::AFTER;
            let len = self.get_available().len();
            let tmp1 = AvailData::new(len, String::from(RETURN), String::from(RETURN), false, false);
            let tmp2 = AvailData::new(len + 1, "COMMON_SEPARATOR".to_string(), String::from(COMMON_SEPARATOR), true, false);
            let tmp3 = AvailData::new(len + 2, String::from("return_type"), String::from(field_name), true, false);
            self.available.push(tmp1);
            self.available.push(tmp2);
            self.available.push(tmp3);
        }
        self
    }
    ///检查是否设置了返回，设置了：true，未设置：false
    fn check_return(&mut self) -> bool {
        let mut exist: bool = false;
        for i in &self.available {
            if i.key() == String::from("return_type") {
                exist = true;
            }
        }
        return exist.clone();
    }
}


