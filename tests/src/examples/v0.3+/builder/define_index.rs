use surrealism::DefaultRes;
use surrealism::builder::{BaseWrapperImpl, SQLBuilderFactory, TableImpl};
use surrealism::builder::define::{FieldColumn, OnType, PwdType, TokenType, UniqueSearch};
use surrealism::builder::select::SelectWrapperImpl;
use surrealism::db::{Condition, ConditionSign, Criteria, CriteriaSign, Roles, TimeUnit, ValueConstructor, ValueType};

#[tokio::main]
async fn main() -> DefaultRes<()> {
    let define_index1 = SQLBuilderFactory::define().index()
        .name("userEmailIndex")
        .on("user")
        .field_column(FieldColumn::COLUMNS(vec!["email"]))
        .unique_search(UniqueSearch::Unique)
        .build();
    let define_index2 = SQLBuilderFactory::define().index()
        .name("userEmailIndex")
        .on("user")
        .field_column(FieldColumn::COLUMNS(vec!["name"]))
        .unique_search(UniqueSearch::search("ascii").push("", "").highlight().clone())
        .build();

    Ok(())
}