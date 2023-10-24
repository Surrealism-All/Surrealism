use surrealism::{SurrealismRes, SurrealID, SurrealValue};
use surrealism::functions::Function;


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