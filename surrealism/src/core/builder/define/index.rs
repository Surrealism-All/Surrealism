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
use super::{FieldColumn, OnType};

#[derive(Debug, Clone)]
pub struct DefineIndex<'a> {
    name: &'a str,
    on: OnType<'a>,
    field_column: Option<FieldColumn<'a>>,
    unique_search: Option<UniqueSearch<'a>>,
}

impl<'a> Default for DefineIndex<'a> {
    fn default() -> Self {
        DefineIndex {
            name: "",
            on: OnType::TABLE(""),
            field_column: None,
            unique_search: None,
        }
    }
}

impl<'a> DefineIndex<'a> {
    pub fn new(
        name: &'a str,
        on: OnType<'a>,
        field_column: Option<FieldColumn<'a>>,
        unique_search: Option<UniqueSearch<'a>>,
    )->Self{
        DefineIndex{
            name,
            on,
            field_column,
            unique_search
        }
    }
    pub fn name(&mut self,name:&'a str)->&mut Self{
        self.name = name;
        self
    }
    pub fn on(&mut self,on:OnType<'a>)->&mut Self{
        self.on = on;
        self
    }
    pub fn field_column(&mut self,columns:FieldColumn)->&mut Self{
        self.field_column.replace(columns);
        self
    }
}

impl<'a> Display for DefineIndex<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

pub enum UniqueSearch<'u> {
    Unique,
    Search {
        analyzer: &'u str,
        bm25: Option<Vec<(&'u str, &'u str)>>,
        highlights: bool,
    },
}

impl<'u> Default for UniqueSearch<'u>{
    fn default() -> Self {
        UniqueSearch::Unique
    }
}

impl<'u> UniqueSearch<'u>{

}

impl<'u> UniqueSearch<'u>{

}