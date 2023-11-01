//! # Define Index
//!
//! DEFINE INDEX @name ON [ TABLE ] @table [ FIELDS | COLUMNS ] @fields
//! 	[ UNIQUE | SEARCH ANALYZER @analyzer [ BM25 [(@k1, @b)] ] [ HIGHLIGHTS ] ]
//!
//! ```txt
//! @author:syf20020816@Outlook.com
//! @date:2023/10/31
//! @version:0.0.1
//! @description:
//! ```

use std::fmt::{Display, Formatter};
use crate::db::constants::{DEFINE_INDEX, ON, STMT_END, UNIQUE};
use super::{FieldColumn, OnType};

#[derive(Debug, Clone)]
pub struct DefineIndex<'a> {
    name: &'a str,
    on: OnType<'a>,
    field_column: FieldColumn<'a>,
    unique_search: Option<UniqueSearch<'a>>,
}

impl<'a> Default for DefineIndex<'a> {
    fn default() -> Self {
        DefineIndex {
            name: "",
            on: OnType::TABLE(""),
            field_column: FieldColumn::default(),
            unique_search: None,
        }
    }
}

impl<'a> DefineIndex<'a> {
    pub fn new(
        name: &'a str,
        on: &'a str,
        field_column: FieldColumn<'a>,
        unique_search: Option<UniqueSearch<'a>>,
    ) -> Self {
        DefineIndex {
            name,
            on: OnType::TABLE(on),
            field_column,
            unique_search,
        }
    }
    pub fn name(&mut self, name: &'a str) -> &mut Self {
        self.name = name;
        self
    }
    pub fn on(&mut self, on: &'a str) -> &mut Self {
        self.on = OnType::TABLE(on);
        self
    }
    pub fn field_column(&mut self, columns: FieldColumn<'a>) -> &mut Self {
        self.field_column = columns;
        self
    }
    pub fn unique_search(&mut self, unique_search: UniqueSearch<'a>) -> &mut Self {
        self.unique_search.replace(unique_search);
        self
    }
}

impl<'a> Display for DefineIndex<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {} {} {} {}", DEFINE_INDEX, self.name, ON, &self.on.to_string(), &self.field_column.to_string());
        if let Some(u_s) = self.unique_search.as_ref() {
            write!(f, " {}", &u_s.to_string());
        }
        write!(f, "{}", STMT_END)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum UniqueSearch<'u> {
    Unique,
    Search {
        analyzer: &'u str,
        bm25: Option<Vec<(&'u str, &'u str)>>,
        highlights: bool,
    },
}

impl<'u> Default for UniqueSearch<'u> {
    fn default() -> Self {
        UniqueSearch::Unique
    }
}

impl<'u> UniqueSearch<'u> {
    pub fn unique() -> Self {
        UniqueSearch::default()
    }
    /// Whole Params constructor -> Search
    pub fn new(
        analyzer: &'u str,
        bm25: Option<Vec<(&'u str, &'u str)>>,
        highlights: bool,
    ) -> Self {
        UniqueSearch::Search {
            analyzer,
            bm25,
            highlights,
        }
    }

    /// build a easy Search
    pub fn search(analyzer: &'u str) -> Self {
        UniqueSearch::Search {
            analyzer,
            bm25: None,
            highlights: false,
        }
    }
    /// push (@k1, @b) to UniqueSearch::Search BM25
    pub fn push(&mut self, key: &'u str, value: &'u str) -> &mut Self {
        match self {
            UniqueSearch::Unique => panic!("push function can not use on UniqueSearch::Unique"),
            UniqueSearch::Search { analyzer: _analyzer, bm25, highlights: _highlights } => {
                bm25.as_mut().unwrap().push((key, value))
            }
        }
        self
    }

    pub fn highlight(&mut self) -> &mut Self {
        match self {
            UniqueSearch::Unique => panic!("highlight function can not use on UniqueSearch::Unique"),
            UniqueSearch::Search { analyzer: _analyzer, bm25: _bm25, highlights } => {
                *highlights = true;
            }
        }
        self
    }
}

impl<'u> Display for UniqueSearch<'u> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            UniqueSearch::Unique => {
                write!(f, "{}", UNIQUE)
            }
            UniqueSearch::Search { analyzer, bm25, highlights } => {
                write!(f, "SEARCH ANALYZER {}", analyzer);
                if let Some(bm25) = bm25 {
                    write!(f, " {}", bm25.iter().map(|(k,v)| format!("{}, {}",k,v)).collect::<Vec<String>>().join(", "));
                }
                if *highlights {
                    return write!(f, " {}", highlights);
                }
                Ok(())
            }
        }
    }
}