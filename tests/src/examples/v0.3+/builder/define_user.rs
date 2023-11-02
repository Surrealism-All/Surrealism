use surrealism::DefaultRes;
use surrealism::builder::{BaseWrapperImpl, SQLBuilderFactory, TableImpl};
use surrealism::builder::define::{FieldColumn, OnType, PwdType, TokenType, UniqueSearch};
use surrealism::builder::select::SelectWrapperImpl;
use surrealism::db::{Condition, ConditionSign, Criteria, CriteriaSign, Roles, TimeUnit, ValueConstructor, ValueType};

#[tokio::main]
async fn main() -> DefaultRes<()> {
    let define_user = SQLBuilderFactory::define()
        .user()
        .name("username")
        .on(OnType::ROOT)
        .pwd(PwdType::Pwd("123456"))
        .role(Roles::OWNER)
        .build();
    Ok(())
}