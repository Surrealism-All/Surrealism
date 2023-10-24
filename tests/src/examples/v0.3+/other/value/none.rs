use surrealism::DefaultRes;
use surrealism::db::{SurrealValue};

// [tests\src\main.rs:10] none1.to_string() = "NONE"
// [tests\src\main.rs:11] none2.to_string() = "NONE"
// [tests\src\main.rs:12] none3.to_string() = "NONE"
#[tokio::main]
async fn main() -> DefaultRes<()> {
    let none1 = SurrealValue::none();
    let none2 = SurrealValue::from("None");
    let none3:SurrealValue = "NONE".into();
    dbg!(none1.to_string());
    dbg!(none2.to_string());
    dbg!(none3.to_string());
    Ok(())
}