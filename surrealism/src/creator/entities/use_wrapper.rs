use super::{Wrapper, DB, NS, USE, SQLField, SQLRegion, Statements, RegionField};


///USE语句包装器
/// keywords:关键词
/// available:参数存储器
/// stmt:具体语句
#[derive(Debug, Clone)]
pub struct UseWrapper {
    keyword: Statements,
    available: SQLRegion,
    stmt: String,
    namespace: String,
    database: String,
}

impl UseWrapper {
    ///=================================================<br>
    /// @params:
    /// <ol>
    ///     <li>namespace:使用的命名空间</li>
    /// </ol>
    /// @return:<br>
    /// @date:2023/5/28<br>
    /// @description:设置SurrealDB使用命名空间<br>
    ///=================================================
    pub fn use_ns(&mut self, namespace: &str) -> &mut Self {
        self.namespace = String::from(namespace);
        self
    }
    pub fn use_db(&mut self, database: &str) -> &mut Self {
        self.database = String::from(database);
        self
    }
    fn get_namespace(&self) -> &str {
        &self.namespace
    }
    fn get_database(&self) -> &str {
        &self.database
    }
}

impl Wrapper for UseWrapper {
    fn new() -> Self {
        UseWrapper {
            keyword: Statements::USE,
            available: SQLRegion::new(RegionField::Multi(Vec::new()), USE),
            stmt: String::new(),
            namespace: String::new(),
            database: String::new(),
        }
    }


    fn commit(&mut self) -> &str {
        // let ns =;
        // let db = ;
        let ns_field = SQLField::new(NS,  self.get_namespace());
        let db_field = SQLField::new(DB, self.get_database());
        self.available.push_set(&ns_field);
        self.available.push_set(&db_field);
        &self.stmt
    }

    fn get_keyword(&self) -> &Statements {
        &self.keyword
    }


    fn get_available(&self) -> &SQLRegion {
        &self.available
    }
}
