use surrealism::DefaultRes;
use surrealism::db::Decimal;

// [tests\src\main.rs:7] decimal1.to_string() = "1243.44dec"
#[tokio::main]
async fn main() -> DefaultRes<()> {
    let decimal1 = Decimal::new("1243.44");
    dbg!(decimal1.to_string());
    Ok(())
}