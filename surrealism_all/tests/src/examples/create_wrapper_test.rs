use surrealism::{SurrealismRes, SurrealID, TimeOut, SurrealValue, TimeUnit, ReturnType, Object};
use surrealism::builder::*;
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
struct User<'a> {
    name: &'a str,
    age: u32,
    works: Vec<&'a str>,
}

#[tokio::main]
async fn main() -> SurrealismRes<()> {
    // use set : CREATE surrealism:rand() SET name = 'Mat' TIMEOUT 5s RETURN AFTER PARALLEL;
    let mut create = SQLBuilderFactory::create()
        .table("surrealism")
        .id(SurrealID::RAND)
        .set()
        .add("name", SurrealValue::Str(String::from("Mat")))
        .timeout(TimeOut::new(5, TimeUnit::SECOND))
        .return_type(ReturnType::After)
        .parallel()
        .deref_mut();
    dbg!(&create.build());
    // use content : CREATE surrealdb:ulid() CONTENT { age : 16 , name : 'Mat' , works : ['cook'] } RETURN name;
    let user = User {
        name: "Mat",
        age: 16,
        works: vec!["cook"],
    };
    let mut create2 = SQLBuilderFactory::create()
        .table("surrealdb")
        .id(SurrealID::ULID)
        .content(Object::from_obj(&user))
        .return_type(ReturnType::Field("name"))
        .deref_mut();
    dbg!(create2.build());
    Ok(())
}