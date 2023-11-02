use surrealism::DefaultRes;
use surrealism::builder::{BaseWrapperImpl, SQLBuilderFactory, TableImpl};
use surrealism::builder::define::{FieldColumn, OnType, PwdType, TokenType, UniqueSearch};
use surrealism::builder::select::SelectWrapperImpl;
use surrealism::db::{Condition, ConditionSign, Criteria, CriteriaSign, Roles, TimeUnit, ValueConstructor, ValueType};

#[tokio::main]
async fn main() -> DefaultRes<()> {
    let define_scope = SQLBuilderFactory::define().scope()
        .name("account")
        .session(24, TimeUnit::HOUR)
        .sign_up("CREATE user SET email = $email, pass = crypto::argon2::generate($pass)")
        .sign_in("SELECT * FROM user WHERE email = $email AND crypto::argon2::compare(pass, $pass)")
        .build();

    Ok(())
}