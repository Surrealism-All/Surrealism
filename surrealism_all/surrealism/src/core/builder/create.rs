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

use std::any::{Any, TypeId};
use serde::Serialize;
use surrealdb::method::Content;
use super::{BaseFunc, ParallelFunc, ReturnFunc, TimeoutFunc};
use crate::{ReturnType, Table, TimeUnit, TimeOut, ContentSet, SurrealID};

#[derive(Debug)]
pub struct CreateWrapper<'w> {
    table: Table,
    content: Option<ContentSet<'w>>,
    return_type: Option<ReturnType>,
    timeout: Option<TimeOut>,
    parallel: bool,
}

impl<'w> CreateWrapper<'w> {
    pub fn new() -> Self {
        CreateWrapper {
            table: Table::default(),
            content: None,
            return_type: None,
            timeout: None,
            parallel: false,
        }
    }
    pub fn table(&mut self, table: &str) -> &mut Self {
        self.table.table(table);
        self
    }
    // pub fn id<T>(&mut self, id: T) -> &mut Self {
    //     match id.type_id() {
    //         TypeId { .. } => {}
    //     }
    //     self.table.id(SurrealID::);
    //     self
    // }
}

// impl BaseFunc for CreateWrapper {
//     fn new(table: &str) -> Self {
//         CreateWrapper {
//             table_param: String::from(table),
//             content_param: String::new(),
//             return_param: String::new(),
//             timeout: 0,
//             time_unit: TimeUnit::SECOND,
//             parallel: false,
//         }
//     }
//
//     fn from<T: Serialize>(table: &Table<T>) -> Self {
//         let table_param = table.build();
//         CreateWrapper {
//             table_param,
//             content_param: String::new(),
//             return_param: String::new(),
//             timeout: 0,
//             time_unit: TimeUnit::SECOND,
//             parallel: false,
//         }
//     }
//
//     fn new_no_args() -> Self {
//         CreateWrapper {
//             table_param: String::new(),
//             content_param: String::new(),
//             return_param: String::new(),
//             timeout: 0,
//             time_unit: TimeUnit::SECOND,
//             parallel: false,
//         }
//     }
//
//     fn table(&mut self, table_name: &str, table_id: &str) -> &mut Self {
//         self.table_param = format!("{}:{}", table_name, table_id);
//         self
//     }
//
//     fn build(&self) -> String {
//         todo!()
//     }
// }

// impl TimeoutFunc for CreateWrapper {
//     fn timeout(&mut self, num: u32, unit: TimeUnit) -> &mut Self {
//         self.timeout = num;
//         self.time_unit = unit;
//         self
//     }
// }
//
// impl ReturnFunc for CreateWrapper {
//     fn return_for(&mut self, return_type: &str) -> &mut Self {
//         self.return_param = String::from(return_type);
//         self
//     }
//
//     fn return_from(&mut self, return_type: ReturnType) -> &mut Self {
//         self.return_param = String::from(return_type.to_str());
//         self
//     }
// }
//
// impl ParallelFunc for CreateWrapper {
//     fn parallel(&mut self) -> &mut Self {
//         self.parallel = true;
//         self
//     }
// }