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

/// # INFO Wrapper
/// INFO 命令输出有关SurrealDB系统设置的信息。有许多不同的INFO 用于在数据库的不同级别检索配置的命令
/// ## example
/// ```rust
/// use std::ops::DerefMut;
/// use surrealism::builder::SQLBuilderFactory;
/// use surrealism::DefaultRes;
///
/// // [tests\src\main.rs:14] info1.build() = "INFO FOR DB;"
/// // [tests\src\main.rs:15] info2.build() = "INFO FOR NS;"
/// // [tests\src\main.rs:16] info3.build() = "INFO FOR SCOPE name;"
/// // [tests\src\main.rs:17] info4.build() = "INFO FOR TABLE user;"
/// // [tests\src\main.rs:18] info5.build() = "INFO FOR KV;"
/// #[tokio::main]
/// async fn main() -> DefaultRes<()> {
///     let info1 = SQLBuilderFactory::info().db();
///     let info2= SQLBuilderFactory::info().ns();
///     let info3 = SQLBuilderFactory::info().scope("name");
///     let info4 = SQLBuilderFactory::info().table("user");
///     let info5 = SQLBuilderFactory::info().kv();
///     dbg!(info1.build());
///     dbg!(info2.build());
///     dbg!(info3.build());
///     dbg!(info4.build());
///     dbg!(info5.build());
///     Ok(())
/// }
/// ```
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