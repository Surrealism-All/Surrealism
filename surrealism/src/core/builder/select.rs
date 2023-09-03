//! # select wrapper
//! ```code
//! SELECT [ VALUE ] @fields [ AS @alias ]
//! 	FROM @targets
//! 	[ WHERE @conditions ]
//! 	[ SPLIT [ AT ] @field ... ]
//! 	[ GROUP [ BY ] @fields ... ]
//! 	[ ORDER [ BY ]
//! 		@fields [
//! 			RAND()
//! 			| COLLATE
//! 			| NUMERIC
//! 		] [ ASC | DESC ] ...
//! 	] ]
//! 	[ LIMIT [ BY ] @limit ]
//! 	[ START [ AT ] @start ]
//! 	[ FETCH @fields ... ]
//! 	[ TIMEOUT @duration ]
//! 	[ PARALLEL ]
//! ;
//! ```
//! ```txt
//! @author:syf20020816@Outlook.com
//! @date:2023/8/27
//! @version:0.0.1
//! @description:
//! ```

use crate::{Condition, Table, TimeOut, Order, Field, SurrealID, TimeUnit, timeout_lifetime_impl, parallel_lifetime_impl, table_lifetime_impl, ParamCombine};
use super::{TimeoutImpl, ParallelImpl, TableImpl, ConditionImpl, BaseWrapperImpl};
use crate::core::db::constants::{SELECT, STMT_END, BLANK, PARALLEL, ALL, GROUP_BY, ORDER_BY, SPLIT, START, LIMIT, FETCH, FROM};

pub trait SelectWrapperImpl<'w>: TableImpl + ParallelImpl + ConditionImpl + TimeoutImpl + BaseWrapperImpl {
    fn column(&mut self, column: &'w str, as_name: Option<&'w str>) -> &mut Self;
    fn split_at(&mut self, column: &'w str) -> &mut Self;
    fn group_by(&mut self, group: Vec<&'w str>) -> &mut Self;
    fn order_by(&mut self, order: Order<'w>) -> &mut Self;
    fn start(&mut self, start: usize) -> &mut Self;
    fn limit(&mut self, limit: usize) -> &mut Self;
    fn fetch(&mut self, fetch: Vec<&'w str>) -> &mut Self;
}

/// # SelectWrapper
/// ## example
/// ```rust
/// use surrealism::{SurrealismRes, SurrealID, SurrealValue, Condition, Criteria, CriteriaSign, ConditionSign, TimeUnit, Order};
/// use surrealism::builder::*;
/// use surrealism::builder::select::SelectWrapperImpl;
/// use surrealism::functions::Function;
///
///
/// // [tests\src\main.rs:22] select1 = "SELECT name FROM SurrealDB:great GROUP BY id;"
/// // [tests\src\main.rs:30] select2 = "SELECT name FROM SurrealDB:great WHERE name != 'Mat' TIMEOUT 5m;"
/// // [tests\src\main.rs:36] select3 = "SELECT * FROM article ORDER BY title , des ASC;"
/// // [tests\src\main.rs:42] select4 = "SELECT * FROM person LIMIT 50;"
/// #[tokio::main]
/// async fn main() -> SurrealismRes<()> {
///     let select1 = SQLBuilderFactory::select()
///         .column("name",None)
///         .table("SurrealDB")
///         .id(SurrealID::from("great"))
///         .group_by(vec!["id"])
///         .build();
///     dbg!(select1);
///     let select2 = SQLBuilderFactory::select()
///         .column("name",None)
///         .table("SurrealDB")
///         .id(SurrealID::from("great"))
///         .where_condition(Condition::new().push(Criteria::new("name", "Mat", CriteriaSign::Neq), ConditionSign::None).deref_mut())
///         .timeout(5, TimeUnit::MINUTE)
///         .build();
///     dbg!(select2);
///     let select3 = SQLBuilderFactory::select()
///         .column("*",None)
///         .table("article")
///         .order_by(Order::new_asc(vec!["title", "des"]))
///         .build();
///     dbg!(select3);
///     let select4 = SQLBuilderFactory::select()
///         .column("*",None)
///         .table("person")
///         .limit(50)
///         .build();
///     dbg!(select4);
///     Ok(())
/// }
/// ```
#[derive(Debug, Clone)]
pub struct SelectWrapper<'w> {
    field: Field<'w>,
    table: Table,
    condition: Option<Condition>,
    split: Option<&'w str>,
    group: Option<Vec<&'w str>>,
    order: Option<Order<'w>>,
    limit: Option<usize>,
    start: Option<usize>,
    fetch: Option<Vec<&'w str>>,
    timeout: Option<TimeOut>,
    parallel: bool,
}

impl<'w> ConditionImpl for SelectWrapper<'w> {
    fn where_condition(&mut self, condition: Condition) -> &mut Self {
        self.condition = Some(condition);
        self
    }
}

impl<'w> BaseWrapperImpl for SelectWrapper<'w> {
    fn new() -> Self {
        SelectWrapper {
            field: Field::default(),
            table: Table::default(),
            condition: None,
            split: None,
            group: None,
            order: None,
            limit: None,
            start: None,
            fetch: None,
            timeout: None,
            parallel: false,
        }
    }

    fn deref_mut(&mut self) -> Self {
        SelectWrapper {
            field: self.field.clone(),
            table: self.table.clone(),
            condition: self.condition.clone(),
            split: self.split.clone(),
            group: self.group.clone(),
            order: self.order.clone(),
            limit: self.limit.clone(),
            start: self.start.clone(),
            fetch: self.fetch.clone(),
            timeout: self.timeout.clone(),
            parallel: self.parallel.clone(),
        }
    }

    fn build(&mut self) -> String {
        let mut res = format!("{} {} {} {}", SELECT, &self.field.combine(), FROM, &self.table.combine());
        if self.condition.is_some() {
            res.push_str(BLANK);
            res.push_str(&self.condition.as_ref().unwrap().combine())
        }
        if self.split.is_some() {
            res.push_str(format!(" {} ", SPLIT).as_str());
            res.push_str(&self.split.as_ref().unwrap())
        }
        if self.group.is_some() {
            res.push_str(format!(" {} ", GROUP_BY).as_str());
            res.push_str(&self.group.as_ref().unwrap().join(" , "))
        }
        if self.order.is_some() {
            res.push_str(format!(" {} ", ORDER_BY).as_str());
            res.push_str(&self.order.as_ref().unwrap().combine())
        }
        if self.limit.is_some() {
            res.push_str(format!(" {} ", LIMIT).as_str());
            res.push_str(&self.limit.as_ref().unwrap().to_string())
        }
        if self.start.is_some() {
            res.push_str(format!(" {} ", START).as_str());
            res.push_str(&self.start.as_ref().unwrap().to_string())
        }
        if self.fetch.is_some() {
            res.push_str(format!(" {} ", FETCH).as_str());
            res.push_str(&self.fetch.as_ref().unwrap().join(" , "))
        }

        if self.timeout.is_some() {
            res.push_str(BLANK);
            res.push_str(&self.timeout.as_ref().unwrap().combine());
        }
        if self.parallel {
            res.push_str(BLANK);
            res.push_str(PARALLEL);
        }
        res.push_str(STMT_END);
        res
    }
}

impl<'w> SelectWrapperImpl<'w> for SelectWrapper<'w> {
    /// build column : column_name AS as_name
    ///
    /// such as : name AS username
    fn column(&mut self, column: &'w str, as_name: Option<&'w str>) -> &mut Self {
        match column {
            ALL => self.field = Field::All,
            other => {
                match self.field {
                    Field::All => {
                        self.field = Field::Fields(vec![]);
                        self.field.push(other, as_name);
                    }
                    Field::Fields(_) => self.field.push(other, as_name)
                }
            }
        }
        self
    }

    fn split_at(&mut self, column: &'w str) -> &mut Self {
        self.split = Some(column);
        self
    }

    fn group_by(&mut self, group: Vec<&'w str>) -> &mut Self {
        self.group = Some(group);
        self
    }

    fn order_by(&mut self, order: Order<'w>) -> &mut Self {
        self.order = Some(order);
        self
    }

    fn start(&mut self, start: usize) -> &mut Self {
        self.start = Some(start);
        self
    }

    fn limit(&mut self, limit: usize) -> &mut Self {
        self.limit = Some(limit);
        self
    }

    fn fetch(&mut self, fetch: Vec<&'w str>) -> &mut Self {
        self.fetch = Some(fetch);
        self
    }
}

timeout_lifetime_impl!(SelectWrapper);
parallel_lifetime_impl!(SelectWrapper);
table_lifetime_impl!(SelectWrapper);