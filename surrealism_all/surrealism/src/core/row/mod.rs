//! # Row Sql
//! ## example
//! ```rust
//! use surrealism::{DefaultInitService, InitService, SurrealismConnector, SurrealismRes, SurrealismCommit, UseNSDB,RowSql};
//!
//! #[tokio::main]
//! async fn main() -> SurrealismRes<()> {
//!     let mut service = DefaultInitService::new().init();
//!     let res = service.use_commit("test", "test").await;
//!     dbg!(res);
//!     let commit1 = service.commit_sql("select * from test;").await;
//!     dbg!(commit1);
//!
//!     let sql1 = RowSql::new("SELECT {} FROM {};").bind("*").bind("test").build();
//!
//!     let sql2 = RowSql::new("SELECT {} FROM {} WHERE {} = 56;")
//!         .bind_index(0,"*")
//!         .bind_index(2,"age")
//!         .bind_index(1,"test")
//!         .build();
//!
//!     dbg!(sql1);
//!     dbg!(sql2);
//!     Ok(())
//! }
//! ```
//! ```txt
//! @author:syf20020816@Outlook.com
//! @date:2023/8/4
//! @version:0.0.1
//! @description:
//! ```

use regex::Regex;

pub struct RowSql<'a> {
    stmt: String,
    params: Vec<(&'a str, &'a str)>,
    pointer: u8,
}

impl<'a> RowSql<'a> {
    /// init RowSql with sql statement
    /// # example
    /// ```code
    /// let sql1 = RowSql::new("SELECT {} FROM {};");
    /// ```
    pub fn new(stmt: &'a str) -> RowSql {
        let stmt = String::from(stmt);
        let mut params = vec![];
        for i in 0..matches_count(&stmt) {
            params.push(("{}", ""));
        }
        RowSql {
            stmt,
            params,
            pointer: 0_u8,
        }
    }
    /// default bind `{}` to needed value
    /// # example
    /// ```code
    /// // "SELECT * FROM test;"
    /// let row1 = RowSql::new("SELECT {} FROM {};").bind("*").bind("test").build();
    /// ```
    pub fn bind(&mut self, value: &'a str) -> &mut Self {
        self.params[self.pointer as usize] = ("{}", value);
        self.pointer += 1;
        self
    }
    /// bind with index
    /// # example
    /// ``` code
    /// //"SELECT * FROM test WHERE age = 56;"
    /// let sql2 = RowSql::new("SELECT {} FROM {} WHERE {} = 56;")
    ///         .bind_index(0,"*")
    ///         .bind_index(2,"age")
    ///         .bind_index(1,"test")
    ///         .build();
    /// ```
    pub fn bind_index(&mut self, index: u8, value: &'a str) -> &mut Self {
        self.params[index as usize] = ("{}", value);
        self.pointer = index;
        self
    }
    /// build complete statement
    pub fn build(&mut self) -> Result<String, &'static str> {
        //build complete statement
        let mut res = String::from(&self.stmt);
        for (k, v) in &self.params {
            if res.contains(k) {
                res = res.replacen(k, v, 1);
            } else {
                return Err("Invalid key");
            }
        }
        Ok(res)
    }
}

/// count how many k in target
/// ```
/// let stmt = "SELECT {} FROM {};";
/// //num = 2
/// let num = matches_count(stmt, "{}");
/// ```
fn matches_count(target: &str) -> usize {
    let reg = Regex::new(r#"\{\}"#).unwrap();
    let count = reg.find_iter(target).count();
    count
}

/// An easy way to use row sql
/// ```rust
/// use surrealism::{row_sql,RowSql};
///    let sql = row_sql!("SELECT {} FROM {} WHERE {} = 56;")
///         .bind_index(0, "*")
///         .bind_index(2, "age")
///         .bind_index(1, "test")
///         .build()
///         .unwrap();
/// ```
#[macro_export]
macro_rules! row_sql {
    ($Stmt:tt) => {
        RowSql::new($Stmt);
    };
}