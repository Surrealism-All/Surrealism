//! # Insert Wrapper
//! ```code
//! INSERT [ IGNORE ] INTO @what
//! 	[ @value
//! 	  | (@fields) VALUES (@values)
//! 		[ ON DUPLICATE KEY UPDATE @field = @value ... ]
//! 	]
//! ;
//! ```
//! ```txt
//! @author:syf20020816@Outlook.com
//! @date:2023/8/26
//! @version:0.0.1
//! @description:
//! ```

use std::collections::{BTreeMap};
use serde::Serialize;
use crate::db::{Table, Criteria, SurrealValue, Object, SurrealID,  ParamCombine, Set, Operator, InsertStrategy};
use super::{BaseWrapperImpl, TableImpl};
use crate::db::constants::{STMT_END, INSERT_UPDATE, INSERT, BLANK, VALUES, INTO};
use crate::table_impl;

pub trait InsertWrapperImpl: BaseWrapperImpl + TableImpl {
    fn add_set<T>(&mut self, field: &str, value: T) -> &mut Self where T: Serialize;
    fn add_content<T>(&mut self, obj: &T) -> &mut Self where T: Serialize;
    fn add_stmt(&mut self, stmt: &str) -> &mut Self;
}

/// # InsertWrapper
/// the wrapper for INSERT statements
/// ```rust
/// use surrealism::{ SurrealID};
/// use surrealism::builder::*;
/// use surrealism::surreal::SurrealismRes;
/// use surrealism::builder::insert::InsertWrapperImpl;
/// use surrealism::builder::relate::RelateWrapperImpl;
/// use serde::{Serialize, Deserialize};
///
/// #[derive(Debug, Serialize, Deserialize)]
/// struct Person<'a> {
///     name: &'a str,
///     company: &'a str,
///     skills: Vec<&'a str>,
/// }
///
/// // [tests\src\main.rs:23] insert1.build() = "INSERT INTO company (founded , name) VALUES ('2021-09-10' , 'SurrealDB') , ('2023-01-01' , 'Surrealism');"
/// // [tests\src\main.rs:38] insert2.build() = "INSERT INTO company CONTENT { company : 'SurrealDB' , name : 'Tobie' , skills : ['Rust', 'Go', 'JS'] };"
/// // [tests\src\main.rs:44] insert3.build() = "INSERT INTO company CONTENT [ { company : 'SurrealDB' , name : 'Tobie' , skills : ['Rust', 'Go', 'JS'] } , { company : 'Surrealism' , name : 'Mat' , skills : ['TS'] } ];"
/// // [tests\src\main.rs:49] insert4.build() = "INSERT INTO company ( SELECT * FROM temperature WHERE city = 'San Francisco' );"
/// #[tokio::main]
/// async fn main() -> SurrealismRes<()> {
///     let mut insert1 = SQLBuilderFactory::insert()
///         .table("company")
///         .add_set("name", "SurrealDB")
///         .add_set("founded", "2021-09-10")
///         .add_set("name", "Surrealism")
///         .add_set("founded", "2023-01-01")
///         .deref_mut();
///     dbg!(insert1.build());
///     let person1 = Person {
///         name: "Tobie",
///         company: "SurrealDB",
///         skills: vec!["Rust", "Go", "JS"],
///     };
///     let person2 = Person {
///         name: "Mat",
///         company: "Surrealism",
///         skills: vec!["TS"],
///     };
///     let mut insert2 = SQLBuilderFactory::insert()
///         .table("company")
///         .add_content(&person1)
///         .deref_mut();
///     dbg!(insert2.build());
///     let mut insert3 = SQLBuilderFactory::insert()
///         .table("company")
///         .add_content(&person1)
///         .add_content(&person2)
///         .deref_mut();
///     dbg!(insert3.build());
///     let mut insert4 = SQLBuilderFactory::insert()
///         .table("company")
///         .add_stmt("SELECT * FROM temperature WHERE city = 'San Francisco'")
///         .deref_mut();
///     dbg!(insert4.build());
///     Ok(())
/// }
/// ```
#[derive(Debug, Clone)]
pub struct InsertWrapper {
    table: Table,
    values: Option<InsertStrategy>,
    update: Option<Criteria>,
}

impl BaseWrapperImpl for InsertWrapper {
    fn new() -> Self {
        InsertWrapper {
            table: Table::default(),
            values: None,
            update: None,
        }
    }

    fn deref_mut(&mut self) -> Self {
        InsertWrapper {
            table: self.table.clone(),
            values: self.values.clone(),
            update: self.update.clone(),
        }
    }

    fn build(&mut self) -> String {
        /// try to build self.values(InsertStrategy) to String
        /// ## example
        /// ``` code
        /// Set([Set {column: "name",value: Str("SurrealDB",),sign: Eq, },
        ///      Set {column: "founded",value: Str("2021-09-10",),sign: Eq, },
        ///      Set {column: "name",value: Str("Surrealism",),sign: Eq, },
        ///      Set {column: "founded",value: Str("2023-01-01",),sign: Eq, },],),
        /// ```
        /// after build like : (founded , name) VALUES ('2021-09-10' , 'SurrealDB') , ('2023-01-01' , 'Surrealism');
        fn build_set(sets: &Vec<Set>) -> String {
            let mut tmp_data: BTreeMap<&str, Vec<String>> = BTreeMap::new();
            for set in sets {
                let (k, v) = set.get();
                // try to get the key and then insert
                let mut value_len = 0_usize;
                match tmp_data.get(k) {
                    None => {}
                    Some(res) => value_len = res.len()
                }
                if 0.eq(&value_len) {
                    let _ = tmp_data.insert(k, vec![v.to_string()]);
                } else {
                    tmp_data.get_mut(k).unwrap().push(v.to_string());
                }
            }

            let keys = format!("({})", &tmp_data.keys().map(|key| key.to_string()).collect::<Vec<String>>().join(" , "));
            let values = tmp_data.values().collect::<Vec<&Vec<String>>>();
            // judge length ,if lengths are different -> panic
            let counter = values[0].len();
            for value in &values {
                if counter.ne(&value.len()) {
                    panic!("{}", "value length are different please check!")
                }
            }
            // transpose values then to String
            let row_len = values.len();
            let col_len = values[0].len();
            let mut transpose_values = vec![vec![String::new(); row_len]; col_len];
            for i in 0..col_len {
                for j in 0..row_len {
                    transpose_values[i][j] = values[j][i].clone();
                }
            }
            let values = transpose_values.iter().map(|x| format!("({})", x.join(" , "))).collect::<Vec<String>>().join(" , ");
            format!("{} {} {}", keys, VALUES, values)
        }
        let mut res = format!("{} {} {}", INSERT, INTO, &self.table.combine());
        if self.values.is_some() {
            res.push_str(BLANK);
            match self.values.as_ref().unwrap() {
                InsertStrategy::Set(s) => {
                    res.push_str(build_set(s).as_str());
                }
                _ => res.push_str(self.values.as_ref().unwrap().combine().as_str())
                // InsertStrategy::Content(c) => {
                //     res.push_str(c.parse().as_str())
                // }
                //
                // InsertStrategy::Stmt(s) => res.push_str(s.as_str())
            }
        }
        if self.update.is_some() {
            res.push_str(BLANK);
            res.push_str(format!("{} {}", INSERT_UPDATE, &self.update.as_ref().unwrap().combine()).as_str());
        }
        res.push_str(STMT_END);
        res
    }
}


impl InsertWrapperImpl for InsertWrapper {
    fn add_set<T>(&mut self, field: &str, value: T) -> &mut Self where T: Serialize {
        let item = Set::new(field, SurrealValue::from(serde_json::to_value(value).unwrap()), Operator::Eq);
        match self.values {
            None => self.values = Some(InsertStrategy::from(vec![item])),
            Some(ref mut values) => {
                let _ = values.push_set(item);
            }
        };
        self
    }
    fn add_content<T>(&mut self, obj: &T) -> &mut Self where T: Serialize {
        // self.content_obj(Object::from_obj(obj))
        let obj = Object::from_obj(obj);
        match self.values {
            None => self.values = Some(InsertStrategy::from(vec![obj])),
            Some(ref mut values) => {
                let _ = values.push_content(obj);
            }
        }
        self
    }

    fn add_stmt(&mut self, stmt: &str) -> &mut Self {
        match self.values {
            None => self.values = Some(InsertStrategy::from(stmt)),
            Some(ref mut values) => {
                let _ = values.push(stmt);
            }
        };
        self
    }
}

table_impl!(InsertWrapper);