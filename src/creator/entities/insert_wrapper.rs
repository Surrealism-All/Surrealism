use serde::Serialize;
use serde_json;
use crate::creator::entities::RegionImpl;
use crate::utils::handle_str;
use super::{Statements, SQLRegion, SQLField, RegionField, Wrapper, INSERT, INTO, VALUES, CONTENT, COMMON_SEPARATOR, NEXT_SEPARATOR, END_SEPARATOR};

///=================================================<br>
///
/// INSERT [ IGNORE ] INTO @what
/// 	[ @value
/// 	  | (@fields) VALUES (@values)
/// 		[ ON DUPLICATE KEY UPDATE @field = @value ... ]
/// 	]
/// ;
/// @params:
/// <ol>
///     <li>keyword:关键字</li>
/// </ol>
/// @date:2023/5/27<br>
/// @description:Select语句包装器,生成select语句，实现查询数据操作<br>
/// @example:<br>
///=================================================
#[derive(Debug, Clone)]
pub struct InsertWrapper {
    ///关键字
    keyword: Statements,
    available: SQLRegion,
    ///table name
    table: SQLField,
    ///fields-values,键值对映射
    key_region: SQLRegion,
    value_region: SQLRegion,
    ///contents,使用该字段说明调用者使用content方式Insert，而非传统key-value
    content_region: SQLRegion,
    ///define
    define: bool,
}

impl Wrapper for InsertWrapper {
    fn new() -> Self {
        InsertWrapper {
            keyword: Statements::INSERT,
            available: SQLRegion::new(RegionField::Multi(Vec::new()), INSERT),
            table: SQLField::new(INTO, ""),
            key_region: SQLRegion::new(RegionField::Multi(Vec::new()), "KEY"),
            content_region: SQLRegion::new(RegionField::Multi(Vec::new()), CONTENT),
            define: false,
            value_region: SQLRegion::new(RegionField::Multi(Vec::new()), VALUES),
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

impl InsertWrapper {
    ///设置插入的表名，若要设置ID直接使用:跟在表名后面
    ///Set the inserted table name. If you want to set the ID, use it directly: follow the table name
    pub fn insert_into(&mut self, table_name: &str) -> &mut Self {
        self.table.set_field_value(table_name);
        self
    }
    ///插入单条记录单个字段
    pub fn set<T: Serialize>(&mut self, key: &str, value: T) -> &mut Self {
        let kv_k = SQLField::new("KEY", key);
        let value_str = handle_str(&serde_json::to_string(&value).unwrap());
        let kv_v = SQLField::new("VALUE", &value_str);
        self.key_region.get_region_multi_mut().push(kv_k);
        self.value_region.get_region_multi_mut().push(kv_v);
        self
    }
    ///使用非传统方式插入单条(CONTENT)
    pub fn insert_one<T: Serialize>(&mut self, obj: &T) -> &mut Self {
        let sql = handle_str(serde_json::to_string(obj).unwrap().as_str());
        let content = SQLField::new(CONTENT, &sql);
        self.content_region.get_region_multi_mut().push(content);
        self
    }
    ///使用非传统方式插入多条(CONTENT)
    pub fn insert_many<T: Serialize>(&mut self, objs: &Vec<T>) -> &mut Self {
        for obj in objs {
            self.insert_one(obj);
        }
        self
    }
    ///通用插入自己写插入语句
    pub fn insert(&mut self, stmt: &str) -> &mut Self {
        self.define = true;
        self.available.set_region_statement(stmt);
        self
    }
    fn build_key_region(&mut self) {
        let fields = self.key_region.get_region_multi();
        let mut key_stmt = String::new();
        for i in 0..fields.len() {
            key_stmt.push_str(fields[i].get_field_value());
            if i != fields.len() - 1 {
                key_stmt.push_str(NEXT_SEPARATOR);
            }
        }

        self.key_region.set_region_statement(format!("({})", &key_stmt).as_str());
    }
    fn build_value_region(&mut self) {
        let fields = self.value_region.get_region_multi();
        let mut value_stmt = String::new();
        for i in 0..fields.len() {
            value_stmt.push_str(fields[i].get_field_value());
            if i != fields.len() - 1 {
                value_stmt.push_str(NEXT_SEPARATOR);
            }
        }
        self.value_region.set_region_statement(format!("({})", &value_stmt).as_str());
    }
    fn build_content_region(&mut self) {
        let fields = self.content_region.get_region_multi();
        let mut res = String::new();
        let mut value_stmt = String::new();
        let field_len = fields.len();
        if field_len == 1 {
            res = String::from(fields[0].get_field_value());
        } else if field_len > 1 {
            for i in 0..field_len {
                value_stmt.push_str(fields[i].get_field_value());
                if i != field_len - 1 {
                    value_stmt.push_str(NEXT_SEPARATOR);
                }
            }
            res = format!("[{}{}{}]", COMMON_SEPARATOR, &value_stmt, COMMON_SEPARATOR);
        }
        self.content_region.set_region_statement(&res);
    }
    fn build_available(&mut self) {
        self.build_content_region();
        self.available_push(
            String::from(self.table.get_keyword()).as_str(),
            String::from(self.table.get_field_value()).as_str(),
            "you mut set table name to insert!",
        );
        if self.content_region.get_region_statement().is_empty() {
            self.build_key_region();
            self.available_push(
                "",
                String::from(self.key_region.get_region_statement()).as_str(),
                "",
            );
            self.build_value_region();
            self.available_push(
                "VALUES",
                String::from(self.value_region.get_region_statement()).as_str(),
                "",
            );
        } else {
            self.available_push(
                "",
                String::from(self.content_region.get_region_statement()).as_str(),
                "if you use content insert , you must set struct data!",
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