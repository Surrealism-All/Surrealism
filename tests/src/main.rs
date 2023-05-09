use surrealdb::engine::remote::ws::Client;
use surrealdb::Surreal;
use surrealism::{SurrealRes, InitServiceImpl, SurrealDB, UseWrapper,Wrapper};
use serde::{Serialize, Deserialize};
use surrealdb::sql::Thing;
use lazy_static::lazy_static;


// lazy_static! {
//     static ref DB: Surreal<Client> = InitServiceImpl::new().init().unwrap();
// }
#[derive(Debug, Serialize, Deserialize)]
struct User {
    id: String,
    name: String,
    email: String,
}

#[derive(Debug, Deserialize)]
struct Record {
    #[allow(dead_code)]
    id: Thing,
}

#[tokio::main]
async fn main() -> SurrealRes<()> {
    let db: SurrealDB = InitServiceImpl::new().init().unwrap();

    let mut stmt = UseWrapper::new();

    stmt.use_ns("test").and().use_db("test").build();

    println!("{}", stmt.commit());
    db.use_commit(stmt).await;

    // let create: Record = DB.create(("user","2")).content(
    //     User {
    //         id: "1".to_string(),
    //         name: "user1".to_string(),
    //         email: "syf@example.com".to_string(),
    //     }
    // ).await?;
    // dbg!(create);
    // let user:Option<Record>= DB.select(("user","1")).await?;
    // dbg!(user.unwrap());
    let res = db.core.cn.query("SELECT * FROM type::table($table)")
        .bind(("table", "user")).await?;
    dbg!(res);

    Ok(())
}

//[default]
//surreal = "Single"
//username = "root"
//password = "syf20020816"
//url = "127.0.0.1"
//port = "10086"
//mode = "Memory"
//path = "E:/Rust/surreal"