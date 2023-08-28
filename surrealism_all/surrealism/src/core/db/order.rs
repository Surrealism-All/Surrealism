//! # Order By
//! ```txt
//! @author:syf20020816@Outlook.com
//! @date:2023/8/27
//! @version:0.0.1
//! @description:
//! ```

use crate::ParamCombine;
use super::constants::{RAND, ASC, DESC};

pub enum Order<'o> {
    Asc(Vec<&'o str>),
    Desc(Vec<&'o str>),
    Rand,
}

impl<'o> Order<'o> {
    pub fn new_asc(asc: Vec<&'o str>) -> Order {
        Order::Asc(asc)
    }
    pub fn new_desc(desc: Vec<&'o str>) -> Order {
        Order::Asc(desc)
    }
    pub fn new_rand() -> Self { Order::Rand }
    pub fn push(&mut self, item: &'o str) -> () {
        match self {
            Order::Rand => panic!("push function can just use in Order::Asc|Order::Desc"),
            Order::Asc(ref mut asc) => asc.push(item),
            Order::Desc(ref mut desc) => desc.push(item)
        }
    }
    pub fn build(&self) -> String {
        match self {
            Order::Asc(asc) => format!("{} {}", asc.join(" , "), ASC),
            Order::Desc(desc) => format!("{} {}", desc.join(" , "), ASC),
            Order::Rand => String::from(RAND)
        }
    }
}

impl<'o> Default for Order<'o> {
    fn default() -> Self {
        Order::Rand
    }
}

impl<'o> ParamCombine for Order<'o> {
    fn combine(&self) -> String {
        self.build()
    }
}