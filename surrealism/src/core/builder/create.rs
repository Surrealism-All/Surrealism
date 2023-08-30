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
use super::{BaseWrapperImpl, TableImpl, ReturnImpl, TimeoutImpl, ParallelImpl};
use crate::core::db::constants::{CREATE, BLANK, PARALLEL, STMT_END};
use crate::core::db::{ReturnType, Table, TimeOut, SurrealID, ParamCombine, Object, SurrealValue, CreateStrategy};
use crate::{Operator, Set, TimeUnit,parallel_impl,return_impl,timeout_impl,table_impl};

pub trait CreateWrapperImpl: BaseWrapperImpl + TableImpl + ReturnImpl + TimeoutImpl + ParallelImpl {
    fn content<T>(&mut self, obj: &T) -> &mut Self where T: Serialize;
    fn content_obj(&mut self, obj: Object) -> &mut Self;
    fn set(&mut self) -> &mut Self;
    fn add<T>(&mut self, field: &str, value: T) -> &mut Self where T: Serialize;
    fn add_from_value(&mut self, field: &str, value: SurrealValue) -> &mut Self;
}

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
///         .timeout(5,TimeUnit::SECOND)
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
#[cfg(feature = "builder")]
#[derive(Debug, Clone)]
pub struct CreateWrapper {
    table: Table,
    content: Option<CreateStrategy>,
    return_type: Option<ReturnType>,
    timeout: Option<TimeOut>,
    parallel: bool,
}

impl BaseWrapperImpl for CreateWrapper {
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


impl CreateWrapperImpl for CreateWrapper {
    fn content<T>(&mut self, obj: &T) -> &mut Self where T: Serialize {
        self.content_obj(Object::from_obj(obj))
    }
    fn content_obj(&mut self, obj: Object) -> &mut Self {
        match self.content {
            None => self.content = Some(CreateStrategy::from(obj)),
            Some(_) => {
                let _ = self.content.replace(CreateStrategy::from(obj));
            }
        };
        self
    }
    fn set(&mut self) -> &mut Self {
        match self.content {
            None => self.content = Some(CreateStrategy::from(vec![])),
            Some(_) => {
                let _ = self.content.replace(CreateStrategy::from(vec![]));
            }
        };
        self
    }
    fn add<T>(&mut self, field: &str, value: T) -> &mut Self where T: Serialize {
        self.add_from_value(field, SurrealValue::from(serde_json::to_value(value).unwrap()))
    }
    fn add_from_value(&mut self, field: &str, value: SurrealValue) -> &mut Self {
        let item = Set::new(field, value, Operator::Eq);
        match self.content {
            None => {
                self.content = Some(CreateStrategy::Set(vec![item]));
            }
            Some(ref mut strategy) => {
                strategy.push(item);
            }
        };
        self
    }
}

table_impl!(CreateWrapper);
timeout_impl!(CreateWrapper);
parallel_impl!(CreateWrapper);
return_impl!(CreateWrapper);