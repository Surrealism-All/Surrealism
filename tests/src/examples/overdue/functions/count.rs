use surrealism::{SurrealismRes, UseNSDB, DefaultInitService, InitService, SurrealValue, parse_response, Criteria, CriteriaSign};
use surrealism::builder::select::SelectWrapperImpl;
use surrealism::builder::{BaseWrapperImpl, SQLBuilderFactory, TableImpl};
use surrealism::functions::{GenerateCompare, CryptoFunc, Function};

// [tests\src\main.rs:12] count1 = "count(true)"
// [tests\src\main.rs:13] count2 = "count()"
// [tests\src\main.rs:14] count3 = "count(10 > 15)"
// [tests\src\main.rs:15] count4 = "count(age > 15)"
// [tests\src\main.rs:22] select = "SELECT country , count(age > 30) AS total FROM test GROUP BY country;"
#[tokio::main]
async fn main() -> SurrealismRes<()> {
    let count1 = Function::count(SurrealValue::from(true));
    let count2 = Function::count(SurrealValue::None);
    let count3 = Function::count(Criteria::new_easy(10, 15, CriteriaSign::Gt).to_value());
    let count4 = Function::count(Criteria::new("age", 15, CriteriaSign::Gt).to_value());
    dbg!(count1);
    dbg!(count2);
    dbg!(count3);
    dbg!(count4);
    let select = SQLBuilderFactory::select()
        .column("country",None)
        .column(&Function::count(Criteria::new("age", 30, CriteriaSign::Gt).to_value()), Some("total"))
        .table("test")
        .group_by(vec!["country"])
        .build();
    dbg!(select);
    Ok(())
}

