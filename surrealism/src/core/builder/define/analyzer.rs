//! # Define Analyzer
//! DEFINE ANALYZER @name [ TOKENIZERS @tokenizers ] [ FILTERS @filters ]
//! ```txt
//! @author:syf20020816@Outlook.com
//! @date:2023/10/31
//! @version:0.0.1
//! @description:
//! ```

use std::fmt::{Display, Formatter};
use crate::db::constants::{BLANK, DEFINE_ANALYZER, FILTERS, STMT_END, TOKENIZERS};

#[derive(Debug, Clone)]
pub struct DefineAnalyzer<'a> {
    name: &'a str,
    tokenizers: Option<Vec<&'a str>>,
    filters: Option<Vec<&'a str>>,
}

impl<'a> Default for DefineAnalyzer<'a> {
    fn default() -> Self {
        DefineAnalyzer {
            name: "",
            tokenizers: None,
            filters: None,
        }
    }
}

impl<'a> DefineAnalyzer<'a> {
    pub fn new(name: &'a str, tokenizers: Vec<&'a str>, filter: Vec<&'a str>) -> Self {
        DefineAnalyzer {
            name,
            tokenizers: Some(tokenizers),
            filters: Some(filter),
        }
    }
    pub fn name(&mut self, name: &'a str) -> &mut Self {
        self.name = name;
        self
    }
    pub fn tokenizer(&mut self, item: &'a str) -> &mut Self {
        match self.tokenizers {
            None => {
                let _ = self.tokenizers.replace(Vec::new());
                let _ = self.tokenizer(item);
            }
            Some(ref mut tokenizers) => tokenizers.push(item)
        }
        self
    }
    pub fn filter(&mut self, item: &'a str) -> &mut Self {
        match self.filters {
            None => {
                let _ = self.filters.replace(Vec::new());
                let _ = self.filter(item);
            }
            Some(ref mut filters) => filters.push(item)
        }
        self
    }
    pub fn build(&self) -> String {
        self.to_string()
    }
}

impl<'a> Display for DefineAnalyzer<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut res = format!("{} {}", DEFINE_ANALYZER, self.name);
        if self.tokenizers.is_some() {
            res.push_str(format!(" {} {}", TOKENIZERS, self.tokenizers.as_ref().unwrap().join(", ")).as_str());
        }
        if self.filters.is_some() {
            res.push_str(format!(" {} {}", FILTERS, self.filters.as_ref().unwrap().join(", ")).as_str())
        }
        res.push_str(STMT_END);
        write!(f, "{}", res)
    }
}