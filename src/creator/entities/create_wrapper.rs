use super::{ContentType, RegionImpl, Statements, SQLRegion, SQLField, RegionField, COMMON_SEPARATOR, SET, END_SEPARATOR, CREATE, EQUAL_SEPARATOR, NEXT_SEPARATOR, IS_SEPARATOR, RETURN, NONE, DIFF, AFTER, BEFORE, RAND, ULID, UUID, CONTENT, Wrapper, TableId, IdFunction};
use crate::{handle_str};
use serde::{Serialize};


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
    keyword: Statements,
    ///可获取值
    available: SQLRegion,
    ///内容区域
    content_region: SQLRegion,
    content_type: ContentType,
    ///构建区域
    create_region: SQLRegion,
    ///返回区域
    return_region: SQLRegion,
}


impl Wrapper for CreateWrapper {
    fn new() -> Self {
        let available_regions = RegionField::Multi(vec![
            SQLField::new(CREATE, ""),
            SQLField::new(SET, ""),
            SQLField::new(RETURN, ""),
        ]);

        CreateWrapper {
            // keyword: String::from(CREATE),
            keyword: Statements::CREATE,
            available: SQLRegion::new(available_regions, CREATE),
            content_region: SQLRegion::new(RegionField::Multi(Vec::new()), NONE),
            content_type: ContentType::NONE,
            create_region: SQLRegion::new(RegionField::Multi(Vec::new()), CREATE),
            return_region: SQLRegion::new(RegionField::Single(String::new()), RETURN),

        }
    }

    fn commit(&mut self) -> &str {
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
        self.available.set_region_statement(format!("{}{}", complete_stmt, END_SEPARATOR).as_str());
        self.available.get_region_statement()
    }

    fn get_keyword(&self) -> &Statements {
        &self.keyword
    }


    fn get_available(&self) -> &SQLRegion {
        &self.available
    }
}

impl CreateWrapper {
    ///创建表名称
    /// create table with name
    pub fn create(&mut self, table_name: &str) -> &mut Self {
        match self.create_region.get_region_field_mut() {
            RegionField::Multi(ref mut fields) => {
                let field = SQLField::new("TABLE_NAME", table_name);
                fields.push(field);
            }
            RegionField::Single(_) => {
                panic!("CreateWrapper's create_region use RegionField::Multi , if you see this panic please send email to developer!")
            }
        }
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
        }

        match self.create_region.get_region_field_mut() {
            RegionField::Multi(ref mut fields) => {
                let field = SQLField::new("TABLE_ID", &tmp_res);
                fields.push(field);
            }
            RegionField::Single(_) => {
                panic!("CreateWrapper's create_region use RegionField::Multi , if you see this panic please send email to developer!")
            }
        }
        self
    }
    ///SET方式构建字段
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
                panic!("ContentType::MERGE and ContentType::PATCH is not allowed to be used in Create statement!");
            }
        };
        match self.content_region.get_region_field_mut() {
            RegionField::Multi(ref mut fields) => {
                let field_value = format!("{}{}{}", field_name, EQUAL_SEPARATOR, handle_str(serde_json::to_string(&value).unwrap().as_str()));
                let field = SQLField::new(SET, &field_value);
                fields.push(field);
            }
            RegionField::Single(_) => {
                panic!("CreateWrapper's content_region(SET) use RegionField::Multi , if you see this panic please send email to developer!")
            }
        }
        self
    }
    ///CONTENT方式构建字段
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
            },
            _ => {
                panic!("ContentType::MERGE and ContentType::PATCH is not allowed to be used in Create statement!");
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
    ///返回某个字段
    pub fn return_field(&mut self, field_name: &str) -> &mut Self {
        self.return_build(field_name);
        self
    }
    ///检查是否设置了返回，设置了：true，未设置：false
    fn check_return(&mut self) {
        match self.return_region.get_region_field() {
            RegionField::Multi(_) => {
                panic!("CreateWrapper's return_region use RegionField::Single,if you see this error please send email to developer!")
            }
            RegionField::Single(field) => {
                if !field.is_empty() {
                    panic!("{}", "you cannot use return twice!");
                }
            }
        }
    }
    fn return_build(&mut self, return_str: &str) {
        self.check_return();
        match self.return_region.get_region_field_mut() {
            RegionField::Multi(_) => {
                panic!("CreateWrapper's return_region use RegionField::Single , if you see this error please send email to developer!")
            }
            RegionField::Single(ref mut field) => {
                *field = String::from(return_str);
            }
        }
    }
    fn build_create_stmt(&mut self) {
        let fields = self.create_region.get_region_multi();
        let mut table_name = "";
        let mut table_id = "";
        for field in fields {
            match field.get("TABLE_NAME") {
                None => (),
                Some(name) => {
                    table_name = name;
                }
            }
            match field.get("TABLE_ID") {
                None => (),
                Some(id) => {
                    table_id = id;
                }
            }
        }

        let stmt = format!("{}{}{}", table_name, IS_SEPARATOR, table_id);
        self.create_region.set_region_statement(&stmt);
    }

    fn build_content_stmt(&mut self) {
        let mut content_str = String::new();
        let tmp_content = self.content_region.clone();
        match self.content_type {
            ContentType::SET => {
                let mut set_str = String::new();
                let mut counter = 0;
                for field in tmp_content.get_region_multi() {
                    counter += 1;
                    set_str.push_str(field.get_field_value());
                    if counter != tmp_content.get_region_multi().len() {
                        set_str.push_str(NEXT_SEPARATOR)
                    }
                }
                content_str.push_str(&set_str);
                self.available.get_region_multi_mut()[1].set_keyword(SET);
            }
            ContentType::CONTENT => {
                content_str.push_str(tmp_content.get_region_single());
                self.available.get_region_multi_mut()[1].set_keyword(CONTENT);
            }
            ContentType::NONE => {
                panic!("CreateWrapper is used to building create statement , if you just wanna check the database please use SelectWrapper")
            },
            _ => {
                panic!("ContentType::MERGE and ContentType::PATCH is not allowed to be used in Create statement!");
            }
        };
        self.content_region.set_region_statement(&content_str);
    }

    fn build_available(&mut self) {
        self.build_create_stmt();
        self.build_content_stmt();

        let stmts = vec![
            self.create_region.get_region_statement(),
            self.content_region.get_region_statement(),
            self.return_region.get_region_single(),
        ];
        for i in 0..self.available.get_region_multi().len() {
            self.available.get_region_multi_mut()[i].set_field_value(stmts[i]);
        }
    }
}


