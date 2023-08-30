use std::collections::HashMap;
use surrealism::{SurrealismRes, SurrealID, Array, Object, SurrealValue};
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
struct User<'a> {
    name: &'a str,
    age: u32,
}

#[tokio::main]
async fn main() -> SurrealismRes<()> {
    let mut id1 = Array::new();
    let _ = id1.push(SurrealValue::Int(1));
    let _ = id1.push(SurrealValue::None);
    let _ = id1.push(SurrealValue::Bool(true));
    let ss = id1.parse();
    dbg!(ss);
    let user = User{name:"Joe",age:12};
    let mut item = HashMap::new();
    item.insert("a".to_string(), SurrealValue::Array(id1));
    item.insert("b".to_string(), SurrealValue::Int(2));
    let mut id2 = Object::from(item);
    dbg!(id2.parse());
    let res = Object::from_obj(&user);
    dbg!(res);
    Ok(())
}