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


use super::{BaseWrapperImpl, TableImpl, ContentSetImpl, ReturnImpl, TimeoutImpl, ParallelImpl};
use crate::core::db::constants::{CREATE, BLANK, PARALLEL, STMT_END};
use crate::core::db::{ReturnType, Table, TimeOut, ContentSet, SurrealID, ParamCombine, Object, SurrealValue};

pub trait CreateWrapperImpl<'w>: BaseWrapperImpl + TableImpl + ContentSetImpl<'w> + ReturnImpl + TimeoutImpl + ParallelImpl {}

#[derive(Debug)]
pub struct CreateWrapper<'w> {
    table: Table,
    content: Option<ContentSet<'w>>,
    return_type: Option<ReturnType>,
    timeout: Option<TimeOut>,
    parallel: bool,
}

impl<'w> BaseWrapperImpl for CreateWrapper<'w> {
    fn new() -> Self {
        CreateWrapper {
            table: Table::default(),
            content: None,
            return_type: None,
            timeout: None,
            parallel: false,
        }
    }

    fn deref_mut(&mut self) -> Self {
        CreateWrapper {
            table: self.table.clone(),
            content: self.content.clone(),
            return_type: self.return_type.clone(),
            timeout: self.timeout.clone(),
            parallel: self.parallel,
        }
    }

    fn build(&mut self) -> String {
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

impl<'w> TableImpl for CreateWrapper<'w> {
    fn table(&mut self, table: &str) -> &mut Self {
        self.table.table(table);
        self
    }

    fn id(&mut self, id: SurrealID) -> &mut Self {
        self.table.id(id);
        self
    }
}

impl<'w> ContentSetImpl<'w> for CreateWrapper<'w> {
    fn content_set(&mut self, content_set: ContentSet<'w>) -> &mut Self {
        let _ = self.content.replace(content_set);
        self
    }

    fn content(&mut self, obj: Object) -> &mut Self {
        match self.content {
            None => self.content = Some(ContentSet::new_content(obj)),
            Some(_) => {
                let _ = self.content.replace(ContentSet::Content(obj));
            }
        };
        self
    }

    fn set(&mut self) -> &mut Self {
        match self.content {
            None => self.content = Some(ContentSet::new_empty_set()),
            Some(_) => {
                let _ = self.content.replace(ContentSet::new_empty_set());
            }
        };
        self
    }

    fn add(&mut self, field: &'w str, value: SurrealValue) -> &mut Self {
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
}

impl<'w> ReturnImpl for CreateWrapper<'w> {
    fn return_type(&mut self, return_type: ReturnType) -> &mut Self {
        let _ = self.return_type.replace(return_type);
        self
    }
}

impl<'w> TimeoutImpl for CreateWrapper<'w> {
    fn timeout(&mut self, timeout: TimeOut) -> &mut Self {
        let _ = self.timeout.replace(timeout);
        self
    }
}

impl<'w> ParallelImpl for CreateWrapper<'w> {
    fn parallel(&mut self) -> &mut Self {
        self.parallel = true;
        self
    }
}

impl<'w> CreateWrapperImpl<'w> for CreateWrapper<'w> {}
//
// impl<'w> CreateWrapper<'w> {
//     pub fn new() -> Self {
//         CreateWrapper {
//             table: Table::default(),
//             content: None,
//             return_type: None,
//             timeout: None,
//             parallel: false,
//         }
//     }
//     pub fn table(&mut self, table: &str) -> &mut Self {
//         self.table.table(table);
//         self
//     }
//     pub fn id(&mut self, id: SurrealID) -> &mut Self {
//         self.table.id(id);
//         self
//     }
//     pub fn content_set(&mut self, content_set: ContentSet<'w>) -> &mut Self {
//         let _ = self.content.replace(content_set);
//         self
//     }
//     pub fn content(&mut self, obj: Object) -> &mut Self {
//         match self.content {
//             None => self.content = Some(ContentSet::new_content(obj)),
//             Some(_) => {
//                 let _ = self.content.replace(ContentSet::Content(obj));
//             }
//         };
//         self
//     }
//     pub fn set(&mut self) -> &mut Self {
//         match self.content {
//             None => self.content = Some(ContentSet::new_empty_set()),
//             Some(_) => {
//                 let _ = self.content.replace(ContentSet::new_empty_set());
//             }
//         };
//         self
//     }
//     pub fn add(&mut self, field: &'w str, value: SurrealValue) -> &mut Self {
//         match self.content {
//             None => {
//                 let mut v = ContentSet::new_empty_set();
//                 let _ = v.add(field, value);
//                 self.content = Some(v);
//             }
//             Some(ref content_set) => {
//                 if content_set.is_set() {
//                     let _ = self.content.as_mut().unwrap().add(field, value);
//                 } else {
//                     panic!("ContentSet::Content cannot use add function")
//                 }
//             }
//         };
//         self
//     }
//     pub fn return_type(&mut self, return_type: ReturnType) -> &mut Self {
//         let _ = self.return_type.replace(return_type);
//         self
//     }
//     pub fn timeout(&mut self, timeout: TimeOut) -> &mut Self {
//         let _ = self.timeout.replace(timeout);
//         self
//     }
//     pub fn parallel(&mut self) -> &mut Self {
//         self.parallel = true;
//         self
//     }
//     pub fn deref_mut(&mut self) -> Self {
//         CreateWrapper {
//             table: self.table.clone(),
//             content: self.content.clone(),
//             return_type: self.return_type.clone(),
//             timeout: self.timeout.clone(),
//             parallel: self.parallel,
//         }
//     }
//     pub fn build(&mut self) -> String {
//
//         let mut res = format!("{} {}", CREATE, &self.table.combine());
//         if self.content.is_some() {
//             res.push_str(BLANK);
//             res.push_str(&self.content.as_ref().unwrap().combine());
//         }
//         if self.timeout.is_some() {
//             res.push_str(BLANK);
//             res.push_str(&self.timeout.as_ref().unwrap().combine());
//         }
//         if self.return_type.is_some() {
//             res.push_str(BLANK);
//             res.push_str(&self.return_type.as_ref().unwrap().combine());
//         }
//         if self.parallel {
//             res.push_str(BLANK);
//             res.push_str(PARALLEL);
//         }
//         res.push_str(STMT_END);
//         res
//     }
// }
