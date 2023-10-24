use std::ops::DerefMut;
use surrealism::builder::{SQLBuilderFactory};
use surrealism::db::{AdapterToValue, SurrealValue};
use surrealism::DefaultRes;

// [tests\src\main.rs:10] sleep = "SLEEP 2d"
#[tokio::main]
async fn main() -> DefaultRes<()> {
    let sleep = SQLBuilderFactory::sleep(SurrealValue::duration().from_days(2).to_value());
    dbg!(sleep);
    Ok(())
}