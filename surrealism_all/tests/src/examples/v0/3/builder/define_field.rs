use std::ops::DerefMut;
use surrealism::builder::{BaseWrapperImpl, SQLBuilderFactory, TableImpl};
use surrealism::builder::define::{FieldColumn, OnType, Permissions, PwdType, Schema, TokenType};
use surrealism::builder::select::SelectWrapperImpl;
use surrealism::db::{AdapterToValue, Condition, ConditionSign, Criteria, CriteriaSign, Role, SurrealValue, TimeOut, TimeUnit, ValueConstructor, ValueType};
use surrealism::db::functions::Function;
use surrealism::DefaultRes;

// [tests\src\main.rs:19] define1 = "DEFINE FIELD metadata ON TABLE user FLEXIBLE TYPE object;"
#[tokio::main]
async fn main() -> DefaultRes<()> {
    let define1 = SQLBuilderFactory::define().field(
        "metadata","user",ValueConstructor::new(
            ValueType::Object,None,None,None,true
        ),None
    ).build();
    dbg!(define1);

    Ok(())
}