use surrealism::DefaultRes;
use surrealism::db::{SurrealValue,AdapterToValue};

#[tokio::main]
async fn main() -> DefaultRes<()> {
    let datetime = SurrealValue::datetime().default().to_value();
    dbg!(datetime.to_string());
    Ok(())
}