use surrealism::DefaultRes;
use surrealism::db::{SurrealValue};

// [tests\src\main.rs:12] null1.to_string() = "NULL"
// [tests\src\main.rs:13] null2.to_string() = "NULL"
// [tests\src\main.rs:14] null3.to_string() = "NULL"
#[tokio::main]
async fn main() -> DefaultRes<()> {
    let null1 = SurrealValue::null();
    let null2 = SurrealValue::from("Null");
    let null3:SurrealValue = "NULL".into();
    dbg!(null1.to_string());
    dbg!(null2.to_string());
    dbg!(null3.to_string());
    Ok(())
}