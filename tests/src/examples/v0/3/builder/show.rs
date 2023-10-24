use std::ops::DerefMut;
use surrealism::builder::{SQLBuilderFactory, TableImpl};
use surrealism::db::{AdapterToValue, SurrealValue};
use surrealism::DefaultRes;

// [tests\src\main.rs:14] show1 = "SHOW CHANGES FOR TABLE reading SINCE '2023-10-22T09:33:06.501656600Z' LIMIT 10"
#[tokio::main]
async fn main() -> DefaultRes<()> {
    let show1 = SQLBuilderFactory::show()
        .table("reading")
        .since(SurrealValue::datetime().default().to_value())
        .limit(10)
        .to_string();
    dbg!(show1);
    Ok(())
}