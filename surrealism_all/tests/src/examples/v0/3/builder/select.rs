use std::ops::DerefMut;
use surrealism::DefaultRes;
use surrealism::db::{Condition, ConditionSign, Criteria, CriteriaSign, Field, TimeUnit};
use surrealism::builder::{BaseWrapperImpl, ConditionImpl, SQLBuilderFactory, TableImpl, TimeoutImpl};
use surrealism::builder::select::SelectWrapperImpl;

// [tests\src\main.rs:54] select1 = "SELECT name AS username, address FROM person:tobie;"
// [tests\src\main.rs:55] select2 = "SELECT * FROM person password ,opts.security;"
// [tests\src\main.rs:56] select3 = "SELECT name FROM SurrealDB:great WHERE name != 'Mat' TIMEOUT 5m;"
// [tests\src\main.rs:57] select4 = "SELECT * FROM person WHERE email = 'tobie@surrealdb.com' EXPLAIN;"
// [tests\src\main.rs:58] select5 = "SELECT name FROM person WITH INDEX idx_name WHERE job = 'engineer' AND genre = 'm';"
#[tokio::main]
async fn main() -> DefaultRes<()> {
    let select1 = SQLBuilderFactory::select()
        .table("person")
        .id("tobie".into())
        .column("name", Some("username"))
        .column("address", None)
        .build();
    let select2 = SQLBuilderFactory::select()
        .table("person")
        .column("*", None)
        .omit(vec!["password", "opts.security"])
        .build();
    let select3 = SQLBuilderFactory::select()
        .column("name", None)
        .table("SurrealDB")
        .id("great".into())
        .where_condition(Condition::new().push(Criteria::new("name", "Mat", CriteriaSign::Neq), ConditionSign::None).deref_mut())
        .timeout(5, TimeUnit::MINUTE)
        .build();
    let select4 = SQLBuilderFactory::select()
        .table("person")
        .column("*", None)
        .where_condition(
            Condition::new().push(
                Criteria::new("email", "tobie@surrealdb.com", CriteriaSign::Eq)
                , ConditionSign::None,
            ).deref_mut()
        )
        .explain(false)
        .build();
    let select5 = SQLBuilderFactory::select()
        .table("person")
        .column("name", None)
        .with(true)
        .with_index("idx_name")
        .where_condition(
            Condition::new().push(
                Criteria::new("job", "engineer", CriteriaSign::Eq), ConditionSign::And,
            ).push(
                Criteria::new("genre", "m", CriteriaSign::Eq), ConditionSign::None,
            ).deref_mut()
        )
        .build();
    dbg!(select1);
    dbg!(select2);
    dbg!(select3);
    dbg!(select4);
    dbg!(select5);
    Ok(())
}