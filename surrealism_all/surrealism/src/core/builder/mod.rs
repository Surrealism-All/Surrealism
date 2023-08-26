//! SQLBuilderFactory for Surrealism
//! ```txt
//! @author:syf20020816@Outlook.com
//! @date:2023/8/4
//! @version:0.0.1
//! @description:
//! ```
mod select;
pub mod update;
mod insert;
pub mod delete;
pub mod create;
pub mod relate;
mod macros;


use serde::Serialize;
use crate::Condition;
use self::relate::{RelateWrapper};
use self::delete::{DeleteWrapper};
use self::update::{UpdateWrapper, UpdateWrapperImpl};
use self::create::{CreateWrapper, CreateWrapperImpl};
use crate::core::db::{ReturnType, TimeOut, SurrealID, TimeUnit};

/// SQLBuilderFactory for Surrealism
/// - CreateWrapper
/// - SelectWrapper
/// - UpdateWrapper
/// - InsertWrapper
/// - DeleteWrapper
pub struct SQLBuilderFactory;

impl SQLBuilderFactory {
    pub fn create() -> CreateWrapper {
        CreateWrapper::new()
    }
    pub fn relate() -> RelateWrapper {
        RelateWrapper::new()
    }
    // fn select() -> SelectWrapper {}
    pub fn update<'w>() -> UpdateWrapper<'w> { UpdateWrapper::new() }
    // fn insert() -> InsertWrapper {}
    pub fn delete() -> DeleteWrapper {
        DeleteWrapper::new()
    }
}

/// Base Wrapper trait
/// each wrapper need to implement
pub trait BaseWrapperImpl {
    /// create a new wrapper
    fn new() -> Self;
    /// deref mut wrapper to Self
    fn deref_mut(&mut self) -> Self;
    /// build wrapper to String
    ///
    /// wrapper will build to String(Complete Statement)
    fn build(&mut self) -> String;
}

/// wrapper param need table:Table
pub trait TableImpl {
    /// create table with name
    fn table(&mut self, table: &str) -> &mut Self;
    /// add table id
    /// > like : table_name:table_id
    fn id(&mut self, id: SurrealID) -> &mut Self;
}


pub trait ConditionImpl {
    fn where_condition(&mut self, condition: Condition) -> &mut Self;
}

/// wrapper param need return_type:ReturnType(Option<ReturnType>)
pub trait ReturnImpl {
    fn return_type(&mut self, return_type: ReturnType) -> &mut Self;
}

/// wrapper param need timeout:TimeOut(Option<TimeOut>)
pub trait TimeoutImpl {
    fn timeout_from(&mut self, timeout: TimeOut) -> &mut Self;
    fn timeout(&mut self, timeout: usize, unit: TimeUnit) -> &mut Self;
}

///PARALLEL
pub trait ParallelImpl {
    fn parallel(&mut self) -> &mut Self;
}