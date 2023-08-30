use crate::{Array, SurrealValue};

mod array;
mod count;

use array::ArrayFunc;

pub struct Function;

impl Function {
    pub fn array() -> ArrayFunc {
        ArrayFunc::default()
    }
    pub fn get(&self) -> &Self {
        self
    }
}

pub enum Functions {
    Array,
    Count,
    Crypto,
    Geo,
    HTTP,
    Validation,
    Math,
    Meta,
    Parse,
    Rand,
    Session,
    Sleep,
    String,
    Time,
    Type,
    Scripting,
}

