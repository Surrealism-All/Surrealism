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
use crate::core::{CREATE, BLANK, PARALLEL, STMT_END};
use crate::{ReturnType, Table, TimeUnit, TimeOut, ContentSet, SurrealID, match_id_type, SurrealIDType, ParamCombine, Object, SurrealValue};

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
    pub fn id(&mut self, id: SurrealID) -> &mut Self {
        self.table.id(id);
        self
    }
    pub fn content_set(&mut self, content_set: ContentSet<'w>) -> &mut Self {
        let _ = self.content.replace(content_set);
        self
    }
    pub fn content(&mut self, obj: Object) -> &mut Self {
        match self.content {
            None => self.content = Some(ContentSet::new_content(obj)),
            Some(_) => {
                let _ = self.content.replace(ContentSet::Content(obj));
            }
        };
        self
    }
    pub fn set(&mut self) -> &mut Self {
        match self.content {
            None => self.content = Some(ContentSet::new_empty_set()),
            Some(_) => {
                let _ = self.content.replace(ContentSet::new_empty_set());
            }
        };
        self
    }
    pub fn add(&mut self, field: &'w str, value: SurrealValue) -> &mut Self {
        match self.content {
            None => {
                let mut v = ContentSet::new_empty_set();
                let _ = v.add(field, value);
                self.content = Some(v);
            }
            Some(ref content_set) => {
                if content_set.is_set() {
                    let _ = self.content.as_mut().unwrap().add(field, value);
                } else {
                    panic!("ContentSet::Content cannot use add function")
                }
            }
        };
        self
    }
    pub fn return_type(&mut self, return_type: ReturnType) -> &mut Self {
        let _ = self.return_type.replace(return_type);
        self
    }
    pub fn timeout(&mut self, timeout: TimeOut) -> &mut Self {
        let _ = self.timeout.replace(timeout);
        self
    }
    pub fn parallel(&mut self) -> &mut Self {
        self.parallel = true;
        self
    }
    pub fn build(&mut self) -> String {
        // let wrapper = self.clone();
        let mut res = format!("{} {}", CREATE, &self.table.combine());
        if self.content.is_some() {
            res.push_str(BLANK);
            res.push_str(&self.content.as_ref().unwrap().combine());
        }
        if self.timeout.is_some() {
            res.push_str(BLANK);
            res.push_str(&self.timeout.as_ref().unwrap().combine());
        }
        if self.return_type.is_some() {
            res.push_str(BLANK);
            res.push_str(&self.return_type.as_ref().unwrap().combine());
        }
        if self.parallel {
            res.push_str(BLANK);
            res.push_str(PARALLEL);
        }
        res.push_str(STMT_END);
        res
    }
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