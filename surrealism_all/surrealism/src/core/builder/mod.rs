//! SQLBuilderFactory for Surrealism
//! ```txt
//! @author:syf20020816@Outlook.com
//! @date:2023/8/4
//! @version:0.0.1
//! @description:
//! ```
mod select;
mod update;
mod insert;
mod delete;
pub mod create;


use self::create::{CreateWrapper, CreateWrapperImpl};
use crate::core::db::{Table, ReturnType, TimeUnit, TimeOut, SurrealID, ContentSet, Object, SurrealValue};

/// SQLBuilderFactory for Surrealism
/// - CreateWrapper
/// - SelectWrapper
/// - UpdateWrapper
/// - InsertWrapper
/// - DeleteWrapper
pub struct SQLBuilderFactory;

impl SQLBuilderFactory {
    pub fn create<'f>() -> CreateWrapper<'f> {
        CreateWrapper::new()
    }
    // fn select() -> SelectWrapper {}
    // fn update() -> UpdateWrapper {}
    // fn insert() -> InsertWrapper {}
    // fn delete() -> DeleteWrapper {}
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

/// wrapper param need content_set:ContentSet(Option<ContentSet>)
pub trait ContentSetImpl<'w> {
    /// add content | set stmt
    fn content_set(&mut self, content_set: ContentSet<'w>) -> &mut Self;
    /// create content_set : ContentSet::Content
    fn content(&mut self, obj: Object) -> &mut Self;
    /// create content_set : ContentSet::Set
    fn set(&mut self) -> &mut Self;
    /// add K-V to ContentSet::Set
    fn add(&mut self, field: &'w str, value: SurrealValue) -> &mut Self;
}

/// wrapper param need return_type:ReturnType(Option<ReturnType>)
pub trait ReturnImpl {
    fn return_type(&mut self, return_type: ReturnType) -> &mut Self;
}
/// wrapper param need timeout:TimeOut(Option<TimeOut>)
pub trait TimeoutImpl {
    fn timeout(&mut self, timeout: TimeOut) -> &mut Self;
}

///PARALLEL
pub trait ParallelImpl {
    fn parallel(&mut self) -> &mut Self;
}