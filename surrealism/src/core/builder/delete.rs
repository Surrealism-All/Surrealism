//! # DeleteWrapper
//! ```code
//! DELETE @targets
//! [ WHERE @condition ]
//! [ RETURN [ NONE | BEFORE | AFTER | DIFF | @projections ... ]
//! [ TIMEOUT @duration ]
//! [ PARALLEL ]
//! ;
//! ```
//! ```txt
//! @author:syf20020816@Outlook.com
//! @date:2023/8/4
//! @version:0.0.1
//! @description:
//! ```


use super::{BaseWrapperImpl, TableImpl, ReturnImpl, TimeoutImpl, ParallelImpl, ConditionImpl};
use crate::core::db::constants::{DELETE, BLANK, PARALLEL, STMT_END};
use crate::core::db::{ReturnType, Table, TimeOut, SurrealID, ParamCombine, Condition, TimeUnit};
use crate::{parallel_impl, return_impl, timeout_impl, table_impl};

pub trait DeleteWrapperImpl: BaseWrapperImpl + TableImpl + ReturnImpl + TimeoutImpl + ParallelImpl + ConditionImpl {}

/// # DeleteWrapper
/// ## example
/// ```rust
/// use surrealism::builder::s_use::UseWrapperImpl;
/// use surrealism::DefaultRes;
/// use surrealism::builder::{BaseWrapperImpl, ConditionImpl, SQLBuilderFactory, TableImpl};
/// use surrealism::db::{Condition, ConditionSign, Criteria, CriteriaSign, Operator, SurrealID};
/// use surrealism::builder::insert::InsertWrapperImpl;
///
/// // [tests\src\main.rs:23] delete1 = "DELETE person:100;"
/// // [tests\src\main.rs:24] delete2 = "DELETE city:USA WHERE name CONTAINS 'Los Angeles';"
/// #[tokio::main]
/// async fn main() -> DefaultRes<()> {
///     let delete1 = SQLBuilderFactory::delete()
///         .table("person")
///         .id(100.into())
///         .build();
///     let mut delete2 = SQLBuilderFactory::delete()
///         .table("city")
///         .id(SurrealID::from("USA"))
///         .where_condition(
///             Condition::new().push(
///                 Criteria::new("name", "Los Angeles", CriteriaSign::Contains), ConditionSign::None,
///             ).deref_mut()
///         )
///         .build();
///     Ok(())
/// }
/// ```
#[derive(Debug, Clone)]
pub struct DeleteWrapper {
    table: Table,
    condition: Option<Condition>,
    return_type: Option<ReturnType>,
    timeout: Option<TimeOut>,
    parallel: bool,
}

impl BaseWrapperImpl for DeleteWrapper {
    fn new() -> Self {
        DeleteWrapper {
            table: Table::default(),
            condition: None,
            return_type: None,
            timeout: None,
            parallel: false,
        }
    }

    fn deref_mut(&mut self) -> Self {
        DeleteWrapper {
            table: self.table.clone(),
            condition: self.condition.clone(),
            return_type: self.return_type.clone(),
            timeout: self.timeout.clone(),
            parallel: self.parallel,
        }
    }

    fn build(&mut self) -> String {
        format!("{}{}", self.build_as_child(), STMT_END)
    }

    fn build_as_child(&mut self) -> String {
        let mut res = format!("{} {}", DELETE, &self.table.combine());
        if self.condition.is_some() {
            res.push_str(BLANK);
            res.push_str(&self.condition.as_ref().unwrap().combine());
        }
        if self.return_type.is_some() {
            res.push_str(BLANK);
            res.push_str(&self.return_type.as_ref().unwrap().combine());
        }
        if self.timeout.is_some() {
            res.push_str(BLANK);
            res.push_str(&self.timeout.as_ref().unwrap().combine());
        }
        if self.parallel {
            res.push_str(BLANK);
            res.push_str(PARALLEL);
        }
        res
    }
}


impl ConditionImpl for DeleteWrapper {
    fn where_condition(&mut self, condition: Condition) -> &mut Self {
        self.condition = Some(condition);
        self
    }
}

impl DeleteWrapperImpl for DeleteWrapper {}

table_impl!(DeleteWrapper);
timeout_impl!(DeleteWrapper);
parallel_impl!(DeleteWrapper);
return_impl!(DeleteWrapper);