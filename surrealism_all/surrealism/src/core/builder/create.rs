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
use super::{BaseWrapperImpl, TableImpl, ContentSetImpl, ReturnImpl, TimeoutImpl, ParallelImpl};
use crate::core::db::constants::{CREATE, BLANK, PARALLEL, STMT_END};
use crate::core::db::{ReturnType, Table, TimeOut, ContentSet, SurrealID, ParamCombine, Object, SurrealValue};

pub trait CreateWrapperImpl<'w>: BaseWrapperImpl + TableImpl + ContentSetImpl<'w> + ReturnImpl + TimeoutImpl + ParallelImpl {}

/// # CreateWrapper
/// 1. TableImpl
/// 2. ContentSetImpl
/// 3. BaseWrapperImpl
/// 4. ReturnImpl
/// 5. TimeoutImpl
/// 6. ParallelImpl
/// ## example
/// ```rust
/// use surrealism::{SurrealismRes, SurrealID, TimeOut, SurrealValue, TimeUnit, ReturnType, Object};
/// use surrealism::builder::*;
/// use serde::{Serialize, Deserialize};
///
/// #[derive(Debug, Serialize, Deserialize)]
/// struct User<'a> {
///     name: &'a str,
///     age: u32,
///     works: Vec<&'a str>,
/// }
///
/// #[tokio::main]
/// async fn main() -> SurrealismRes<()> {
///     // use set : CREATE surrealism:rand() SET name = 'Mat' RETURN AFTER TIMEOUT 5s PARALLEL;
///     let mut create = SQLBuilderFactory::create()
///         .table("surrealism")
///         .id(SurrealID::RAND)
///         .set()
///         .add("name", "Mat")
///         .timeout(TimeOut::new(5, TimeUnit::SECOND))
///         .return_type(ReturnType::After)
///         .parallel()
///         .deref_mut();
///     dbg!(&create.build());
///     // use content : CREATE surrealdb:ulid() CONTENT { age : 16 , name : 'Mat' , works : ['cook'] } RETURN name;
///     let user = User {
///         name: "Mat",
///         age: 16,
///         works: vec!["cook"],
///     };
///     let mut create2 = SQLBuilderFactory::create()
///         .table("surrealdb")
///         .id(SurrealID::ULID)
///         .content(&user)
///         .return_type(ReturnType::Field("name"))
///         .deref_mut();
///     dbg!(create2.build());
///     Ok(())
/// }
/// ```
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
        if self.return_type.is_some() {
            res.push_str(BLANK);
            res.push_str(&self.return_type.as_ref().unwrap().combine());
        }
        if self.timeout.is_some() {
            res.push_str(BLANK);
            res.push_str(&self.timeout.as_ref().unwrap().combine());
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
    fn content_obj(&mut self, obj: Object) -> &mut Self {
        match self.content {
            None => self.content = Some(ContentSet::new_content(obj)),
            Some(_) => {
                let _ = self.content.replace(ContentSet::Content(obj));
            }
        };
        self
    }
    fn content<T>(&mut self, obj: &'w T) -> &mut Self where T: Serialize {
        self.content_obj(Object::from_obj(obj))
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
    fn add_from_value(&mut self, field: &'w str, value: SurrealValue) -> &mut Self {
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
    fn add<T>(&mut self, field: &'w str, value: T) -> &mut Self where T: Serialize {
        self.add_from_value(field, SurrealValue::from(serde_json::to_value(value).unwrap()))
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

