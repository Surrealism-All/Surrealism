use surrealism::{Condition, ConditionSign, Criteria, CriteriaSign, SurrealismRes, SurrealValue, TimeOut, TimeUnit, ValueConstructor, ValueType};
use surrealism::builder::*;
use surrealism::builder::define::{FieldColumn, OnType, Permissions, PwdType, Schema, TokenType};

#[tokio::main]
async fn main() -> SurrealismRes<()> {
    let define_field1 = SQLBuilderFactory::define()
        .field("email", "user", ValueConstructor::new(ValueType::String, None, None), None).build();
    let define_field2 = SQLBuilderFactory::define()
        .field("countrycode", "user",
               ValueConstructor::new(ValueType::String,
                                     Some(SurrealValue::from("GBR")),
                                     Some(Condition::new()
                                         .push(Criteria::new_field("NONE", CriteriaSign::Neq), ConditionSign::And)
                                         .push(Criteria::new_field("/[A-Z]{3}/", CriteriaSign::Eq), ConditionSign::None).deref_mut())),
               None)
        .build();
    // use infer
    let define_field3 = SQLBuilderFactory::define()
        .field("countrycode", "user",
               ValueConstructor::new_infer(Some(SurrealValue::from(true)),None),None).build();
    let define_field4 = SQLBuilderFactory::define()
        .field("email", "user", ValueConstructor::new(ValueType::Option(Box::new(ValueType::String)), None, None), None).build();
    dbg!(define_field1);
    dbg!(define_field2);
    dbg!(define_field3);
    dbg!(define_field4);
    Ok(())
}