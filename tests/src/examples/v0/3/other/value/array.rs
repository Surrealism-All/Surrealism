use surrealism::DefaultRes;
use surrealism::db::{SurrealValue, Object, AdapterToValue};
use serde::{Serialize, Deserialize};


#[derive(Debug, Clone, Serialize, Deserialize)]
struct User<'a> {
    name: &'a str,
    age: u8,
}

#[tokio::main]
async fn main() -> DefaultRes<()> {
    let arr1 = SurrealValue::array(vec![2,3,5,6]);
    let arr2 = SurrealValue::array(
        vec![
            User{name:"John",age:1},
            User{name:"Matt",age:13},
        ]
    );
    dbg!(arr1.to_string());
    dbg!(arr2.to_string());
    Ok(())
}