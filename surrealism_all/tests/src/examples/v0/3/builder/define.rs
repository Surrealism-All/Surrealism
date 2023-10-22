use std::ops::DerefMut;
use surrealism::builder::{BaseWrapperImpl, SQLBuilderFactory, TableImpl};
use surrealism::builder::define::{FieldColumn, OnType, Permissions, PwdType, Schema, TokenType};
use surrealism::builder::select::SelectWrapperImpl;
use surrealism::db::{AdapterToValue, Condition, ConditionSign, Criteria, CriteriaSign, Role, SurrealValue, TimeOut, TimeUnit, ValueConstructor, ValueType};
use surrealism::db::functions::Function;
use surrealism::DefaultRes;

// [tests\src\main.rs:43] define1 = "DEFINE FIELD email ON TABLE user TYPE string;"
// [tests\src\main.rs:44] define2 = "DEFINE FIELD metadata ON TABLE user FLEXIBLE TYPE object;"
// [tests\src\main.rs:45] define3 = "DEFINE FIELD roles.* ON TABLE user TYPE array;"
// [tests\src\main.rs:46] define4 = "DEFINE FIELD biography ON TABLE user TYPE option<string>;"
// [tests\src\main.rs:47] define5 = "DEFINE FIELD locked ON TABLE user TYPE bool DEFAULT false;"
// [tests\src\main.rs:48] define6 = "DEFINE FIELD email ON TABLE user TYPE string VALUE 'string::lowercase($value)';"
// [tests\src\main.rs:49] define7 = "DEFINE FIELD countrycode ON TABLE user TYPE string DEFAULT 'GBR' VALUE 'STR' ASSERT $value = '/[A-Z]{3}/';"
#[tokio::main]
async fn main() -> DefaultRes<()> {
    let define1 = SQLBuilderFactory::define().field(
        "email","user",ValueConstructor::new(ValueType::String,None,None,None,false),None
    ).build();
    let define2 = SQLBuilderFactory::define().field(
        "metadata","user",ValueConstructor::new(
            ValueType::Object,None,None,None,true
        ),None
    ).build();
    let define3 = SQLBuilderFactory::define().field(
        "roles.*","user",ValueConstructor::new(ValueType::Array,None,None,None,false),None
    ).build();
    let define4 = SQLBuilderFactory::define().field(
        "biography","user",ValueConstructor::new(ValueType::option(ValueType::String),None,None,None,false),None
    ).build();
    let define5 = SQLBuilderFactory::define().field(
        "locked","user",ValueConstructor::new(ValueType::Bool,Some(false.into()),None,None,false),None
    ).build();
    let define6 = SQLBuilderFactory::define().field(
        "email","user",ValueConstructor::new(ValueType::String,None,Some("string::lowercase($value)".into()),None,false),None
    ).build();
    let define7 = SQLBuilderFactory::define().field(
        "countrycode","user",ValueConstructor::new(
            ValueType::String,Some("GBR".into()),Some("STR".into()),
            Some(Condition::new().push(Criteria::new_field("/[A-Z]{3}/",CriteriaSign::Eq),ConditionSign::None).deref_mut()),
            false
        ),None
    ).build();
    dbg!(define1);
    dbg!(define2);
    dbg!(define3);
    dbg!(define4);
    dbg!(define5);
    dbg!(define6);
    dbg!(define7);
    Ok(())
}