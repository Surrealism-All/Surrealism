use surrealism::{SurrealismRes, SurrealID, SurrealValue, Condition, Criteria, CriteriaSign, ConditionSign, TimeUnit, Order};
use surrealism::builder::*;
use surrealism::builder::select::SelectWrapperImpl;
use surrealism::functions::Function;


// [tests\src\main.rs:18] live_select1 = "LIVE SELECT * FROM person;"
// [tests\src\main.rs:24] live_select2 = "LIVE SELECT DIFF FROM person;"
// [tests\src\main.rs:33] live_select3 = "LIVE SELECT * FROM person WHERE age > 18;"
#[tokio::main]
async fn main() -> SurrealismRes<()> {
    let live_select1 = SQLBuilderFactory::select()
        .column("*",None)
        .table("person")
        .to_live()
        .build();
    dbg!(live_select1);
    let live_select2 = SQLBuilderFactory::select()
        .column("DIFF",None)
        .table("person")
        .to_live()
        .build();
    dbg!(live_select2);
    let live_select3 = SQLBuilderFactory::select()
        .column("*",None)
        .table("person")
        .where_condition(
            Condition::new().push(Criteria::new("age",18,CriteriaSign::Gt),ConditionSign::None).deref_mut()
        )
        .to_live()
        .build();
    dbg!(live_select3);
    Ok(())
}
