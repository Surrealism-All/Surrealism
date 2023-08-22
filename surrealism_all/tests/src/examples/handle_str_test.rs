//! handle_str内部测试 do not use !!!
//! ```txt
//! @author:syf20020816@Outlook.com
//! @date:2023/8/23
//! @version:0.0.1
//! @description:
//! ```

use surrealism::{SurrealismRes, SurrealID, handle_str};
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
struct User<'a> {
    name: &'a str,
    age: u32,
}

#[tokio::main]
async fn main() -> SurrealismRes<()>{
    let user1 = User { name: "Mark", age: 16 };
    let user2 = User { name: "Joe", age: 21 };
    let user_str1 = serde_json::to_string(&user1).unwrap();
    let handle_user1 = handle_str(&user_str1);
    let vec1 = vec![user1, user2];
    let handle_user2 = handle_str(serde_json::to_string(&vec1).unwrap().as_str());
    dbg!(&user_str1);
    dbg!(&handle_user1);
    dbg!(&handle_user2);
    Ok(())
}

