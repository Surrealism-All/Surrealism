//!RELATE @from -> @table -> @with
//! 	[ CONTENT @value
//! 	  | SET @field = @value ...
//! 	]
//! 	[ RETURN [ NONE | BEFORE | AFTER | DIFF | @projections ... ]
//! 	[ TIMEOUT @duration ]
//! 	[ PARALLEL ]
//! ;

use super::{Statements, SQLRegion, SQLField, RegionField, Wrapper, COMMON_SEPARATOR, END_SEPARATOR, ContentType};

#[derive(Debug, Clone)]
pub struct RelateWrapper {
    ///关键词
    keyword: Statements,
    ///可获取值
    available: SQLRegion,
    content_region: SQLRegion,
    content_type: ContentType,
    timeout_region: SQLField,
    return_region: SQLField,
}

impl RelateWrapper{

}

impl Wrapper for RelateWrapper{
    fn new() -> Self {
        todo!()
    }

    fn commit(&mut self) -> &str {
        todo!()
    }

    fn get_keyword(&self) -> &Statements {
        todo!()
    }

    fn get_available(&self) -> &SQLRegion {
        todo!()
    }
}