//! # select wrapper
//! ```code
//! SELECT [ VALUE ] @fields [ AS @alias ]
//! 	[ OMIT @fields ...]
//! 	FROM [ ONLY ] @targets
//! 	[ WITH [ NOINDEX | INDEX @indexes ... ]]
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
//! 	[ EXPLAIN [ FULL ]]
//! ;
//! ```
//! ```txt
//! @author:syf20020816@Outlook.com
//! @date:2023/8/27
//! @version:0.0.1
//! @description:
//! ```

use std::fmt::{Display, Formatter};
use crate::{timeout_lifetime_impl, parallel_lifetime_impl, table_lifetime_impl};
use crate::core::db::{Condition, Table, TimeOut, Order, Field, SurrealID, TimeUnit, ParamCombine};
use super::{TimeoutImpl, ParallelImpl, TableImpl, ConditionImpl, BaseWrapperImpl};
use crate::core::db::constants::{EXPLAIN, FULL, SELECT, STMT_END, BLANK, PARALLEL, GROUP_BY, ORDER_BY, SPLIT, START, LIMIT, FETCH, FROM, LIVE_SELECT};
use crate::db::With;

pub trait SelectWrapperImpl<'w>: TableImpl + ParallelImpl + ConditionImpl + TimeoutImpl + BaseWrapperImpl {
    fn column(&mut self, column: &'w str, as_field: Option<&'w str>) -> &mut Self;
    fn omit(&mut self, columns: Vec<&'w str>) -> &mut Self;
    fn with(&mut self, with_index: bool) -> &mut Self;
    fn with_index(&mut self, index: &'w str) -> &mut Self;
    fn explain(&mut self, full: bool) -> &mut Self;
    fn split_at(&mut self, column: &'w str) -> &mut Self;
    fn group_by(&mut self, group: Vec<&'w str>) -> &mut Self;
    fn order_by(&mut self, order: Order<'w>) -> &mut Self;
    fn start(&mut self, start: usize) -> &mut Self;
    fn limit(&mut self, limit: usize) -> &mut Self;
    fn fetch(&mut self, fetch: Vec<&'w str>) -> &mut Self;
    fn to_live(&mut self) -> LiveSelectWrapper<'w>;
}


/// # SelectWrapper
/// ## example
/// ```rust
/// use std::ops::DerefMut;
/// use surrealism::DefaultRes;
/// use surrealism::db::{Condition, ConditionSign, Criteria, CriteriaSign, Field, TimeUnit};
/// use surrealism::builder::{BaseWrapperImpl, ConditionImpl, SQLBuilderFactory, TableImpl, TimeoutImpl};
/// use surrealism::builder::select::SelectWrapperImpl;
///
/// // [tests\src\main.rs:54] select1 = "SELECT name AS username, address FROM person:tobie;"
/// // [tests\src\main.rs:55] select2 = "SELECT * FROM person password ,opts.security;"
/// // [tests\src\main.rs:56] select3 = "SELECT name FROM SurrealDB:great WHERE name != 'Mat' TIMEOUT 5m;"
/// // [tests\src\main.rs:57] select4 = "SELECT * FROM person WHERE email = 'tobie@surrealdb.com' EXPLAIN;"
/// // [tests\src\main.rs:58] select5 = "SELECT name FROM person WITH INDEX idx_name WHERE job = 'engineer' AND genre = 'm';"
/// #[tokio::main]
/// async fn main() -> DefaultRes<()> {
///     let select1 = SQLBuilderFactory::select()
///         .table("person")
///         .id("tobie".into())
///         .column("name", Some("username"))
///         .column("address", None)
///         .build();
///     let select2 = SQLBuilderFactory::select()
///         .table("person")
///         .column("*", None)
///         .omit(vec!["password", "opts.security"])
///         .build();
///     let select3 = SQLBuilderFactory::select()
///         .column("name", None)
///         .table("SurrealDB")
///         .id("great".into())
///         .where_condition(Condition::new().push(Criteria::new("name", "Mat", CriteriaSign::Neq), ConditionSign::None).deref_mut())
///         .timeout(5, TimeUnit::MINUTE)
///         .build();
///     let select4 = SQLBuilderFactory::select()
///         .table("person")
///         .column("*", None)
///         .where_condition(
///             Condition::new().push(
///                 Criteria::new("email", "tobie@surrealdb.com", CriteriaSign::Eq)
///                 , ConditionSign::None,
///             ).deref_mut()
///         )
///         .explain(false)
///         .build();
///     let select5 = SQLBuilderFactory::select()
///         .table("person")
///         .column("name", None)
///         .with(true)
///         .with_index("idx_name")
///         .where_condition(
///             Condition::new().push(
///                 Criteria::new("job", "engineer", CriteriaSign::Eq), ConditionSign::And,
///             ).push(
///                 Criteria::new("genre", "m", CriteriaSign::Eq), ConditionSign::None,
///             ).deref_mut()
///         )
///         .build();
///     Ok(())
/// }
/// ```
#[derive(Debug, Clone)]
pub struct SelectWrapper<'w> {
    field: Vec<Field<'w>>,
    omit: Option<Vec<&'w str>>,
    with: Option<With<'w>>,
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
    explain: Option<bool>,
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
            field: Vec::new(),
            omit: None,
            with: None,
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
            explain: None,
        }
    }

    fn deref_mut(&mut self) -> Self {
        SelectWrapper {
            field: self.field.clone(),
            omit: None,
            with: None,
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
            explain: None,
        }
    }

    fn build(&mut self) -> String {
        format!("{};", self.build_as_child())
    }
    fn build_as_child(&mut self) -> String {
        self.to_string()
    }
}

impl<'w> Display for SelectWrapper<'w> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut res = format!("{} {} {} {}", SELECT, &self.field.iter().map(|x| x.to_string()).collect::<Vec<String>>().join(", "), FROM, &self.table.combine());
        if self.omit.is_some() {
            res.push_str(BLANK);

            res.push_str(&self.omit.as_ref().unwrap().iter().map(|x| x.to_string()).collect::<Vec<String>>().join(" ,"));
        }
        if self.with.is_some() {
            res.push_str(BLANK);
            res.push_str(&self.with.as_ref().unwrap().to_string())
        }
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
        if let Some(full) = self.explain.as_ref() {
            res.push_str(BLANK);
            res.push_str(EXPLAIN);
            if *full {
                res.push_str(BLANK);
                res.push_str(FULL);
            }
        }
        write!(f, "{}", res)
    }
}

impl<'w> SelectWrapperImpl<'w> for SelectWrapper<'w> {
    /// build column : column_name AS as_name
    ///
    /// such as : name AS username
    fn column(&mut self, column: &'w str, as_field: Option<&'w str>) -> &mut Self {
        match as_field {
            None => self.field.push(Field::new_field(column)),
            Some(field) => self.field.push(Field::new_field(column).as_field(field).to_owned())
        };
        self
    }

    fn omit(&mut self, columns: Vec<&'w str>) -> &mut Self {
        self.omit.replace(columns);
        self
    }

    fn with(&mut self, with_index: bool) -> &mut Self {
        match with_index {
            true => self.with.replace(With::INDEX(Vec::new())),
            false => self.with.replace(With::NOINDEX)
        };
        self
    }

    fn with_index(&mut self, index: &'w str) -> &mut Self {
        match self.with {
            None => {
                self.with.replace(With::INDEX(Vec::new()));
                self.with_index(index)
            }
            Some(ref mut indexs) => {
                let _ = indexs.push(index);
                self
            }
        }
    }

    fn explain(&mut self, full: bool) -> &mut Self {
        self.explain.replace(full);
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

    fn to_live(&mut self) -> LiveSelectWrapper<'w> {
        LiveSelectWrapper {
            field: self.field.to_owned(),
            table: self.table.to_owned(),
            condition: self.condition.to_owned(),
            fetch: self.fetch.to_owned(),
        }
    }
}

timeout_lifetime_impl!(SelectWrapper);
parallel_lifetime_impl!(SelectWrapper);
table_lifetime_impl!(SelectWrapper);

/// # LiveSelectWrapper
/// The LIVE SELECT statement can be used to initiate a real-time selection from a table, including the option to apply filters.
///
/// In practical terms, when you execute a LIVE SELECT query, it triggers an ongoing session that captures any subsequent changes to the data in real-time.
///
/// These changes are then immediately transmitted to the client, ensuring that the client is consistently updated with the latest data modifications.
/// ```code
/// LIVE SELECT
/// 	[
/// 		[ VALUE ] @fields [ AS @alias ]
/// 		| DIFF
/// 	]
/// 	FROM @targets
/// 	[ WHERE @conditions ]
/// 	[ FETCH @fields ... ]
/// ;
/// ```
/// ## example
/// ```rust
/// use std::ops::DerefMut;
/// use surrealism::DefaultRes;
/// use surrealism::db::{Condition, ConditionSign, Criteria, CriteriaSign, Field, TimeUnit};
/// use surrealism::builder::{BaseWrapperImpl, ConditionImpl, SQLBuilderFactory, TableImpl, TimeoutImpl};
/// use surrealism::builder::select::SelectWrapperImpl;
///
/// // [tests\src\main.rs:19] live_select1 = "LIVE SELECT * FROM person;"
/// // [tests\src\main.rs:25] live_select2 = "LIVE SELECT DIFF FROM person;"
/// // [tests\src\main.rs:34] live_select3 = "LIVE SELECT * FROM person WHERE age > 18;"
/// #[tokio::main]
/// async fn main() -> DefaultRes<()> {
///         let live_select1 = SQLBuilderFactory::select()
///             .column("*",None)
///             .table("person")
///             .to_live()
///             .build();
///         dbg!(live_select1);
///         let live_select2 = SQLBuilderFactory::select()
///             .column("DIFF",None)
///             .table("person")
///             .to_live()
///             .build();
///         dbg!(live_select2);
///         let live_select3 = SQLBuilderFactory::select()
///             .column("*",None)
///             .table("person")
///             .where_condition(
///                 Condition::new().push(Criteria::new("age",18,CriteriaSign::Gt),ConditionSign::None).deref_mut()
///             )
///             .to_live()
///             .build();
///         dbg!(live_select3);
///         Ok(())
/// }
/// ```
#[derive(Debug, Clone)]
pub struct LiveSelectWrapper<'w> {
    field: Vec<Field<'w>>,
    table: Table,
    condition: Option<Condition>,
    fetch: Option<Vec<&'w str>>,
}

impl<'w> BaseWrapperImpl for LiveSelectWrapper<'w> {
    fn new() -> Self {
        LiveSelectWrapper {
            field: Vec::new(),
            table: Table::default(),
            condition: None,
            fetch: None,
        }
    }

    fn deref_mut(&mut self) -> Self {
        LiveSelectWrapper {
            field: self.field.clone(),
            table: self.table.clone(),
            condition: self.condition.clone(),
            fetch: self.fetch.clone(),
        }
    }

    fn build(&mut self) -> String {
        format!("{}{}", self.build_as_child(), STMT_END)
    }

    fn build_as_child(&mut self) -> String {
        let mut res = format!("{} {} {} {}", LIVE_SELECT, &self.field.iter().map(|x| x.to_string()).collect::<Vec<String>>().join(", "), FROM, &self.table.combine());
        if self.condition.is_some() {
            res.push_str(BLANK);
            res.push_str(&self.condition.as_ref().unwrap().combine())
        }
        if self.fetch.is_some() {
            res.push_str(format!(" {} ", FETCH).as_str());
            res.push_str(&self.fetch.as_ref().unwrap().join(" , "))
        }
        res
    }
}