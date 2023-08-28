use surrealism::{SurrealismRes, SurrealID, SurrealValue, Condition, Criteria, CriteriaSign, ConditionSign, TimeUnit, Order};
use surrealism::builder::*;
use surrealism::builder::select::SelectWrapperImpl;
use surrealism::functions::Function;


// [tests\src\main.rs:22] select1 = "SELECT name FROM SurrealDB:great GROUP BY id;"
// [tests\src\main.rs:30] select2 = "SELECT name FROM SurrealDB:great WHERE name != 'Mat' TIMEOUT 5m;"
// [tests\src\main.rs:36] select3 = "SELECT * FROM article ORDER BY title , des ASC;"
// [tests\src\main.rs:42] select4 = "SELECT * FROM person LIMIT 50;"
#[tokio::main]
async fn main() -> SurrealismRes<()> {
    let select1 = SQLBuilderFactory::select()
        .column("name")
        .table("SurrealDB")
        .id(SurrealID::from("great"))
        .group_by(vec!["id"])
        .build();
    dbg!(select1);
    let select2 = SQLBuilderFactory::select()
        .column("name")
        .table("SurrealDB")
        .id(SurrealID::from("great"))
        .where_condition(Condition::new().push(Criteria::new("name", "Mat", CriteriaSign::Neq), ConditionSign::None).deref_mut())
        .timeout(5, TimeUnit::MINUTE)
        .build();
    dbg!(select2);
    let select3 = SQLBuilderFactory::select()
        .column("*")
        .table("article")
        .order_by(Order::new_asc(vec!["title", "des"]))
        .build();
    dbg!(select3);
    let select4 = SQLBuilderFactory::select()
        .column("*")
        .table("person")
        .limit(50)
        .build();
    dbg!(select4);
    Ok(())
}

