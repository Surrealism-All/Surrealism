//! # RelateWrapper
//! ```code
//! RELATE @from_record -> @table -> @to_record
//! 	[ CONTENT @value
//! 	  | SET @field = @value ...
//! 	]
//! 	[ RETURN [ NONE | BEFORE | AFTER | DIFF | @fields ... ]
//! 	[ TIMEOUT @duration ]
//! 	[ PARALLEL ]
//! ;
//! ```
//! ```txt
//! @author:syf20020816@Outlook.com
//! @date:2023/8/26
//! @version:0.0.1
//! @description:
//! ```

use serde::Serialize;
use super::{BaseWrapperImpl, TableImpl, ReturnImpl, TimeoutImpl, ParallelImpl};
use crate::core::db::constants::{RELATE, BLANK, PARALLEL, STMT_END, LINK,DELETE,WHERE};
use crate::core::db::{ReturnType, Table, TimeOut, SurrealID, ParamCombine, Object, SurrealValue, CreateStrategy};
use crate::{Operator, Set, TimeUnit, timeout_impl, return_impl, parallel_impl};

pub trait RelateWrapperImpl: BaseWrapperImpl + ReturnImpl + TimeoutImpl + ParallelImpl {
    fn table_from(&mut self, table: &str, id: SurrealID) -> &mut Self;
    fn table_with(&mut self, table: &str, id: SurrealID) -> &mut Self;
    fn table_to(&mut self, table: &str, id: SurrealID) -> &mut Self;
    fn content<T>(&mut self, obj: &T) -> &mut Self where T: Serialize;
    fn content_obj(&mut self, obj: Object) -> &mut Self;
    fn set(&mut self) -> &mut Self;
    fn add<T>(&mut self, field: &str, value: T) -> &mut Self where T: Serialize;
    fn add_from_value(&mut self, field: &str, value: SurrealValue) -> &mut Self;
    /// convert relate to delete
    /// ```code
    /// RELATE person:tobie->bought->product:iphone;
    /// ↓ ↓ ↓ ↓ ↓ ↓ ↓ ↓
    /// DELETE person:tobie->bought WHERE out=product:iphone;
    /// ```
    fn delete(&self) -> String;
}

/// # RelateWrapper
/// the wrapper for RELATE
/// ```rust
/// use surrealism::{SurrealismRes,SurrealID};
/// use surrealism::builder::*;
/// use surrealism::builder::relate::RelateWrapperImpl;
///
///
/// #[tokio::main]
/// async fn main() -> SurrealismRes<()> {
///     // RELATE person:l19zjikkw1p1h9o6ixrg->wrote->article:8nkk6uj4yprt49z7y3zm;
///     let mut relate1 = SQLBuilderFactory::relate()
///         .table_from("person", SurrealID::from("l19zjikkw1p1h9o6ixrg"))
///         .table_with("wrote", SurrealID::Default)
///         .table_to("article", SurrealID::from("8nkk6uj4yprt49z7y3zm"))
///         .deref_mut();
///     dbg!(relate1.build());
///     Ok(())
/// }
/// ```
#[derive(Debug, Clone)]
pub struct RelateWrapper {
    table_from: Table,
    table_with: Table,
    table_to: Table,
    content: Option<CreateStrategy>,
    return_type: Option<ReturnType>,
    timeout: Option<TimeOut>,
    parallel: bool,
}

impl BaseWrapperImpl for RelateWrapper {
    fn new() -> Self {
        RelateWrapper {
            table_from: Default::default(),
            table_with: Default::default(),
            table_to: Default::default(),
            content: None,
            return_type: None,
            timeout: None,
            parallel: false,
        }
    }

    fn deref_mut(&mut self) -> Self {
        RelateWrapper {
            table_from: self.table_from.clone(),
            table_with: self.table_with.clone(),
            table_to: self.table_to.clone(),
            content: self.content.clone(),
            return_type: self.return_type.clone(),
            timeout: self.timeout.clone(),
            parallel: self.parallel,

        }
    }

    fn build(&mut self) -> String {
        let mut res = format!("{} {}{}{}{}{}", RELATE, &self.table_from.combine(), LINK, &self.table_with.combine(), LINK, &self.table_to.combine());
        if self.content.is_some() {
            res.push_str(BLANK);
            res.push_str(&self.content.as_ref().unwrap().combine());
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
        res.push_str(STMT_END);
        res
    }
}


impl RelateWrapperImpl for RelateWrapper {
    fn table_from(&mut self, table: &str, id: SurrealID) -> &mut Self {
        self.table_from.table(table).id(id);
        self
    }

    fn table_with(&mut self, table: &str, id: SurrealID) -> &mut Self {
        self.table_with.table(table).id(id);
        self
    }

    fn table_to(&mut self, table: &str, id: SurrealID) -> &mut Self {
        self.table_to.table(table).id(id);
        self
    }

    fn content<T>(&mut self, obj: &T) -> &mut Self where T: Serialize {
        self.content_obj(Object::from_obj(obj))
    }
    fn content_obj(&mut self, obj: Object) -> &mut Self {
        match self.content {
            None => self.content = Some(CreateStrategy::from(obj)),
            Some(_) => {
                let _ = self.content.replace(CreateStrategy::from(obj));
            }
        };
        self
    }
    fn set(&mut self) -> &mut Self {
        match self.content {
            None => self.content = Some(CreateStrategy::from(vec![])),
            Some(_) => {
                let _ = self.content.replace(CreateStrategy::from(vec![]));
            }
        };
        self
    }
    fn add<T>(&mut self, field: &str, value: T) -> &mut Self where T: Serialize {
        self.add_from_value(field, SurrealValue::from(serde_json::to_value(value).unwrap()))
    }
    fn add_from_value(&mut self, field: &str, value: SurrealValue) -> &mut Self {
        let item = Set::new(field, value, Operator::Eq);
        match self.content {
            None => {
                self.content = Some(CreateStrategy::Set(vec![item]));
            }
            Some(ref mut strategy) => {
                strategy.push(item);
            }
        };
        self
    }
    fn delete(&self) -> String {
        format!("{} {}{}{} {} out={}",DELETE,&self.table_from.combine(),LINK,&self.table_with.combine(),WHERE,&self.table_to.combine())
    }
}

timeout_impl!(RelateWrapper);
return_impl!(RelateWrapper);
parallel_impl!(RelateWrapper);