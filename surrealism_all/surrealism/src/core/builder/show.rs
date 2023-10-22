use std::fmt::{Display, Formatter};
use crate::builder::{BaseWrapperImpl, TableImpl};
use crate::db::{SurrealValue, Table,SurrealID};
use crate::db::constants::{SHOW, LIMIT, SINCE, TABLE, BLANK};
use crate::table_impl;

/// # ShowWrapper
/// SHOW 语句可用于重播对表所做的更改
/// ## example
/// ```rust
/// use std::ops::DerefMut;
/// use surrealism::builder::{SQLBuilderFactory, TableImpl};
/// use surrealism::db::{AdapterToValue, SurrealValue};
/// use surrealism::DefaultRes;
///
/// // [tests\src\main.rs:14] show1 = "SHOW CHANGES FOR TABLE reading SINCE '2023-10-22T09:33:06.501656600Z' LIMIT 10"
/// #[tokio::main]
/// async fn main() -> DefaultRes<()> {
///     let show1 = SQLBuilderFactory::show()
///         .table("reading")
///         .since(SurrealValue::datetime().default().to_value())
///         .limit(10)
///         .to_string();
///     dbg!(show1);
///     Ok(())
/// }
/// ```
#[derive(Debug, Clone)]
pub struct ShowWrapper {
    table: Table,
    since: Option<SurrealValue>,
    limit: Option<u32>,
}

impl ShowWrapper {
    pub fn since(&mut self, since: SurrealValue) -> &mut Self {
        self.since = Some(since);
        self
    }
    pub fn limit(&mut self, limit: u32) -> &mut Self {
        self.limit = Some(limit);
        self
    }
}

impl BaseWrapperImpl for ShowWrapper {
    fn new() -> Self {
        ShowWrapper::default()
    }

    fn deref_mut(&mut self) -> Self {
        self.clone()
    }

    fn build(&mut self) -> String {
        self.to_string()
    }
}

impl Display for ShowWrapper {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut res = format!("{} {} {}", SHOW, TABLE, self.table.to_string());
        if self.since.is_some() {
            res.push_str(format!("{}{}{}",BLANK,SINCE,BLANK).as_str());
            res.push_str(self.since.as_ref().unwrap().to_string().as_str());
        }
        if self.limit.is_some() {
            res.push_str(format!("{}{}{}",BLANK,LIMIT,BLANK).as_str());
            res.push_str(self.limit.as_ref().unwrap().to_string().as_str());
        }
        write!(f, "{}", res)
    }
}

impl Default for ShowWrapper {
    fn default() -> Self {
        ShowWrapper {
            table: Table::default(),
            since: None,
            limit: None,
        }
    }
}

table_impl!(ShowWrapper);