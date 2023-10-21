use surrealism::DefaultRes;
use surrealism::db::{SurrealValue};

// [tests\src\main.rs:10] bool1.to_string() = "true"
// [tests\src\main.rs:11] bool2.to_string() = "false"
// [tests\src\main.rs:12] bool3.to_string() = "true"
#[tokio::main]
async fn main() -> DefaultRes<()> {
    let bool1 = SurrealValue::Bool(true);
    let bool2:SurrealValue = false.into();
    let bool3 = SurrealValue::bool(true);
    dbg!(bool1.to_string());
    dbg!(bool2.to_string());
    dbg!(bool3.to_string());
    Ok(())
}