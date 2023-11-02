use surrealism::DefaultRes;
use surrealism::builder::{BaseWrapperImpl, SQLBuilderFactory, TableImpl};
use surrealism::builder::define::{FieldColumn, OnType, PwdType, TokenType, UniqueSearch};
use surrealism::builder::select::SelectWrapperImpl;
use surrealism::db::{Condition, ConditionSign, Criteria, CriteriaSign, Roles, TimeUnit, ValueConstructor, ValueType};

#[tokio::main]
async fn main() -> DefaultRes<()> {
    let define_analyzer1 = SQLBuilderFactory::define().analyzer()
        .name("example_camel")
        .tokenizer("camel")
        .build();
    let define_analyzer2 = SQLBuilderFactory::define().analyzer()
        .name("code")
        .tokenizer("class")
        .tokenizer("camel")
        .filter("lowercase")
        .filter("ascii")
        .build();
    Ok(())
}