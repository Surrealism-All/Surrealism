use surrealism::{SurrealismRes, SurrealID, Array, SurrealValue, Object, Range, ParamCombine, Table};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct User<'a> {
    name: &'a str,
    age: u32,
}

#[tokio::main]
async fn main() -> SurrealismRes<()> {
    let id1 = SurrealID::RAND;
    let id2 = SurrealID::Default;
    let id3 = SurrealID::Str(String::from("surrealism"));
    let id4 = SurrealID::Int(56_i32);
    let id5 = SurrealID::Float(45.5454647_f32);
    let id6 = SurrealID::Array(Array::from(vec![SurrealValue::Str(String::from("John")), SurrealValue::Str(String::from("Mat"))]));
    let user = User {
        name: "Mat",
        age: 16,
    };
    let id7 = SurrealID::Object(Object::from_obj(&user));
    let id8 = SurrealID::Range(Range::new_from_str("2", "6", true));
    let id9 = SurrealID::from("ulid()");

    let table1 = Table::new_no_arg()
        .table("surrealism")
        .id(id1)
        .build();

    let table2 = Table::new_no_arg()
        .table("surrealism")
        .id(id4)
        .build();
    let table3 = Table::new("surrealism", id6).combine();
    let table4 = Table::new_into("surrealdb", "2..6").combine();
    dbg!(&table1);
    dbg!(&table2);
    dbg!(&table3);
    dbg!(&table4);
    Ok(())
}
