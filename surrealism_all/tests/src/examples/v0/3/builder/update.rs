use surrealism::db::{ SurrealID, TimeOut, SurrealValue, TimeUnit, ReturnType, Object,  Operator, Condition, Criteria, CriteriaSign, ConditionSign, Patch};
use surrealism::builder::*;
use serde::{Serialize, Deserialize};
use surrealism::builder::update::UpdateWrapperImpl;
use surrealism::DefaultRes;

#[derive(Debug, Serialize, Deserialize)]
struct Person<'a> {
    name: &'a str,
    company: &'a str,
    skills: Vec<&'a str>,
}

// [tests\src\main.rs:30] update1.build() = "UPDATE person:100 SET name = 'Tobie' , company = 'SurrealDB' , skills = ['Rust', 'Go', 'JS'];"
// [tests\src\main.rs:42] update2.build() = "UPDATE city SET population = 954100 , interests -= 'Java' WHERE name = 'London';"
// [tests\src\main.rs:53] update3.build() = "UPDATE person CONTENT { company : 'SurrealDB' , name : 'Tobie' , skills : ['Rust', 'Go', 'JS'] };"
// [tests\src\main.rs:60] update4.build() = "UPDATE person:tobie MERGE settings:{ company : 'SurrealDB' , name : 'Tobie' , skills : ['Rust', 'Go', 'JS'] };"
// [tests\src\main.rs:67] update5.build() = "UPDATE person:tobie PATCH [ {\"op\":\"add\",\"path\":\"Engineering\",\"value\":\"true\"} ];"
// [tests\src\main.rs:79] update6.build() = "UPDATE person SET important = true WHERE -> knows -> person -> (knows WHERE influencer = true) TIMEOUT 5s;"
#[tokio::main]
async fn main() -> DefaultRes<()> {
    let mut update1 = SQLBuilderFactory::update()
        .table("person")
        .id(SurrealID::Int(100))
        .set()
        .add("name", "Tobie", Operator::Eq)
        .add("company", "SurrealDB", Operator::Eq)
        .add("skills", vec!["Rust", "Go", "JS"], Operator::Eq)
        .deref_mut();
    dbg!(update1.build());

    let mut update2 = SQLBuilderFactory::update()
        .table("city")
        .id(SurrealID::Default)
        .set()
        .add("population", 954100, Operator::Eq)
        .add("interests", "Java", Operator::Minus)
        .where_condition(Condition::new()
            .push(Criteria::new("name", "London", CriteriaSign::Eq), ConditionSign::None)
            .deref_mut())
        .deref_mut();
    dbg!(update2.build());

    let person = Person {
        name: "Tobie",
        company: "SurrealDB",
        skills: vec!["Rust", "Go", "JS"],
    };
    let mut update3 = SQLBuilderFactory::update()
        .table("person")
        .content(&person)
        .deref_mut();
    dbg!(update3.build());

    let mut update4 = SQLBuilderFactory::update()
        .table("person")
        .id("tobie".into())
        .merge(&person)
        .deref_mut();
    dbg!(update4.build());

    let mut update5 = SQLBuilderFactory::update()
        .table("person")
        .id(SurrealID::from("tobie"))
        .patch(Patch::add("Engineering", true))
        .deref_mut();
    dbg!(update5.build());

    let mut update6 = SQLBuilderFactory::update()
        .table("person")
        .set()
        .add("important", true, Operator::Eq)
        .where_condition(Condition::new()
            .push(Criteria::new("knows", "person", CriteriaSign::Link), ConditionSign::Link)
            .push(Criteria::cheat("knows", "influencer = true", "WHERE"), ConditionSign::None)
            .deref_mut())
        .timeout(5, TimeUnit::SECOND)
        .deref_mut();
    dbg!(update6.build());
    Ok(())
}

