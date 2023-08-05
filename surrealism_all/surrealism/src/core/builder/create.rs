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

use serde::Serialize;
use super::BaseFunc;
use crate::{ReturnType, Table};

pub struct CreateWrapper {
    table_param: String,
    content_param: String,
    return_param: String,
    timeout: u32,
    parallel: bool,
}

impl BaseFunc for CreateWrapper {
    fn new(table: &str) -> Self {
        CreateWrapper {
            table_param: String::from(table),
            content_param: String::new(),
            return_param: String::new(),
            timeout: 0,
            parallel: false,
        }
    }

    fn from<T: Serialize>(table: &Table<T>) -> Self {
        let table_param = table.build();
        CreateWrapper {
            table_param,
            content_param: String::new(),
            return_param: String::new(),
            timeout: 0,
            parallel: false,
        }
    }

    fn new_no_args() -> Self {
        CreateWrapper {
            table_param: String::new(),
            content_param: String::new(),
            return_param: String::new(),
            timeout: 0,
            parallel: false,
        }
    }

    fn table(&mut self, table_name: &str, table_id: &str) -> &mut Self {
        self.table_param = format!("{}:{}", table_name, table_id);
        self
    }
}