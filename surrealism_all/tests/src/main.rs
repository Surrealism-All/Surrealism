use surrealism::DefaultRes;
use surrealism::db::{Range, SurrealID, Array, SurrealValue, Object};
use serde::{Serialize,Deserialize};

#[derive(Debug,Serialize,Deserialize)]
struct User<'a>{
    name: &'a str,
    age: u8,
}

// [tests\src\main.rs:27] id1.to_string() = "rand()"
// [tests\src\main.rs:28] id2.to_string() = ""
// [tests\src\main.rs:29] id3.to_string() = "surrealism"
// [tests\src\main.rs:30] id4.to_string() = "56"
// [tests\src\main.rs:31] id5.to_string() = "45.5454647"
// [tests\src\main.rs:32] id6.to_string() = "['John', 'Matt']"
// [tests\src\main.rs:33] id7.to_string() = "{ age : 16 , name : 'Mat' }"
// [tests\src\main.rs:34] id8.to_string() = "'2'..='6'"
// [tests\src\main.rs:35] id9.to_string() = "ulid()"
#[tokio::main]
async fn main() -> DefaultRes<()> {
    let id1 = SurrealID::RAND;
    let id2 = SurrealID::Default;
    let id3 = SurrealID::from("surrealism");
    let id4 = SurrealID::Int(56_i64);
    let id5 = SurrealID::Float(45.5454647_f64);
    let id6 = SurrealID::Array(Array::from(vec!["John".into(), "Matt".into()]));
    let user = User {
        name: "Mat",
        age: 16,
    };
    let id7 = SurrealID::Object(Object::from_obj(&user));
    let id8 = SurrealID::Range(Range::new_from_str("2", "6", true));
    let id9 = SurrealID::from("ulid()");
    dbg!(id1.to_string());
    dbg!(id2.to_string());
    dbg!(id3.to_string());
    dbg!(id4.to_string());
    dbg!(id5.to_string());
    dbg!(id6.to_string());
    dbg!(id7.to_string());
    dbg!(id8.to_string());
    dbg!(id9.to_string());
    Ok(())
}