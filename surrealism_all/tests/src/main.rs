mod lib;

use surrealism::{SurrealismRes, ConditionUnit, Condition, CompareSign, ValueUnit, SurrealValue, ParamCombine};
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
    // let mut condition1 = Condition::new_no_args();
    // let _ = condition1.left_from_str("username");
    // dbg!(condition1);
    // let condition2 = Condition::new(ConditionUnit::from("age"), ValueUnit::Value(SurrealValue::Int(16)), CompareSign::Neq);
    // dbg!(condition2);
    // let mut condition3 = Condition::new_no_args();
    // let _ = condition3.left_from_vec(vec!["user", "person"]);
    // let _ = condition3.add_to_left("job");
    // dbg!(condition3);


    dbg!(u1);
    dbg!(u2);
    dbg!(u3);
    Ok(())
}



