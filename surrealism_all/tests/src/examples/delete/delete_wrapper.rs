use surrealism::{SurrealismRes, SurrealID, ConditionSign, Condition, Criteria, CriteriaSign};
use surrealism::builder::*;
use surrealism::builder::delete::DeleteWrapperImpl;


#[tokio::main]
async fn main() -> SurrealismRes<()> {
    // DELETE city:USA WHERE name = 'Los Angeles';
    let mut delete1 = SQLBuilderFactory::delete()
        .table("city")
        .id(SurrealID::from("USA"))
        .where_condition(Condition::new().push(Criteria::new("name","Los Angeles",CriteriaSign::Eq),ConditionSign::None).deref_mut())
        .deref_mut();
    dbg!(delete1.build());
    Ok(())
}

