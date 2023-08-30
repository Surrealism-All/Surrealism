//! # Remove Wrapper
//! ```code
//! REMOVE [
//! NAMESPACE @name
//! | DATABASE @name
//! | LOGIN @name ON [ NAMESPACE | DATABASE ]
//! | TOKEN @name ON [ NAMESPACE | DATABASE ]
//! | SCOPE @name
//! | TABLE @name
//! | EVENT @name ON [ TABLE ] @table
//! | FUNCTION fn::@name
//! | FIELD @name ON [ TABLE ] @table
//! | INDEX @name ON [ TABLE ] @table
//! | PARAM $@name
//! ]
//! ```
//! ```txt
//! @author:syf20020816@Outlook.com
//! @date:2023/8/30
//! @version:0.0.1
//! @description:
//! ```

use crate::core::builder::define::OnType;
use crate::core::db::constants::{REMOVE_INDEX, REMOVE_PARAM, ON, ON_TABLE, REMOVE_DB, REMOVE_EVENT, REMOVE_FIELD, REMOVE_FN, REMOVE_LOGIN, REMOVE_NS, REMOVE_SCOPE, REMOVE_TABLE, REMOVE_TOKEN, STMT_END};

/// # RemoveWrapper
/// 考虑是否可以将其和DefineWrapper产生关联进行合并？
/// 是否有必要？
/// ## example
/// ```rust
/// use surrealism::{SurrealismRes};
/// use surrealism::builder::*;
/// use surrealism::builder::define::{OnType};
///
/// // [tests\src\main.rs:18] remove1 = "REMOVE NAMESPACE surrealdb;"
/// // [tests\src\main.rs:19] remove2 = "REMOVE DATABASE blog;"
/// // [tests\src\main.rs:20] remove3 = "REMOVE LOGIN writer ON DATABASE;"
/// // [tests\src\main.rs:21] remove4 = "REMOVE TOKEN writer_token ON NAMESPACE;"
/// // [tests\src\main.rs:22] remove5 = "REMOVE SCOPE documentation;"
/// // [tests\src\main.rs:23] remove6 = "REMOVE TABLE article;"
/// // [tests\src\main.rs:24] remove7 = "REMOVE EVENT new_post ON TABLE article;"
/// // [tests\src\main.rs:25] remove8 = "REMOVE FUNCTION fn::update_author"
/// // [tests\src\main.rs:26] remove9 = "REMOVE FIELD tags ON TABLE article;"
/// // [tests\src\main.rs:27] remove10 = "REMOVE INDEX authors ON TABLE article;"
/// // [tests\src\main.rs:28] remove11 = "REMOVE PARAM $author;"
/// #[tokio::main]
/// async fn main() -> SurrealismRes<()> {
///     let remove1 = SQLBuilderFactory::remove().ns("surrealdb").build();
///     let remove2 = SQLBuilderFactory::remove().db("blog").build();
///     let remove3 = SQLBuilderFactory::remove().login("writer", OnType::NS).build();
///     let remove4 = SQLBuilderFactory::remove().token("writer_token", OnType::NS).build();
///     let remove5 = SQLBuilderFactory::remove().scope("documentation").build();
///     let remove6 = SQLBuilderFactory::remove().table("article").build();
///     let remove7 = SQLBuilderFactory::remove().event("new_post", "article").build();
///     let remove8 = SQLBuilderFactory::remove().function("update_author").build();
///     let remove9 = SQLBuilderFactory::remove().field("tags", "article").build();
///     let remove10 = SQLBuilderFactory::remove().index("authors", "article").build();
///     let remove11 = SQLBuilderFactory::remove().param("author").build();
///     Ok(())
/// }
/// ```
#[derive(Debug, Clone)]
pub enum RemoveWrapper<'w> {
    NONE,
    NAMESPACE(&'w str),
    DATABASE(&'w str),
    LOGIN {
        name: &'w str,
        on: OnType<'w>,
    },
    TOKEN {
        name: &'w str,
        on: OnType<'w>,
    },
    SCOPE(&'w str),
    TABLE(&'w str),
    EVENT {
        name: &'w str,
        on: &'w str,
    },
    FUNCTION(&'w str),
    FIELD {
        name: &'w str,
        on: &'w str,
    },
    INDEX {
        name: &'w str,
        on: &'w str,
    },
    PARAM(&'w str),
}

impl<'w> RemoveWrapper<'w> {
    pub fn new() -> Self {
        RemoveWrapper::NONE
    }
    /// remove namespace
    pub fn ns(&self, ns: &'w str) -> Self {
        RemoveWrapper::NAMESPACE(ns)
    }
    pub fn db(&self, db: &'w str) -> Self {
        RemoveWrapper::DATABASE(db)
    }
    pub fn login(&self, name: &'w str, on: OnType<'w>) -> Self {
        if on.is_scope() {
            panic!("cannot remove scope");
        }
        RemoveWrapper::LOGIN {
            name,
            on: OnType::DB,
        }
    }
    pub fn token(&self, name: &'w str, on: OnType<'w>) -> Self {
        if on.is_scope() {
            panic!("cannot remove scope");
        }
        RemoveWrapper::TOKEN {
            name,
            on,
        }
    }
    pub fn scope(&self, scope: &'w str) -> Self {
        RemoveWrapper::SCOPE(scope)
    }
    pub fn table(&self, table: &'w str) -> Self {
        RemoveWrapper::TABLE(table)
    }
    pub fn event(&self, event: &'w str, on: &'w str) -> Self {
        RemoveWrapper::EVENT { name: event, on }
    }
    pub fn function(&self, func: &'w str) -> Self {
        RemoveWrapper::FUNCTION(
            func
        )
    }
    pub fn field(&self, field: &'w str, on: &'w str) -> Self {
        RemoveWrapper::FIELD {
            name: field,
            on,
        }
    }
    pub fn index(&self, index: &'w str, on: &'w str) -> Self {
        RemoveWrapper::INDEX {
            name: index,
            on,
        }
    }
    pub fn param(&self, param: &'w str) -> Self {
        RemoveWrapper::PARAM(param)
    }

    pub fn build(&self) -> String {
        match self {
            RemoveWrapper::NONE => panic!("can not remove none from Surrealdb"),
            RemoveWrapper::NAMESPACE(ns) => format!("{} {}{}", REMOVE_NS, ns, STMT_END),
            RemoveWrapper::DATABASE(db) => format!("{} {}{}", REMOVE_DB, db, STMT_END),
            RemoveWrapper::LOGIN { name, on } => format!("{} {} {} {}{}", REMOVE_LOGIN, name, ON, on.to_str(), STMT_END),
            RemoveWrapper::TOKEN { name, on } => format!("{} {} {} {}{}", REMOVE_TOKEN, name, ON, on.to_str(), STMT_END),
            RemoveWrapper::SCOPE(scope) => format!("{} {}{}", REMOVE_SCOPE, scope, STMT_END),
            RemoveWrapper::TABLE(table) => format!("{} {}{}", REMOVE_TABLE, table, STMT_END),
            RemoveWrapper::EVENT { name, on } => format!("{} {} {} {}{}", REMOVE_EVENT, name, ON_TABLE, on, STMT_END),
            RemoveWrapper::FUNCTION(f) => format!("{} fn::{}", REMOVE_FN, f),
            RemoveWrapper::FIELD { name, on } => format!("{} {} {} {}{}", REMOVE_FIELD, name, ON_TABLE, on, STMT_END),
            RemoveWrapper::INDEX { name, on } => format!("{} {} {} {}{}", REMOVE_INDEX, name, ON_TABLE, on, STMT_END),
            RemoveWrapper::PARAM(param) => format!("{} ${}{}", REMOVE_PARAM, param, STMT_END),
        }
    }
}