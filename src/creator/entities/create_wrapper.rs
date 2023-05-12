use std::collections::HashMap;
use std::ops::Add;
use std::sync::{Arc, Mutex};
use super::{COMMON_SEPARATOR,END_SEPARATOR,EQUAL_SEPARATOR,RETURN,NONE,DIFF,AFTER,BEFORE,RAND,ULID,UUID,CONTENT,Wrapper,TableId,IdRange,IdFunction};
use log::error;
use crate::{ParseSQL,SQLParser,handle_str};
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
#[derive(Debug,Clone)]
pub struct CreateWrapper {
    pub keyword: String,
    pub available: Arc<Mutex<HashMap<&'static str, String>>>,
    pub stmt: String,
    pub field_type: FieldType,
    pub return_type: ReturnType,
}

#[derive(Debug,Clone)]
pub enum FieldType {
    SET,
    CONTENT,
    NONE,
}
///Create语句返回类型
#[derive(Debug,Clone)]
pub enum ReturnType {
    NONE,
    BEFORE,
    AFTER,
    DIFF,
    FIELD(String),
}

impl Wrapper for CreateWrapper {
    fn new() -> Self {
        CreateWrapper {
            keyword: "CREATE".to_string(),
            available: Arc::new(Mutex::new(HashMap::new())),
            stmt: String::from("CREATE").add(COMMON_SEPARATOR),
            field_type: FieldType::NONE,
            return_type: ReturnType::AFTER,
        }
    }

    fn and(&mut self) -> &mut Self {
        self.stmt = format!("{}{}", self.stmt, COMMON_SEPARATOR);
        self
    }

    fn build(&mut self) -> &mut Self {
        //匹配解析SET或CONTENT
        match self.field_type {
            FieldType::NONE => error!("{}","you must use SET or CONTENT to create the table"),
            FieldType::SET => {
                let fields = self.available.lock().unwrap().clone();
                let mut stmt: Vec<String> = Vec::new();
                for (fname, fvalue) in fields {
                    stmt.push(handle_str(format!("{}={}", fname, fvalue).as_str()));
                }
                let len = stmt.len();
                let mut counter = 0;
                for s in stmt {
                    counter += 1;
                    if counter == len {
                        self.stmt = format!("{}{}", self.stmt, s)
                    } else {
                        self.stmt = format!("{}{},", self.stmt, s)
                    }
                }
            }
            FieldType::CONTENT => (),
        }
        self.stmt = format!("{}{}", self.stmt, COMMON_SEPARATOR);
        //解析RETURN
        match &self.return_type {
            ReturnType::NONE => self.stmt = format!("{}{}{}{}", self.stmt, RETURN, COMMON_SEPARATOR, NONE),
            ReturnType::DIFF => self.stmt = format!("{}{}{}{}", self.stmt, RETURN, COMMON_SEPARATOR, DIFF),
            ReturnType::AFTER => self.stmt = format!("{}{}{}{}", self.stmt, RETURN, COMMON_SEPARATOR, AFTER),
            ReturnType::BEFORE => self.stmt = format!("{}{}{}{}", self.stmt, RETURN, COMMON_SEPARATOR, BEFORE),
            ReturnType::FIELD(field) => self.stmt = format!("{}{}{}{}", self.stmt, RETURN, COMMON_SEPARATOR, field),
        }
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

impl CreateWrapper {
    ///创建表名称
    /// create table with name
    pub fn create(&mut self, table_name: &str) -> &mut Self {
        self.stmt = format!("{}{}", self.stmt, table_name);
        self
    }
    /// 创建表的ID , ID使用TableId进行构建!
    /// create table with id , use TableId enum to create!
    pub fn id<'a, T: Serialize>(&mut self, table_id: TableId<'a, T>) -> &mut Self {
        match table_id {
            TableId::Array(arr) => {
                //序列化
                let res = serde_json::to_string(&arr).unwrap();
                self.stmt = format!("{}{}{}", self.stmt, ":", res);
            }
            TableId::Object(obj) => {
                let res = serde_json::to_string(&obj).unwrap();
                self.stmt = format!("{}{}{}", self.stmt, ":", res);
            }
            TableId::Fun(fun_type) => {
                match fun_type {
                    IdFunction::ULID => {
                        self.stmt = format!("{}{}{}", self.stmt, ":", ULID);
                    }
                    IdFunction::UUID => {
                        self.stmt = format!("{}{}{}", self.stmt, ":", UUID);
                    }
                    IdFunction::RAND => {
                        self.stmt = format!("{}{}{}", self.stmt, ":", RAND);
                    }
                }
            }
            TableId::Range { min, max } => {
                let min_str = serde_json::to_string(&min).unwrap();
                let max_str = serde_json::to_string(&max).unwrap();

                self.stmt = format!("{}{}{}{}{}", self.stmt, ":", min_str, "..", max_str);
            }
            TableId::Num(n) => {
                self.stmt = format!("{}{}{}", self.stmt, ":", n);
            }
            TableId::Str(s) => {
                self.stmt = format!("{}{}{}", self.stmt, ":", s);
            }
            _ => {
                panic!("{}", "Cannot find type in TableId enum")
            }
        }

        self
    }
    ///SET方式构建字段
    pub fn set<T: Serialize>(&mut self, field_name: &'static str, value: T) -> &mut Self {
        self.available.lock().unwrap().insert(field_name, serde_json::to_string(&value).unwrap());
        self.field_type = FieldType::SET;
        self
    }
    ///CONTENT方式构建字段
    pub fn content<T: Serialize + SQLParser>(&mut self, content_obj: T) -> &mut Self {
        let obj_str = content_obj.parse_sql();
        self.stmt = format!("{}{}{}{}", self.stmt, CONTENT, COMMON_SEPARATOR, obj_str);
        self.field_type = FieldType::CONTENT;
        self
    }
    ///返回NONE
    pub fn return_none(&mut self) -> &mut Self {
        self.return_type = ReturnType::NONE;
        self
    }
    ///返回DIFF
    pub fn return_diff(&mut self) -> &mut Self {
        self.return_type = ReturnType::DIFF;
        self
    }
    ///返回BEFORE
    pub fn return_before(&mut self) -> &mut Self {
        self.return_type = ReturnType::BEFORE;
        self
    }
    ///返回AFTER
    pub fn return_after(&mut self) -> &mut Self {
        self.return_type = ReturnType::AFTER;
        self
    }
    ///返回某个字段
    pub fn return_field(&mut self, field_name: &'static str) -> &mut Self {
        self.return_type = ReturnType::FIELD(field_name.to_string());
        self
    }
}


