//! 用于构建事务
//! 1. BEGIN [ TRANSACTION ]; 开始
//! 2. CANCEL [ TRANSACTION ]; 取消
//! 3. COMMIT [ TRANSACTION ]; 提交

use crate::creator::entities::{SQLRegion, Statements};
use super::{Wrapper};

const BEGIN: &str = "BEGIN TRANSACTION;";
const CANCEL: &str = "CANCEL TRANSACTION;";
const COMMIT: &str = "COMMIT TRANSACTION;";

/// TRANSACTION事务
/// >事务无需调用者进行调取,但是需要指定事务最后是取消的还是提交的
pub struct Transaction {
    /// Wrapper list 存储容器
    list: Vec<String>,
    stmt: String,
}


impl Transaction {
    pub fn new() -> Self {
        Transaction {
            list: vec![],
            stmt: String::new(),
        }
    }

    /// 添加一条语句
    /// ```rust
    /// use surrealism::{Transaction, SelectWrapper, Wrapper};
    /// let mut wrapper1 = SelectWrapper::new();
    /// wrapper1.select("SELECT * FROM user;");
    /// let mut transaction = Transaction::new();
    /// transaction.add(&mut wrapper1);
    /// ```
    pub fn add(&mut self, item: &mut impl Wrapper) -> &mut Self {
        let stmt = item.commit();
        self.list.push(String::from(stmt));
        self
    }
    /// 添加一条语句
    pub fn add_stmt(&mut self, stmt: &str) -> &mut Self {
        self.list.push(String::from(stmt));
        self
    }
    ///添加一组语句
    pub fn add_stmts(&mut self, stmts: Vec<&str>) -> &mut Self {
        for stmt in stmts {
            self.list.push(String::from(stmt));
        }
        self
    }
    /// 根据索引获取一条语句
    pub fn get(&self, index: usize) -> &str {
        &self.list[index]
    }
    /// 取消一组事务
    pub fn cancel(&mut self) {
        self.stmt.push_str(BEGIN);
        for stmt in self.list.iter() {
            self.stmt.push_str(stmt);
        }
        self.stmt.push_str(CANCEL);
    }
    /// 提交一组事务
    pub fn commit(&mut self) {
        self.stmt.push_str(BEGIN);
        for stmt in self.list.iter() {
            self.stmt.push_str(stmt);
        }
        self.stmt.push_str(COMMIT);
    }
    pub fn get_stmt(&self) -> &str {
        &self.stmt
    }
}


