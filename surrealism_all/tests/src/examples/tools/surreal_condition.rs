use surrealism::{SurrealismRes, Condition, SurrealValue, ParamCombine, Criteria, CriteriaSign, ConditionSign};

#[tokio::main]
async fn main() -> SurrealismRes<()> {
    // WHERE username = 'Mat' AND age != 16
    let condition = Condition::new()
        .push(Criteria::new("username", SurrealValue::Str(String::from("Mat")), CriteriaSign::Eq), ConditionSign::And)
        .push(Criteria::new("age", SurrealValue::Int(16), CriteriaSign::Neq), ConditionSign::None)
        .deref_mut();
    dbg!(condition.combine());
    // WHERE -> knows -> person -> (knows WHERE influencer = true)
    let link = Condition::new()
        .push(Criteria::new("knows", SurrealValue::from("person"), CriteriaSign::Link), ConditionSign::Link)
        .push(Criteria::cheat("knows","influencer = true","WHERE"),ConditionSign::None)
        .deref_mut();
    dbg!(link.combine());
    Ok(())
}