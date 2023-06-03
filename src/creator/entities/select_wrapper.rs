use std::cell::RefCell;
use super::{Statements, Criteria, JudgeCriteria, SQLField, SQLRegion, RegionField, RegionImpl, FETCH, TIMEOUT, AND, OR, COMMON_SEPARATOR, END_SEPARATOR, NEXT_SEPARATOR, AS, SELECT, FROM, WHERE, Wrapper, EQ, NEQ, LT, GT, LTE, GTE, ORDER_BY, GROUP_BY, SPLIT_AT, START_AT, LIMIT_BY, TimeUnit, MILLISECOND, SECOND, HOUR, MINUTE};


///=================================================<br>
/// @params:
/// <ol>
///     <li>keyword:关键字</li>
///     <li>stmt:最终语句</li>
///     <li>table_region:表区域（生成FROM @TableName）</li>
///     <li>field_region:字段区域（生成SELECT @Fields...）</li>
///     <li>where_region:条件区域，where子句（生成WHERE @Condition）</li>
///     <li>handle_region:处理区域（生成ORDER BY，GROUP BY等子句）</li>
/// </ol>
/// @date:2023/5/27<br>
/// @description:Select语句包装器,生成select语句，实现查询数据操作<br>
/// @example:<br>
///=================================================
#[derive(Debug, Clone)]
pub struct SelectWrapper {
    ///关键字
    keyword: Statements,
    available: SQLRegion,
    ///表区域（生成FROM @TableName）
    table_region: SQLRegion,
    ///字段区域（生成SELECT @Fields...）
    field_region: SQLRegion,
    ///条件区域，where子句（生成WHERE @Condition）
    where_region: SQLRegion,
    ///处理区域（生成ORDER BY，GROUP BY等子句）
    handle_region: HandleRegion,
    ///直接使用通用查询语句（自己手写）设置为true，用于判断
    define: bool,
}


///=================================================<br>
/// @params:
/// <ol>
///     <li>name:名称</li>
///     <li>as_name:别名</li>
/// </ol>
/// @date:2023/5/27<br>
/// @description:字段结构体<br>
///=================================================
#[derive(Debug, Clone)]
pub struct Field {
    name: String,
    as_name: String,
}

impl Field {
    pub fn new(name: &str) -> Self {
        Field {
            name: String::from(name),
            as_name: String::new(),
        }
    }
    ///添加字段别名<br>
    ///@example： name AS stuName
    pub fn as_name(&mut self, name: &str) -> &mut Self {
        self.as_name = String::from(name);
        self
    }
    pub fn get_as_name(&self) -> &str {
        &self.as_name
    }
    pub fn get_name(&self) -> &str {
        &self.name
    }
}


///=================================================<br>
/// @params:
/// <ol>
///     <li>field:字段名</li>
///     <li>other:<br>
///     RAND()是随机排序的函数，用于返回一个随机数值，可以用于对结果集进行随机排序。<br>
///     COLLATE可以指定排序时使用的字符集和排序规则，用于解决不同字符集或语言环境下的排序问题。<br>
///     NUMERIC可以指定对数字类型的排序方式，如按数字大小排序。</li>
///     <li>ordered:排列（升序|降序）</li>
/// </ol>
/// @date:2023/5/27
/// @description: Order By 子句结构体
///=================================================
pub struct OrderCondition {
    field: String,
    other: String,
    ordered: Ordered,
    condition_str: String,
}

///排序枚举
/// ASC:升序|DESC:降序
pub enum Ordered {
    ASC,
    DESC,
}

impl OrderCondition {
    ///全参new方法
    ///一般来说不需要使用到other去设置字符集，数组排序所以直接传入空字符串切片即可
    pub fn new(field: &str, other: &str, ordered: Ordered) -> Self {
        OrderCondition {
            field: String::from(field),
            other: String::from(other),
            ordered,
            condition_str: String::new(),
        }
    }
    ///无参构造，默认使用升序ASC
    pub fn new_no_args() -> Self {
        OrderCondition {
            field: String::new(),
            other: String::new(),
            ordered: Ordered::ASC,
            condition_str: String::new(),
        }
    }
    pub fn field(&mut self, field: &str) -> &mut Self {
        self.field = String::from(field);
        self
    }
    pub fn other(&mut self, other: &str) -> &mut Self {
        self.other = String::from(other);
        self
    }
    pub fn ordered(&mut self, ordered: Ordered) -> &mut Self {
        self.ordered = ordered;
        self
    }
    pub fn combine(&mut self) -> &str {
        let mut ordered = String::new();
        match self.ordered {
            Ordered::ASC => ordered = String::from("ASC"),
            Ordered::DESC => ordered = String::from("DESC"),
        };
        if self.other.is_empty() {
            self.condition_str = format!("{}{}{}", self.field, COMMON_SEPARATOR, ordered);
        } else {
            self.condition_str = format!("{}{}{}{}{}", self.field, COMMON_SEPARATOR, self.other, COMMON_SEPARATOR, ordered);
        }
        self.condition_str.as_str()
    }
}


///=================================================<br>
/// @params:
/// <ol>
///     <li></li>
/// </ol>
/// @date:2023/5/27<br>
/// @description:条件子语句包括order,split.start,timeout...<br>
///=================================================
#[derive(Debug, Clone)]
pub struct HandleRegion {
    order: SQLRegion,
    split: SQLRegion,
    group: SQLRegion,
    limit: SQLRegion,
    start: SQLRegion,
    timeout: SQLRegion,
    fetch: SQLRegion,
    handle_str: String,
}

impl HandleRegion {
    pub fn new() -> Self {
        HandleRegion {
            order: SQLRegion::new(RegionField::Multi(Vec::new()), ORDER_BY),
            split: SQLRegion::new(RegionField::Multi(Vec::new()), SPLIT_AT),
            group: SQLRegion::new(RegionField::Multi(Vec::new()), GROUP_BY),
            limit: SQLRegion::new(RegionField::Single(String::new()), LIMIT_BY),
            start: SQLRegion::new(RegionField::Single(String::new()), START_AT),
            timeout: SQLRegion::new(RegionField::Single(String::new()), TIMEOUT),
            fetch: SQLRegion::new(RegionField::Multi(Vec::new()), FETCH),
            handle_str: String::new(),
        }
    }
    fn build_order_stmt(&mut self) {
        let fields = self.order.get_region_multi();
        let mut counter = 0;
        let mut field_stmt = String::new();
        for field in fields {
            counter += 1;
            field_stmt.push_str(field.get_field_value());
            if counter != fields.len() {
                field_stmt.push_str(NEXT_SEPARATOR);
            }
        }

        self.order.set_region_statement(&field_stmt);
    }
    fn build_split_stmt(&mut self) {
        let splits = self.split.get_region_multi();
        let mut counter = 0;
        let mut split_stmt = String::new();
        for split in splits {
            counter += 1;
            split_stmt.push_str(split.get_field_value());
            if counter != splits.len() {
                split_stmt.push_str(NEXT_SEPARATOR);
            }
        }

        self.split.set_region_statement(&split_stmt);
    }
    fn build_group_stmt(&mut self) {
        let fields = self.group.get_region_multi();
        let mut counter = 0;
        let mut field_stmt = String::new();
        for field in fields {
            counter += 1;
            field_stmt.push_str(field.get_field_value());
            if counter != fields.len() {
                field_stmt.push_str(NEXT_SEPARATOR);
            }
        }

        self.group.set_region_statement(&field_stmt);
    }
    fn build_limit_stmt(&mut self) {
        let limit = String::from(self.limit.get_region_single());
        self.limit.set_region_statement(&limit);
    }
    fn build_start_stmt(&mut self) {
        let start = String::from(self.start.get_region_single());
        self.start.set_region_statement(&start);
    }
    fn build_timeout_stmt(&mut self) {
        let timout = String::from(self.timeout.get_region_single());
        self.timeout.set_region_statement(&timout);
    }
    fn build_fetch_stmt(&mut self) {
        let fields = self.fetch.get_region_multi();
        let mut counter = 0;
        let mut field_stmt = String::new();
        for field in fields {
            counter += 1;
            field_stmt.push_str(field.get_field_value());
            if counter != fields.len() {
                field_stmt.push_str(NEXT_SEPARATOR);
            }
        }
        self.fetch.set_region_statement(&field_stmt);
    }
    fn handle_str_push(&mut self, keyword: &str, stmt: &str, msg: &str) {
        if !stmt.is_empty() {
            let push_stmt = format!("{}{}{}{}", keyword, COMMON_SEPARATOR, stmt, COMMON_SEPARATOR);
            self.handle_str.push_str(&push_stmt);
        } else {
            if !msg.is_empty() {
                panic!("{}", msg);
            }
        }
    }
    pub fn combine(&mut self) -> &str {
        self.build_split_stmt();
        self.handle_str_push(
            String::from(self.split.get_keyword()).as_str(),
            String::from(self.split.get_region_statement()).as_str(),
            "",
        );
        self.build_group_stmt();
        self.handle_str_push(
            String::from(self.group.get_keyword()).as_str(),
            String::from(self.group.get_region_statement()).as_str(),
            "",
        );
        self.build_order_stmt();
        self.handle_str_push(
            String::from(self.order.get_keyword()).as_str(),
            String::from(self.order.get_region_statement()).as_str(),
            "",
        );
        self.build_limit_stmt();
        self.handle_str_push(
            String::from(self.limit.get_keyword()).as_str(),
            String::from(self.limit.get_region_statement()).as_str(),
            "",
        );
        self.build_start_stmt();
        self.handle_str_push(
            String::from(self.start.get_keyword()).as_str(),
            String::from(self.start.get_region_statement()).as_str(),
            "",
        );
        self.build_fetch_stmt();
        self.handle_str_push(
            String::from(self.fetch.get_keyword()).as_str(),
            String::from(self.fetch.get_region_statement()).as_str(),
            "",
        );
        self.build_timeout_stmt();
        self.handle_str_push(
            String::from(self.timeout.get_keyword()).as_str(),
            String::from(self.timeout.get_region_statement()).as_str(),
            "",
        );
        self.handle_str.as_str()
    }
}


impl Wrapper for SelectWrapper {
    fn new() -> Self {
        SelectWrapper {
            keyword: Statements::SELECT,
            available: SQLRegion::new(RegionField::Multi(Vec::new()), SELECT),
            table_region: SQLRegion::new(RegionField::Single(String::new()), FROM),
            field_region: SQLRegion::new(RegionField::Multi(Vec::new()), SELECT),
            where_region: SQLRegion::new(RegionField::Single(String::new()), WHERE),
            handle_region: HandleRegion::new(),
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
            self.available.set_region_statement(format!("{}{}", complete_stmt, END_SEPARATOR).as_str());

        }
        self.available.get_region_statement()
    }

    fn get_available(&self) -> &SQLRegion {
        &self.available
    }
    fn get_keyword(&self) -> &Statements {
        &self.keyword
    }
}

impl SelectWrapper {
    ///=================================================<br>
    /// @params:
    /// <ol>
    ///     <li>query:查询语句,全写</li>
    /// </ol>
    /// @return:<br>
    /// @date:2023/5/28<br>
    /// @description:通用查询<br>
    ///=================================================
    pub fn select(&mut self, query: &str) -> &mut Self {
        self.define = true;
        self.available.set_region_statement(query);
        self
    }
    ///=================================================<br>
    /// @params:
    /// <ol>
    ///     <li>fields:字段</li>
    /// </ol>
    /// @return:SelectWrapper<br>
    /// @date:2023/5/28<br>
    /// @description:查询字段<br>
    ///=================================================
    pub fn select_fields(&mut self, fields: &Vec<Field>) -> &mut Self {
        for field in fields {
            self.select_field(field);
        }
        self
    }
    ///=================================================<br>
    /// @params:
    /// <ol>
    ///     <li>fields:字段</li>
    /// </ol>
    /// @return:<br>
    /// @date:2023/5/28<br>
    /// @description:查询字段-单个查询<br>
    ///=================================================
    pub fn select_field(&mut self, field: &Field) -> &mut Self {
        let mut field_stmt = String::new();
        if field.get_as_name().is_empty() {
            field_stmt = format!("{}", field.get_name());
        } else {
            field_stmt = format!("{}{}{}{}{}", field.get_name(), COMMON_SEPARATOR, AS, COMMON_SEPARATOR, field.get_as_name());
        }
        let field_value = SQLField::new("FIELD", &field_stmt);
        self.field_region.push_set(&field_value);
        self
    }
    ///=================================================<br>
    /// @params:
    /// <ol>
    ///     <li>table_name:表名</li>
    /// </ol>
    /// @return:<br>
    /// @date:2023/5/28<br>
    /// @description:
    /// 设置查询的表<br />
    /// set table name which you wanna select<br>
    ///=================================================
    pub fn from(&mut self, table_name: &str) -> &mut Self {
        self.table_region.set_region_single(table_name);
        self
    }
    /// 构建where子句
    /// build a where statement
    pub fn where_condition(&mut self, condition: &Criteria) -> &mut Self {
        let condition = SQLField::new("CRITERIA", &condition.combine());
        self.where_region.push_set(&condition);
        self
    }
    ///构建OrderBy子句
    /// build an OrderBy statement
    pub fn order_by(&mut self, conditions: &mut Vec<OrderCondition>) -> &mut Self {
        for order in conditions {
            let order_condition = SQLField::new(ORDER_BY, order.combine());
            self.handle_region.order.push_set(&order_condition);
        }
        self
    }
    ///构建GroupBy子句
    ///
    pub fn group_by(&mut self, conditions: &Vec<&str>) -> &mut Self {
        for group in conditions.clone() {
            let group_condition = SQLField::new(GROUP_BY, group);
            self.handle_region.group.push_set(&group_condition);
        }
        self
    }
    /// 构建SplitAt子句
    pub fn split_at(&mut self, conditions: &Vec<&str>) -> &mut Self {
        for split in conditions.clone() {
            let split_condition = SQLField::new(SPLIT_AT, split);
            self.handle_region.split.push_set(&split_condition);
        }
        self
    }
    ///构建Fetch子句
    pub fn fetch(&mut self, fields: &Vec<&str>) -> &mut Self {
        for field in fields.clone() {
            let fetch_condition = SQLField::new(FETCH, field);
            self.handle_region.fetch.push_set(&fetch_condition);
        }
        self
    }
    ///构建延时Timeout子句
    pub fn timeout(&mut self, time: usize, unit: TimeUnit) -> &mut Self {
        let mut res = String::new();
        match unit {
            TimeUnit::MILLISECOND => res = String::from(MILLISECOND),
            TimeUnit::SECOND => res = String::from(SECOND),
            TimeUnit::MINUTE => res = String::from(MINUTE),
            TimeUnit::HOUR => res = String::from(HOUR)
        };
        self.handle_region.timeout.set_region_single(format!("{}{}", time, &res).as_str());
        self
    }
    ///构建limit子句
    pub fn limit_by(&mut self, pieces: usize) -> &mut Self {
        self.handle_region.limit.set_region_single(format!("{}", pieces).as_str());
        self
    }
    ///构建Start子句
    pub fn start_at(&mut self, pieces: usize) -> &mut Self {
        self.handle_region.start.set_region_single(format!("{}", pieces).as_str());
        self
    }
    fn build_field_stmt(&mut self) {
        let fields = self.field_region.get_region_multi();
        let mut counter = 0;
        let mut field_stmt = String::new();
        for field in fields {
            counter += 1;
            field_stmt.push_str(field.get_field_value());
            if counter != fields.len() {
                field_stmt.push_str(NEXT_SEPARATOR);
            }
        }

        self.field_region.set_region_statement(&field_stmt);
    }
    fn build_table_stmt(&mut self) {
        let table = String::from(self.table_region.get_region_single());
        self.table_region.set_region_statement(&table);
    }
    fn build_where_stmt(&mut self) {
        let condition = String::from(self.where_region.get_region_single());
        self.where_region.set_region_statement(&condition);
    }
    fn build_handle_stmt(&mut self) -> &str {
        self.handle_region.combine()
    }
    fn build_available(&mut self) {
        //field_region
        self.build_field_stmt();
        self.available_push(
            String::from(self.field_region.get_keyword()).as_str(),
            String::from(self.field_region.get_region_statement()).as_str(),
            "SELECT statement is the basic statement that must be constructed during select!",
        );
        //table_region
        self.build_table_stmt();
        self.available_push(
            String::from(self.table_region.get_keyword()).as_str(),
            String::from(self.table_region.get_region_statement()).as_str(),
            "FROM statement is the basic statement that must be constructed during select!",
        );
        //where_region
        self.build_where_stmt();
        self.available_push(
            String::from(self.where_region.get_keyword()).as_str(),
            String::from(self.where_region.get_region_statement()).as_str(),
            "",
        );
        let other = String::from(self.handle_region.combine());
        self.available_push(
            "",
            &other,
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