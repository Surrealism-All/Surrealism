use surrealism::DefaultRes;
use surrealism::db::{SurrealValue};

// [tests\src\main.rs:13] int_num1.to_string() = "2022"
// [tests\src\main.rs:14] int_num2.to_string() = "2023"
// [tests\src\main.rs:15] int_num3.to_string() = "2024"
// [tests\src\main.rs:16] float_num1.to_string() = "41.5"
// [tests\src\main.rs:17] float_num2.to_string() = "56.23"
// [tests\src\main.rs:18] decimal_num1.to_string() = "99.99dec"
// [tests\src\main.rs:19] decimal_num2.to_string() = "564.22dec"
#[tokio::main]
async fn main() -> DefaultRes<()> {
    let int_num1 = SurrealValue::Int(2022);
    let int_num2 = SurrealValue::int(2023);
    let int_num3:SurrealValue = 2024.into();
    let float_num1 = SurrealValue::float(41.5);
    let float_num2:SurrealValue = 56.23.into();
    let decimal_num1 = SurrealValue::decimal(99.99);
    let decimal_num2 = SurrealValue::decimal_str("564.22");
    dbg!(int_num1.to_string());
    dbg!(int_num2.to_string());
    dbg!(int_num3.to_string());
    dbg!(float_num1.to_string());
    dbg!(float_num2.to_string());
    dbg!(decimal_num1.to_string());
    dbg!(decimal_num2.to_string());
    Ok(())
}