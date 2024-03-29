//! # UpdateWrapper
//!
//! ## example
//! ```code
//! UPDATE @targets
//! 	[ CONTENT @value
//! 	  | MERGE @value
//! 	  | PATCH @value
//! 	  | SET @field = @value ...
//! 	]
//! 	[ WHERE @condition ]
//! 	[ RETURN [ NONE | BEFORE | AFTER | DIFF | @projections ... ]
//! 	[ TIMEOUT @duration ]
//! 	[ PARALLEL ]
//! ;
//! ```
//! ```txt
//! @author:syf20020816@Outlook.com
//! @date:2023/8/23
//! @version:0.0.1
//! @description:
//! ```

use std::fmt::{Display, Formatter};
use serde::Serialize;
use crate::{parallel_lifetime_impl, table_lifetime_impl, return_lifetime_impl, timeout_lifetime_impl};
use crate::core::db::{Condition, ReturnType, TimeOut, Patch, Object, SurrealValue, UpdateStrategy, Table, ParamCombine, Operator, Set, SurrealID, TimeUnit};
use super::{BaseWrapperImpl, ReturnImpl, ParallelImpl, TimeoutImpl, ConditionImpl, TableImpl};
use crate::core::db::constants::{UPDATE, BLANK, STMT_END, PARALLEL};

pub trait UpdateWrapperImpl<'w>: BaseWrapperImpl + ReturnImpl + ParallelImpl + TimeoutImpl + ConditionImpl + TableImpl {
    fn content<T>(&mut self, obj: &T) -> &mut Self where T: Serialize;
    fn content_obj(&mut self, obj: Object) -> &mut Self;
    fn set(&mut self) -> &mut Self;
    fn add<T>(&mut self, field: &str, value: T, sign: Operator) -> &mut Self where T: Serialize;
    fn add_from_value(&mut self, field: &str, value: SurrealValue, sign: Operator) -> &mut Self;
    /// use merge to update
    fn merge<T>(&mut self, merge: &T) -> &mut Self where T: Serialize;
    /// update by json patch
    fn patch(&mut self, patch: Patch<'w>) -> &mut Self;
}

/// # Update Wrapper
/// the wrapper for UPDATE
/// ## example
/// ```rust
/// use surrealism::db::{ SurrealID, TimeOut, SurrealValue, TimeUnit, ReturnType, Object,  Operator, Condition, Criteria, CriteriaSign, ConditionSign, Patch};
/// use surrealism::builder::*;
/// use serde::{Serialize, Deserialize};
/// use surrealism::builder::update::UpdateWrapperImpl;
/// use surrealism::DefaultRes;
///
/// #[derive(Debug, Serialize, Deserialize)]
/// struct Person<'a> {
///     name: &'a str,
///     company: &'a str,
///     skills: Vec<&'a str>,
/// }
///
/// // [tests\src\main.rs:30] update1.build() = "UPDATE person:100 SET name = 'Tobie' , company = 'SurrealDB' , skills = ['Rust', 'Go', 'JS'];"
/// // [tests\src\main.rs:42] update2.build() = "UPDATE city SET population = 954100 , interests -= 'Java' WHERE name = 'London';"
/// // [tests\src\main.rs:53] update3.build() = "UPDATE person CONTENT { company : 'SurrealDB' , name : 'Tobie' , skills : ['Rust', 'Go', 'JS'] };"
/// // [tests\src\main.rs:60] update4.build() = "UPDATE person:tobie MERGE settings:{ company : 'SurrealDB' , name : 'Tobie' , skills : ['Rust', 'Go', 'JS'] };"
/// // [tests\src\main.rs:67] update5.build() = "UPDATE person:tobie PATCH [ {\"op\":\"add\",\"path\":\"Engineering\",\"value\":\"true\"} ];"
/// // [tests\src\main.rs:79] update6.build() = "UPDATE person SET important = true WHERE -> knows -> person -> (knows WHERE influencer = true) TIMEOUT 5s;"
/// #[tokio::main]
/// async fn main() -> DefaultRes<()> {
///     let mut update1 = SQLBuilderFactory::update()
///         .table("person")
///         .id(SurrealID::Int(100))
///         .set()
///         .add("name", "Tobie", Operator::Eq)
///         .add("company", "SurrealDB", Operator::Eq)
///         .add("skills", vec!["Rust", "Go", "JS"], Operator::Eq)
///         .deref_mut();
///     dbg!(update1.build());
///
///     let mut update2 = SQLBuilderFactory::update()
///         .table("city")
///         .id(SurrealID::Default)
///         .set()
///         .add("population", 954100, Operator::Eq)
///         .add("interests", "Java", Operator::Minus)
///         .where_condition(Condition::new()
///             .push(Criteria::new("name", "London", CriteriaSign::Eq), ConditionSign::None)
///             .deref_mut())
///         .deref_mut();
///     dbg!(update2.build());
///
///     let person = Person {
///         name: "Tobie",
///         company: "SurrealDB",
///         skills: vec!["Rust", "Go", "JS"],
///     };
///     let mut update3 = SQLBuilderFactory::update()
///         .table("person")
///         .content(&person)
///         .deref_mut();
///     dbg!(update3.build());
///
///     let mut update4 = SQLBuilderFactory::update()
///         .table("person")
///         .id("tobie".into())
///         .merge(&person)
///         .deref_mut();
///     dbg!(update4.build());
///
///     let mut update5 = SQLBuilderFactory::update()
///         .table("person")
///         .id(SurrealID::from("tobie"))
///         .patch(Patch::add("Engineering", true))
///         .deref_mut();
///     dbg!(update5.build());
///
///     let mut update6 = SQLBuilderFactory::update()
///         .table("person")
///         .set()
///         .add("important", true, Operator::Eq)
///         .where_condition(Condition::new()
///             .push(Criteria::new("knows", "person", CriteriaSign::Link), ConditionSign::Link)
///             .push(Criteria::cheat("knows", "influencer = true", "WHERE"), ConditionSign::None)
///             .deref_mut())
///         .timeout(5, TimeUnit::SECOND)
///         .deref_mut();
///     dbg!(update6.build());
///     Ok(())
/// }
/// ```
#[derive(Debug, Clone)]
pub struct UpdateWrapper<'w> {
    table: Table,
    strategy: Option<UpdateStrategy<'w>>,
    condition: Option<Condition>,
    return_type: Option<ReturnType>,
    timeout: Option<TimeOut>,
    parallel: bool,
}

impl<'w> BaseWrapperImpl for UpdateWrapper<'w> {
    fn new() -> Self {
        UpdateWrapper {
            table: Table::default(),
            strategy: None,
            condition: None,
            return_type: None,
            timeout: None,
            parallel: false,
        }
    }

    fn deref_mut(&mut self) -> Self {
        UpdateWrapper {
            table: self.table.clone(),
            strategy: self.strategy.clone(),
            return_type: self.return_type.clone(),
            timeout: self.timeout.clone(),
            parallel: self.parallel,
            condition: self.condition.clone(),
        }
    }

    fn build(&mut self) -> String {
        format!("{}{}", self.to_string(), STMT_END)
    }

    fn build_as_child(&mut self) -> String {
        self.to_string()
    }
}

impl<'w> Display for UpdateWrapper<'w> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut res = format!("{} {}", UPDATE, &self.table.combine());
        if self.strategy.is_some() {
            res.push_str(BLANK);
            res.push_str(&self.strategy.as_ref().unwrap().combine());
        }
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

        write!(f, "{}", res)
    }
}


impl<'w> ConditionImpl for UpdateWrapper<'w> {
    fn where_condition(&mut self, condition: Condition) -> &mut Self {
        self.condition = Some(condition);
        self
    }
}

impl<'w> UpdateWrapperImpl<'w> for UpdateWrapper<'w> {
    fn content<T>(&mut self, obj: &T) -> &mut Self where T: Serialize {
        self.content_obj(Object::from_obj(obj))
    }
    fn content_obj(&mut self, obj: Object) -> &mut Self {
        match self.strategy {
            None => self.strategy = Some(UpdateStrategy::from_content(obj)),
            Some(_) => {
                let _ = self.strategy.replace(UpdateStrategy::from_content(obj));
            }
        };
        self
    }
    fn set(&mut self) -> &mut Self {
        match self.strategy {
            None => self.strategy = Some(UpdateStrategy::from(vec![])),
            Some(_) => {
                let _ = self.strategy.replace(UpdateStrategy::from(vec![]));
            }
        };
        self
    }

    fn add<T>(&mut self, field: &str, value: T, sign: Operator) -> &mut Self where T: Serialize {
        self.add_from_value(field, SurrealValue::from(serde_json::to_value(value).unwrap()), sign)
    }

    fn add_from_value(&mut self, field: &str, value: SurrealValue, sign: Operator) -> &mut Self {
        let item = Set::new(field, value, sign);
        match self.strategy {
            None => {
                self.strategy = Some(UpdateStrategy::from(vec![item]));
            }
            Some(ref mut strategy) => {
                strategy.push(item);
            }
        };
        self
    }

    fn merge<T>(&mut self, merge: &T) -> &mut Self where T: Serialize {
        match self.strategy {
            None => self.strategy = Some(UpdateStrategy::from_merge(Object::from_obj(merge))),
            Some(_) => {
                let _ = self.strategy.replace(UpdateStrategy::from_merge(Object::from_obj(merge)));
            }
        }
        self
    }

    fn patch(&mut self, patch: Patch<'w>) -> &mut Self {
        match self.strategy {
            None => self.strategy = Some(UpdateStrategy::from(patch)),
            Some(_) => {
                let _ = self.strategy.replace(UpdateStrategy::from(patch));
            }
        }
        self
    }
}

parallel_lifetime_impl!(UpdateWrapper);
table_lifetime_impl!(UpdateWrapper);
return_lifetime_impl!(UpdateWrapper);
timeout_lifetime_impl!(UpdateWrapper);
