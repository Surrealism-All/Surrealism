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

use serde::Serialize;
use self::create::CreateWrapper;
use crate::{Table, ReturnType, TimeUnit};

pub struct SQLBuilder;

impl SQLBuilder {
    // pub fn create(table: &str) -> CreateWrapper {
    //     CreateWrapper::new(table)
    // }
    // fn select() -> SelectWrapper {}
    // fn update() -> UpdateWrapper {}
    // fn insert() -> InsertWrapper {}
    // fn delete() -> DeleteWrapper {}
}

pub trait BaseFunc {
    /// 创建一个Table指定name和id
    fn new(table: &str) -> Self;
    /// 通过使用Table工具创建一个Wrapper
    /// 这样会指定好Table的name和id
    fn from(table: &Table) -> Self;
    /// 直接创建一个Wrapper
    fn new_no_args() -> Self;
    /// 指定table name
    fn table(&mut self, table_name: &str, table_id: &str) -> &mut Self;
    fn build(&self)->String;
}

pub trait ReturnFunc {
    fn return_for(&mut self,return_type: &str) -> &mut Self;
    fn return_from(&mut self,return_type: ReturnType) -> &mut Self;
}

pub trait TimeoutFunc {
    fn timeout(&mut self,num: u32, unit: TimeUnit) -> &mut Self;
}

///PARALLEL
pub trait ParallelFunc {
    fn parallel(&mut self)->&mut Self;
}