use surrealism::DefaultRes;
use surrealism::builder::{BaseWrapperImpl, SQLBuilderFactory, TableImpl};
use surrealism::builder::define::{FieldColumn, OnType, PwdType, TokenType, UniqueSearch};
use surrealism::builder::select::SelectWrapperImpl;
use surrealism::db::{Condition, ConditionSign, Criteria, CriteriaSign, Roles, TimeUnit, ValueConstructor, ValueType};

#[tokio::main]
async fn main() -> DefaultRes<()> {
    let define_table1 = SQLBuilderFactory::define().table()
        .name("reading")
        .build();
    let define_table2 = SQLBuilderFactory::define().table()
        .name("reading")
        .drop()
        .build();
    let define_table3 = SQLBuilderFactory::define().table()
        .name("reading")
        .changefeed(1, TimeUnit::DAY)
        .build();
    let define_table4 = SQLBuilderFactory::define().table()
        .schemafull()
        .build();
    let as_select = SQLBuilderFactory::select().table("reading").column("count()", Some("total")).group_by(vec!["city"]).to_string();
    let define_table5 = SQLBuilderFactory::define().table()
        .name("temperatures_by_month")
        .as_expression(&as_select)
        .build();

    Ok(())
}