use surrealism::DefaultRes;
use surrealism::builder::{BaseWrapperImpl, SQLBuilderFactory, TableImpl};
use surrealism::builder::define::{FieldColumn, OnType, PwdType, TokenType, UniqueSearch};
use surrealism::builder::select::SelectWrapperImpl;
use surrealism::db::{Condition, ConditionSign, Criteria, CriteriaSign, Roles, TimeUnit, ValueConstructor, ValueType};

#[tokio::main]
async fn main() -> DefaultRes<()> {
    let define_field1 = SQLBuilderFactory::define().field()
        .name("email")
        .on("user")
        .value(ValueConstructor::new(ValueType::String, None, None, None, false))
        .build();
    let define_field2 = SQLBuilderFactory::define().field()
        .name("locked")
        .on("user")
        .value(ValueConstructor::new(ValueType::Bool, Some(true.into()), Some(true.into()), None, true))
        .build();

    Ok(())
}