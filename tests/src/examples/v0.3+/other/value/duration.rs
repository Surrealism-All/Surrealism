use surrealism::DefaultRes;
use surrealism::db::{SurrealValue};

// [tests\src\main.rs:11] dur_day.to_string() = "1d"
// [tests\src\main.rs:12] dur_weak.to_string() = "12w"
// [tests\src\main.rs:13] dur_min.to_string() = "20m"
#[tokio::main]
async fn main() -> DefaultRes<()> {
    let dur_day = SurrealValue::duration().from_days(1);
    let dur_weak = SurrealValue::duration().from_weeks(12);
    let dur_min  = SurrealValue::duration().from_mins(20);
    dbg!(dur_day.to_string());
    dbg!(dur_weak.to_string());
    dbg!(dur_min.to_string());
    Ok(())
}