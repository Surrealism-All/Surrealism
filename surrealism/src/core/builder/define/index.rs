//! # Define Index
//! ```txt
//! @author:syf20020816@Outlook.com
//! @date:2023/10/31
//! @version:0.0.1
//! @description:
//! ```

use std::fmt::{Display, Formatter};
use super::FieldColumn;

#[derive(Debug,Clone)]
pub struct DefineIndex<'a>{
    name: & 'a str,
    on: & 'a str,
    field_column: FieldColumn<'a >,
    unique: bool,
}
impl<'a> Default for DefineIndex<'a>{
    fn default() -> Self {
        todo!()
    }
}
impl<'a> DefineIndex<'a> {

}
impl<'a> Display for  DefineIndex<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}