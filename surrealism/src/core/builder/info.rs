//! # wrapper for info
//! ```code
//! INFO FOR [
//! 	KV
//! 	| NS | NAMESPACE
//! 	| DB | DATABASE
//! 	| SCOPE @scope
//! 	| TABLE @table
//! ];
//! ```
//! ```txt
//! @author:syf20020816@Outlook.com
//! @date:2023/8/29
//! @version:0.0.1
//! @description:
//! ```


use crate::core::db::constants::{INFO, KV, DB, SCOPE, TABLE, STMT_END, NS};

#[cfg(feature = "builder")]
#[derive(Debug, Clone)]
pub enum InfoWrapper<'w> {
    KV,
    NS,
    DB,
    SCOPE(&'w str),
    TABLE(&'w str),
}

impl<'w> InfoWrapper<'w> {
    pub fn new() -> Self {
        InfoWrapper::KV
    }
    pub fn kv(&self) -> Self {
        InfoWrapper::KV
    }
    pub fn ns(&self) -> Self {
        InfoWrapper::NS
    }
    pub fn db(&self) -> Self {
        InfoWrapper::DB
    }
    pub fn scope(&self, scope: &'w str) -> Self {
        InfoWrapper::SCOPE(scope)
    }
    pub fn table(&self, table: &'w str) -> Self {
        InfoWrapper::TABLE(table)
    }
    pub fn build(&self) -> String {
        match self {
            InfoWrapper::KV => format!("{} {}{}", INFO, KV, STMT_END),
            InfoWrapper::NS => format!("{} {}{}", INFO, NS, STMT_END),
            InfoWrapper::DB => format!("{} {}{}", INFO, DB, STMT_END),
            InfoWrapper::SCOPE(scope) => format!("{} {} {}{}", INFO, SCOPE, scope, STMT_END),
            InfoWrapper::TABLE(table) => format!("{} {} {}{}", INFO, TABLE, table, STMT_END),
        }
    }
}