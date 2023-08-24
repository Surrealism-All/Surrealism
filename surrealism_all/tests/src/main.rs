use std::collections::HashMap;
use serde::{Serialize, Deserialize};
use serde_json::Value;
use serde_json::json;
use surrealism::{ParamCombine, SurrealismRes, ContentSet, SurrealValue, Array, Object, Patch};

#[derive(Debug, Serialize, Deserialize)]
struct User<'a> {
    name: &'a str,
    age: u32,
    works: Vec<&'a str>,
}

#[tokio::main]
async fn main() -> SurrealismRes<()> {
    // let mut map: HashMap<&str, SurrealValue> = HashMap::new();
    // let _ = map.insert("name", SurrealValue::Str(String::from("Mat")));
    // let _ = map.insert("age", SurrealValue::Int(16));
    // let _ = map.insert("address", SurrealValue::from("China - Shanghai"));
    // let _ = map.insert("male", SurrealValue::Bool(true));
    // let mut c_set1 = ContentSet::new_set(map);
    // let mut arr = Array::new();
    // let _ = arr.push(SurrealValue::Str(String::from("cook")))
    //     .push(SurrealValue::Str("author".to_string()));
    // c_set1.add("works", SurrealValue::Array(arr));
    // // dbg!(&c_set1.set());
    // dbg!(c_set1.build_to_merge());
    // let user = User { name: "Mat", age: 20, works: vec!["lawyer", "worker"] };
    // let content = ContentSet::new_content(Object::from_obj(&user));
    // dbg!(content.build_to_merge());
    // let p = Patch::add("/name", "John");
    // dbg!(&p);
    // dbg!(p.build());
    let p = Patch::move_path("/user", "/name");
    dbg!(p.combine());
    Ok(())
}