use surrealism::DefaultRes;
use surrealism::builder::{BaseWrapperImpl, SQLBuilderFactory, TableImpl};
use surrealism::builder::define::{FieldColumn, OnType, PwdType, TokenType, UniqueSearch};
use surrealism::builder::select::SelectWrapperImpl;
use surrealism::db::{Condition, ConditionSign, Criteria, CriteriaSign, Roles, TimeUnit, ValueConstructor, ValueType};

#[tokio::main]
async fn main() -> DefaultRes<()> {
    let define_event = SQLBuilderFactory::define().event()
        .on("user")
        .when(Condition::new().push(Criteria::new("1@mail.com", "2@mail.com", CriteriaSign::Neq), ConditionSign::None).deref_mut())
        .then("CREATE event SET user = $value.id, time = time::now(), value = $after.email, action = 'email_changed'")
        .build();
    Ok(())
}