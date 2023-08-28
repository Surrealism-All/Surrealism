use surrealism::{SurrealismRes, SurrealID, SurrealValue};
use surrealism::builder::*;
use surrealism::builder::insert::InsertWrapperImpl;
use surrealism::builder::relate::RelateWrapperImpl;
use serde::{Serialize, Deserialize};
use surrealism::functions::Function;

#[derive(Debug, Serialize, Deserialize)]
struct Person<'a> {
    name: &'a str,
    company: &'a str,
    skills: Vec<&'a str>,
}

#[tokio::main]
async fn main() -> SurrealismRes<()> {
    let func1 = Function::array().add_from(SurrealValue::from(vec!["hello", "world"]), SurrealValue::from("Rust"));
    dbg!(func1);
    let func2 = Function::array().add_from(SurrealValue::from(vec![SurrealValue::from(5), SurrealValue::from(true)]), SurrealValue::from("Rust"));
    dbg!(func2);
    let func3 = Function::array().add(vec!["hello", "world"], "Rust");
    dbg!(func3);
    Ok(())
}

