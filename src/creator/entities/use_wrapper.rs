use super::{Wrapper, DB, NS, USE, AvailData};


///USE语句包装器
/// keywords:关键词
/// available:参数存储器
/// stmt:具体语句
#[derive(Debug, Clone)]
pub struct UseWrapper {
    keyword: String,
    available: Vec<AvailData>,
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
            keyword: String::from(USE),
            available: Vec::new(),
            stmt: String::new(),
            namespace: String::new(),
            database: String::new(),
        }
    }


    fn commit(&mut self) -> &str {
        let ns = self.get_namespace();
        let db = self.get_database();
        let t_node1 =  AvailData::new(0, String::from(NS), String::from(ns), false, false);
        let t_node2 =  AvailData::new(1, String::from(DB), String::from(db), false, false);
        self.available.push(t_node1);
        self.available.push(t_node2);
        &self.stmt
    }
    fn get_keyword(&self) -> &str {
        &self.keyword
    }
    fn get_available(&self) -> &Vec<AvailData> {
        &self.available
    }
}
