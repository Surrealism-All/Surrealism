use std::ops::DerefMut;
use surrealism::builder::s_use::UseWrapperImpl;
use surrealism::DefaultRes;
use surrealism::builder::{BaseWrapperImpl, ConditionImpl, SQLBuilderFactory, TableImpl, TimeoutImpl};
use surrealism::db::{Condition, ConditionSign, Criteria, CriteriaSign, Operator, SurrealID, TimeUnit};
use surrealism::builder::insert::InsertWrapperImpl;

// [tests\src\main.rs:23] delete1 = "DELETE person:100;"
// [tests\src\main.rs:24] delete2 = "DELETE city:USA WHERE name CONTAINS 'Los Angeles';"
#[tokio::main]
async fn main() -> DefaultRes<()> {
    let delete1 = SQLBuilderFactory::delete()
        .table("person")
        .id(100.into())
        .build();
    let delete2 = SQLBuilderFactory::delete()
        .table("city")
        .id(SurrealID::from("USA"))
        .where_condition(
            Condition::new().push(
                Criteria::new("name", "Los Angeles", CriteriaSign::Contains), ConditionSign::None,
            ).deref_mut()
        )
        .build();
    let delete3 = SQLBuilderFactory::delete()
        .table("person")
        .where_condition(
            Condition::new()
                .push(Criteria::new("knows->person","(knows WHERE influencer = false)",CriteriaSign::Link),ConditionSign::None)
                .deref_mut()
        )
        .timeout(5,TimeUnit::SECOND)
        .build();
    dbg!(delete1);
    dbg!(delete2);
    dbg!(delete3);
    Ok(())
}