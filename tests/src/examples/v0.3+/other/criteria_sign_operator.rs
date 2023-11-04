use surrealism::builder::SQLBuilderFactory;
use surrealism::db::{Criteria, CriteriaSign};
use surrealism::DefaultRes;

// [tests\src\main.rs:13] operator1 = "SELECT * FROM 10 AND 20;"
// [tests\src\main.rs:14] operator2 = "SELECT * FROM 0 OR false;"
// [tests\src\main.rs:15] operator3 = "SELECT * FROM NULL ?? 0;"
// [tests\src\main.rs:16] operator4 = "SELECT * FROM true ?: 1;"
// [tests\src\main.rs:17] operator5 = "SELECT * FROM 'test text' ~ 'Test';"
#[tokio::main]
async fn main() -> DefaultRes<()> {
    let operator1 = CriteriaSign::And.build(10.into(), 20.into());
    let operator2 = CriteriaSign::Or.build(0.into(), false.into());
    let operator3 = CriteriaSign::Either.build("NULL".into(), 0.into());
    let operator4 = CriteriaSign::Both.build(true.into(), 1.into());
    let operator5 = CriteriaSign::Like.build("test text".into(), "Test".into());

    dbg!(operator1);
    dbg!(operator2);
    dbg!(operator3);
    dbg!(operator4);
    dbg!(operator5);
    Ok(())
}