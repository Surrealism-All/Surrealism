use std::ops::DerefMut;
use surrealism::builder::{BaseWrapperImpl, SQLBuilderFactory, TableImpl};
use surrealism::builder::define::{FieldColumn, OnType, Permissions, PwdType, Schema, TokenType};
use surrealism::builder::select::SelectWrapperImpl;
use surrealism::db::{AdapterToValue, Condition, ConditionSign, Criteria, CriteriaSign, Role, SurrealValue, TimeOut, TimeUnit, ValueConstructor, ValueType};
use surrealism::db::functions::Function;
use surrealism::DefaultRes;

// [tests\src\main.rs:25] define1 = "DEFINE TABLE reading DROP;"
// [tests\src\main.rs:26] define2 = "DEFINE TABLE reading SCHEMAFULL;"
// [tests\src\main.rs:27] define3 = "DEFINE TABLE temperatures_by_month AS SELECT count() AS total FROM reading GROUP BY city;"
#[tokio::main]
async fn main() -> DefaultRes<()> {
    let define1 = SQLBuilderFactory::define().table(
        "reading",true,None,None,None
    ).build();
    let define2 = SQLBuilderFactory::define().table(
        "reading",false,Some(Schema::SCHEMAFULL),None,None
    ).build();
    let define3 = SQLBuilderFactory::define().table(
        "temperatures_by_month",
        false,
        None,
        Some(SQLBuilderFactory::select().table("reading").column(Function::count("".into()).as_str(),Some("total")).group_by(vec!["city"]).build_as_child().as_str()),
        None
    ).build();
    dbg!(define1);
    dbg!(define2);
    dbg!(define3);
    Ok(())
}