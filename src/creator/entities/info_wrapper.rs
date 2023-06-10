use super::{Statements, SQLField, SQLRegion, RegionField, Wrapper,  INFO, END_SEPARATOR, COMMON_SEPARATOR, TABLE, SCOPE, KV, NS, DB};

///=================================================
/// INFO FOR [
/// 	KV
/// 	| NS | NAMESPACE
/// 	| DB | DATABASE
/// 	| SCOPE @scope
/// 	| TABLE @table
/// ];
/// @params:
/// <ol>
///     <li></li>
/// </ol>
/// @date:2023/6/4
///
/// @description: 构建INFO语句,通过提供的next方法切换下一个语句
///
///=================================================
#[derive(Debug, Clone)]
pub struct InfoWrapper {
    ///关键字
    keyword: Statements,
    available: SQLRegion,
    current: usize,
}

impl Wrapper for InfoWrapper {
    fn new() -> Self {
        InfoWrapper {
            keyword: Statements::INFO,
            available: SQLRegion::new(RegionField::Multi(Vec::new()), INFO),
            current: 0,
        }
    }

    fn commit(&mut self) -> &str {
        let mut stmt = self.build_available();
        self.available.set_region_statement(format!("{}{}{}{}", self.available.get_keyword(), COMMON_SEPARATOR, &stmt, END_SEPARATOR).as_str());
        self.available.get_region_statement()
    }

    fn get_keyword(&self) -> &Statements {
        &self.keyword
    }

    fn get_available(&self) -> &SQLRegion {
        &self.available
    }
}

impl InfoWrapper {
    /// 最高层KV 命令返回关于SurrealDB系统内存在的命名空间的信息。
    /// 需要ROOT权限
    pub fn kv(&mut self) -> &mut Self {
        self.add(KV);
        self
    }
    pub fn ns(&mut self) -> &mut Self {
        self.add(NS);
        self
    }
    pub fn db(&mut self) -> &mut Self {
        self.add(DB);
        self
    }
    pub fn scope(&mut self, value: &str) -> &mut Self {
        self.add_kv(SCOPE, value);
        self
    }
    pub fn table(&mut self, value: &str) -> &mut Self {
        self.add_kv(TABLE, value);
        self
    }
    fn add(&mut self, field: &str) {
        let stmt = SQLField::new("", field);
        self.available.push_set(&stmt);
    }
    fn add_kv(&mut self, key: &str, value: &str) {
        let stmt = SQLField::new(key, value);
        self.available.push_set(&stmt);
    }
    ///切换下一条语句
    pub fn next(&mut self) -> &mut Self {
        self.current += 1;
        self
    }
    fn build_available(&mut self) -> String {
        let len = self.current;
        let data_list = self.available.get_region_multi();
        if data_list[len].get_keyword().is_empty() {
            String::from(data_list[len].get_field_value())
        } else {
            format!("{}{}{}", data_list[len].get_keyword(), COMMON_SEPARATOR, data_list[len].get_field_value())
        }
    }
}