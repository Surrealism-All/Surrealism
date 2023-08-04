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

use self::create::CreateWrapper;

pub struct SQLBuilder {}

impl SQLBuilder {
    fn create() -> CreateWrapper {
        CreateWrapper::new()
    }
    // fn select() -> SelectWrapper {}
    // fn update() -> UpdateWrapper {}
    // fn insert() -> InsertWrapper {}
    // fn delete() -> DeleteWrapper {}
}

pub trait BaseFunc{
    fn new()->Self;
}
pub trait ReturnFunc{}
pub trait TimeoutFunc{}
///PARALLEL
pub trait ParallelFunc{}