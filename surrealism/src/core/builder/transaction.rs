//! 用于构建事务
//! 1. BEGIN [ TRANSACTION ]; 开始
//! 2. CANCEL [ TRANSACTION ]; 取消
//! 3. COMMIT [ TRANSACTION ]; 提交
//! ```txt
//! @author:syf20020816@Outlook.com
//! @date:2023/8/29
//! @version:0.0.1
//! @description:
//! ```

use crate::core::db::constants::{BEGIN_TRANSACTION, CANCEL_TRANSACTION, COMMIT_TRANSACTION};

/// 用于构建事务
/// 1. BEGIN [ TRANSACTION ]; 开始
/// 2. CANCEL [ TRANSACTION ]; 取消
/// 3. COMMIT [ TRANSACTION ]; 提交
#[derive(Debug, Clone)]
pub struct Transaction<'t> {
    stmts: Vec<&'t str>,
}

impl<'t> Transaction<'t> {
    pub fn new() -> Self {
        Transaction {
            stmts: vec![]
        }
    }
    pub fn add(&mut self, stmt: &'t str) -> &mut Self {
        self.stmts.push(stmt);
        self
    }
    /// 取消事务
    pub fn cancel(&self) -> String {
        format!("{} {} {}", BEGIN_TRANSACTION, &self.stmts.join(" "), CANCEL_TRANSACTION)
    }
    /// 提交事务
    pub fn commit(&self) -> String {
        format!("{} {} {}", BEGIN_TRANSACTION, &self.stmts.join(" "), COMMIT_TRANSACTION)
    }
}