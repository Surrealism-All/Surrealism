//! # CreateWrapper
//! ```code
//! CREATE @targets
//! 	[ CONTENT @value
//! 	  | SET @field = @value ...
//! 	]
//! 	[ RETURN [ NONE | BEFORE | AFTER | DIFF | @projections ... ]
//! 	[ TIMEOUT @duration ]
//! 	[ PARALLEL ]
//! ;
//! ```
//! ```txt
//! @author:syf20020816@Outlook.com
//! @date:2023/8/4
//! @version:0.0.1
//! @description:
//! ```

use super::BaseFunc;
use crate::{ReturnType};

pub struct CreateWrapper {
    table_param: String,
    content_param: String,
    return_param: String,
    timeout: u32,
    parallel: bool,
}

impl BaseFunc for CreateWrapper {
    fn new() -> Self {
        CreateWrapper {
            table_param: String::new(),
            content_param: String::new(),
            return_param:String::new(),
            timeout: 0,
            parallel: false,
        }
    }
}