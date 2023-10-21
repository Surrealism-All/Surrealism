use surrealism::DefaultRes;
use surrealism::db::{SurrealValue, Object};
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
struct User<'a> {
    name: &'a str,
    age: u8,
}

impl<'a> User<'a> {
    pub fn new() -> Self { User { name: "Matt", age: 16 } }
}

// [tests\src\main.rs:20] obj1.to_string() = "{ age : 16 , name : 'Matt' }"
// [tests\src\main.rs:21] obj2.to_string() = "{ age : 16 , name : 'Matt' }"
#[tokio::main]
async fn main() -> DefaultRes<()> {
    let user = User::new();
    let obj1 = SurrealValue::object(&user);
    let obj2 = SurrealValue::from(Object::from_obj(&user));
    dbg!(obj1.to_string());
    dbg!(obj2.to_string());
    Ok(())
}