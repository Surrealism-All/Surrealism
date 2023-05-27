use super::{RegionImpl, COMMON_SEPARATOR, SET, END_SEPARATOR, CREATE, EQUAL_SEPARATOR, NEXT_SEPARATOR, IS_SEPARATOR, RETURN, NONE, DIFF, AFTER, BEFORE, RAND, ULID, UUID, CONTENT, AvailData, Wrapper, TableId, IdFunction};
use crate::{ParseSQL, SQLParser, handle_str, check_available_order};
use serde::{Deserialize, Serialize};


///create语句包装器
/// 生成create语句，实现添加数据操作
/// example:
/// use surrealism::{CreateWrapper,TableId,IdFunction}
/// let mut create_table = CreateWrapper::new();
///
///     create_table.create("user")
///         .id(TableId::<IdFunction>::Fun(IdFunction::RAND))
///         .set("name","zhangsan")
///         .set("email","syf2002@out.com")
///         .return_field("name");
///
/// let res = db.commit(create_table).await?;
///
#[derive(Debug, Clone)]
pub struct CreateWrapper {
    ///关键词
    pub keyword: String,
    ///可获取值
    pub available: Vec<AvailData>,
    ///语句
    pub stmt: String,
    ///内容区域
    pub content_region: ContentRegion,
    ///构建区域
    pub create_region: CreateRegion,
    ///返回区域
    pub return_region: ReturnRegion,
}

///构建第一部分tablename和Id
/// 形如：
///person:100
#[derive(Debug, Clone)]
pub struct CreateRegion {
    id: String,
    table: String,
    keyword: String,
    create_str: String,
    available_data: Vec<AvailData>,
}

impl RegionImpl for CreateRegion {
    fn combine(&mut self) -> &str {
        self.create_str = format!("{}{}{}{}{}", self.keyword, COMMON_SEPARATOR, self.table, IS_SEPARATOR, self.id);
        self.create_str.as_str()
    }
}

#[derive(Debug, Clone)]
pub struct ReturnRegion {
    return_type: ReturnType,
    return_str: String,
    keyword: String,
    available_data: Vec<AvailData>,
}

impl RegionImpl for ReturnRegion {
    fn combine(&mut self) -> &str {
        let res = self.available_data[0].value();
        self.return_str = format!("{}{}{}", self.keyword, COMMON_SEPARATOR, res);
        self.return_str.as_str()
    }
}


#[derive(Debug, Clone)]
pub struct ContentRegion {
    content_type: ContentType,
    content_str: String,
    available_data: Vec<AvailData>,
    keyword: String,
}

impl RegionImpl for ContentRegion {
    fn combine(&mut self) -> &str {
        match self.content_type {
            ContentType::SET => {
                let mut counter = 0;
                let mut stmt = String::new();
                for data in &self.available_data {
                    counter += 1;
                    if counter == self.available_data.len() {
                        stmt.push_str(data.value());
                    } else {
                        stmt.push_str(data.value());
                        stmt.push_str(NEXT_SEPARATOR);
                    }
                }
                self.content_str = format!("{}{}{}", SET, COMMON_SEPARATOR, stmt);
            }
            ContentType::CONTENT => {
                self.content_str = format!("{}{}{}",CONTENT,COMMON_SEPARATOR,self.available_data[0].value())
            }
            ContentType::NONE => panic!("{}", "you must create a content")
        };


        self.content_str.as_str()
    }
}


#[derive(Debug, Clone)]
pub enum ContentType {
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
    FIELD,
}

impl Wrapper for CreateWrapper {
    fn new() -> Self {
        CreateWrapper {
            keyword: String::from(CREATE),
            available: Vec::new(),
            stmt: String::new(),
            content_region: ContentRegion {
                content_type: ContentType::NONE,
                content_str: String::new(),
                available_data: Vec::new(),
                keyword: String::from(NONE),
            },
            create_region: CreateRegion {
                id: String::from(RAND),
                table: String::new(),
                keyword: String::from(CREATE),
                create_str: String::new(),
                available_data: Vec::new(),
            },
            return_region: ReturnRegion {
                return_type: ReturnType::NONE,
                return_str: String::new(),
                keyword: String::from(RETURN),
                available_data: Vec::new(),
            },
        }
    }

    fn commit(&mut self) -> &str {//匹配解析SET或CONTENT


        self.stmt = format!("{}{}{}{}{}{}", self.create_region.combine(), COMMON_SEPARATOR, self.content_region.combine(), COMMON_SEPARATOR, self.return_region.combine(), END_SEPARATOR);
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
        self.create_region.table = String::from(table_name);
        self
    }
    /// 创建表的ID , ID使用TableId进行构建!
    /// create table with id , use TableId enum to create!
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
                tmp_res = n.to_string();
            }
            TableId::Str(s) => {
                tmp_res = String::from(s);
            }
            _ => {
                panic!("{}", "Cannot find type in TableId enum")
            }
        }
        self.create_region.id = tmp_res;
        self
    }
    ///SET方式构建字段
    pub fn set<T: Serialize>(&mut self, field_name: &'static str, value: T) -> &mut Self {
        let len = self.get_available().len();
        match self.content_region.content_type {
            ContentType::CONTENT => panic!("you cannot use set and content together!"),
            ContentType::SET => (),
            ContentType::NONE => {
                self.content_region.content_type = ContentType::SET;
                self.content_region.keyword = String::from(SET);
            }
        };
        let value = AvailData::new(len, String::from("set_value"), format!("{}{}{}", field_name, EQUAL_SEPARATOR, handle_str(serde_json::to_string(&value).unwrap().as_str())), false, false);
        self.content_region.available_data.push(value);
        self
    }
    ///CONTENT方式构建字段
    pub fn content<T: Serialize + SQLParser>(&mut self, content_obj: T) -> &mut Self {
        match self.content_region.content_type {
            ContentType::SET => panic!("you cannot use set and content together!"),
            ContentType::CONTENT => panic!("you cannot use content twice!"),
            ContentType::NONE => {
                self.content_region.content_type = ContentType::CONTENT;
                self.content_region.keyword = String::from(CONTENT);
            }
        };

        let obj_str = content_obj.parse_sql();
        let len = self.get_available().len();
        let value = AvailData::new(len, String::from("content_value"), obj_str, false, false);
        self.content_region.available_data.push(value);
        self
    }
    ///返回NONE
    pub fn return_none(&mut self) -> &mut Self {
        if self.check_return() {
            panic!("{}", "you cannot use return twice!");
        } else {
            self.return_region.return_type = ReturnType::NONE;
            let len = self.get_available().len();
            let value = AvailData::new(len, String::from("return_value"), String::from(NONE), false, false);
            self.return_region.available_data.push(value);
        }
        self
    }
    ///返回DIFF
    pub fn return_diff(&mut self) -> &mut Self {
        if self.check_return() {
            panic!("{}", "you cannot use return twice!");
        } else {
            self.return_region.return_type = ReturnType::DIFF;
            let len = self.get_available().len();
            let value = AvailData::new(len, String::from("return_value"), String::from(DIFF), false, false);
            self.return_region.available_data.push(value);
        }
        self
    }
    ///返回BEFORE
    pub fn return_before(&mut self) -> &mut Self {
        if self.check_return() {
            panic!("{}", "you cannot use return twice!");
        } else {
            self.return_region.return_type = ReturnType::BEFORE;
            let len = self.get_available().len();
            let value = AvailData::new(len, String::from("return_value"), String::from(BEFORE), false, false);
            self.return_region.available_data.push(value);
        }
        self
    }
    ///返回AFTER
    pub fn return_after(&mut self) -> &mut Self {
        if self.check_return() {
            panic!("{}", "you cannot use return twice!");
        } else {
            self.return_region.return_type = ReturnType::AFTER;
            let len = self.get_available().len();
            let value = AvailData::new(len, String::from("return_value"), String::from(AFTER), false, false);
            self.return_region.available_data.push(value);
        }
        self
    }
    ///返回某个字段
    pub fn return_field(&mut self, field_name: &str) -> &mut Self {
        if self.check_return() {
            panic!("{}", "you cannot use return twice!");
        } else {
            self.return_region.return_type = ReturnType::FIELD;
            let len = self.get_available().len();
            let value = AvailData::new(len, String::from("return_value"), String::from(field_name), false, false);
            self.return_region.available_data.push(value);
        }
        self
    }
    ///检查是否设置了返回，设置了：true，未设置：false
    fn check_return(&mut self) -> bool {
        return !self.return_region.available_data.is_empty();
    }
}


