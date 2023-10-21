use surrealism::DefaultRes;
use surrealism::db::{SurrealValue};

//[tests\src\main.rs:14] s1 = String(
//     "Lorem ipsum dolor sit amet",
// )
// [tests\src\main.rs:15] s2 = String(
//     "I ❤\u{fe0f} SurrealDB",
// )
// [tests\src\main.rs:16] s3 = Int(
//     89,
// )
// [tests\src\main.rs:17] s4 = Object(
//     Object(
//         {
//             "address": String(
//                 "China - Shanghai",
//             ),
//         },
//     ),
// )
// ------------------------------------------------
//[tests\src\main.rs:14] s1 = String(
//     "Lorem ipsum dolor sit amet",
// )
// [tests\src\main.rs:15] s2.to_string() = "'I ❤\u{fe0f} SurrealDB'"
// [tests\src\main.rs:16] s3.to_string() = "89"
// [tests\src\main.rs:17] s4.to_string() = "{ address : 'China - Shanghai' }"
#[tokio::main]
async fn main() -> DefaultRes<()> {
    // 请注意使用string方法创建的只可能是字符串
    // 而是用from或from_json方法创建的可能会进行隐式转换
    let s1 = SurrealValue::string("Lorem ipsum dolor sit amet");
    let s2 = SurrealValue::from("I ❤️ SurrealDB");
    //这里则会隐式转换为Int
    let s3 =  SurrealValue::from("89");
    // 这里会隐式转换为Object
    let s4 = SurrealValue::from_json("{ \"address\": \"China - Shanghai\"}");
    dbg!(s1);
    dbg!(s2.to_string());
    dbg!(s3.to_string());
    dbg!(s4.to_string());
    Ok(())
}