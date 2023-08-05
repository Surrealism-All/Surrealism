use surrealism::{SurrealismRes, IDNumber, SurrealID};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct User<'a> {
    name: &'a str,
    age: u32,
}

// [tests\src\main.rs:20] sn1 = ""
// [tests\src\main.rs:21] sn2 = "Joe"
// [tests\src\main.rs:22] sn3 = Array(
//     [
//         User {
//             name: "Joe",
//             age: 16,
//         },
//         User {
//             name: "Mark",
//             age: 25,
//         },
//     ],
// )
// [tests\src\main.rs:23] sn4 = "23.56546"
// [tests\src\main.rs:24] sn5 = Object(
//     User {
//         name: "Mary",
//         age: 23,
//     },
// )
// 56
#[tokio::main]
async fn main() -> SurrealismRes<()> {
    let n1 = IDNumber::Int(56).to_str();
    let sn1 = SurrealID::<String>::Default.to_str();
    let sn2 = SurrealID::<String>::Str("Joe".to_string()).to_str();
    let sn3 = SurrealID::<User>::Array(vec![User { name: "Joe", age: 16 }, User { name: "Mark", age: 25 }]);
    let sn4 = SurrealID::<f32>::Number(IDNumber::Float(23.56546_f32)).to_str();
    let sn5 = SurrealID::<User>::Object(User { name: "Mary", age: 23 });
    let sn6 =  SurrealID::<String>::UUID;
    dbg!(sn1);
    dbg!(sn2);
    dbg!(sn3);
    dbg!(sn4);
    dbg!(sn5);
    dbg!(sn6);
    println!("{}", &n1);
    Ok(())
}
