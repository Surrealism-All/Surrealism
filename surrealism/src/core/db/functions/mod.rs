mod array;
mod crypto;

use array::ArrayFunc;
use crate::SurrealValue;
pub use crypto::{CryptoFunc, GenerateCompare};

pub struct Function;

impl Function {
    pub fn array() -> ArrayFunc {
        ArrayFunc::default()
    }
    /// only have one function
    pub fn count(value: SurrealValue) -> String {
        format!("count({})", value.to_str())
    }
    pub fn crypto<'a>() -> CryptoFunc<'a> {
        CryptoFunc::default()
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


pub fn generate_easy(prefix: &str, f_name: &str, value: &SurrealValue) -> String {
    format!("{}::{}({})", prefix, f_name, value.to_str())
}