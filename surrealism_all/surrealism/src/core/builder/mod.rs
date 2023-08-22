//! SQLBuilder for Surrealism
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
mod create;


use self::create::{CreateWrapper, CreateWrapperImpl};
use crate::core::db::{Table, ReturnType, TimeUnit, TimeOut, SurrealID, ContentSet, Object, SurrealValue};

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


pub trait BaseWrapperImpl {
    fn new() -> Self;
    fn deref_mut(&mut self) -> Self;
    fn build(&mut self) -> String;
}

pub trait TableImpl {
    fn table(&mut self, table: &str) -> &mut Self;
    fn id(&mut self, id: SurrealID) -> &mut Self;
}

pub trait ContentSetImpl<'w> {
    fn content_set(&mut self, content_set: ContentSet<'w>) -> &mut Self;
    fn content(&mut self, obj: Object) -> &mut Self;
    fn set(&mut self) -> &mut Self;
    fn add(&mut self, field: &'w str, value: SurrealValue) -> &mut Self;
}

pub trait ReturnImpl {
    fn return_type(&mut self, return_type: ReturnType) -> &mut Self;
}

pub trait TimeoutImpl {
    fn timeout(&mut self, timeout: TimeOut) -> &mut Self;
}

///PARALLEL
pub trait ParallelImpl {
    fn parallel(&mut self) -> &mut Self;
}