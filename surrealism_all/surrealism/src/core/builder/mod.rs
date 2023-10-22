//! SQLBuilderFactory for Surrealism
//! ```txt
//! @author:syf20020816@Outlook.com
//! @date:2023/8/4
//! @version:0.0.1
//! @description:
//! ```
pub mod s_use;
pub mod select;
pub mod update;
pub mod insert;
pub mod delete;
pub mod create;
pub mod relate;
pub mod info;
pub mod transaction;
pub mod define;
pub mod remove;
pub mod show;
mod macros;


use self::insert::{InsertWrapper};
use self::transaction::Transaction;
use self::relate::{RelateWrapper};
use self::delete::{DeleteWrapper};
use self::select::{SelectWrapper};
use self::info::InfoWrapper;
use self::remove::RemoveWrapper;
use self::define::*;
use self::s_use::UseWrapper;
use self::update::{UpdateWrapper};
use self::create::{CreateWrapper};
use self::show::ShowWrapper;
use crate::db::{ReturnType, TimeOut, SurrealID, TimeUnit, Condition, SurrealValue};

/// SQLBuilderFactory for Surrealism
/// - CreateWrapper
/// - SelectWrapper
/// - UpdateWrapper
/// - InsertWrapper
/// - DeleteWrapper
pub struct SQLBuilderFactory;

impl SQLBuilderFactory {
    pub fn use_stmt<'w>() -> UseWrapper<'w> {
        UseWrapper::new()
    }
    pub fn create() -> CreateWrapper {
        CreateWrapper::new()
    }
    pub fn relate() -> RelateWrapper {
        RelateWrapper::new()
    }
    pub fn select<'w>() -> SelectWrapper<'w> {
        SelectWrapper::new()
    }
    pub fn update<'w>() -> UpdateWrapper<'w> { UpdateWrapper::new() }
    pub fn insert() -> InsertWrapper {
        InsertWrapper::new()
    }
    pub fn delete() -> DeleteWrapper {
        DeleteWrapper::new()
    }
    pub fn info<'w>() -> InfoWrapper<'w> { InfoWrapper::new() }
    pub fn transaction<'w>() -> Transaction<'w> {
        Transaction::new()
    }
    pub fn define<'w>() -> DefineWrapper<'w> {
        DefineWrapper::new()
    }
    pub fn remove<'w>() -> RemoveWrapper<'w> {
        RemoveWrapper::new()
    }
    pub fn show()->ShowWrapper{ShowWrapper::new()}
    /// # make sleep statement
    /// ## example
    /// ```rust
    /// use std::ops::DerefMut;
    /// use surrealism::builder::{SQLBuilderFactory};
    /// use surrealism::db::{AdapterToValue, SurrealValue};
    /// use surrealism::DefaultRes;
    ///
    /// // [tests\src\main.rs:10] sleep = "SLEEP 2d"
    /// #[tokio::main]
    /// async fn main() -> DefaultRes<()> {
    ///     let sleep = SQLBuilderFactory::sleep(SurrealValue::duration().from_days(2).to_value());
    ///     dbg!(sleep);
    ///     Ok(())
    /// }
    /// ```
    pub fn sleep(duration:SurrealValue)->String{
        return if duration.is_duration(){
           format!("SLEEP {}",duration.to_string())
        }else{
            panic!("SLEEP should use Duration")
        }
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
    fn build_as_child(&mut self)->String;
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