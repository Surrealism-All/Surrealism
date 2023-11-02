use surrealism::DefaultRes;
use surrealism::builder::{BaseWrapperImpl, SQLBuilderFactory, TableImpl};
use surrealism::builder::define::{FieldColumn, OnType, PwdType, TokenType, UniqueSearch};
use surrealism::builder::select::SelectWrapperImpl;
use surrealism::db::{Condition, ConditionSign, Criteria, CriteriaSign, Roles, TimeUnit, ValueConstructor, ValueType};

#[tokio::main]
async fn main() -> DefaultRes<()> {

    let define_func = SQLBuilderFactory::define().function()
        .name("great")
        .args(vec!["$name:string"])
        .returned("'Hello, '+$name + '!'")
        .build();

    Ok(())
}