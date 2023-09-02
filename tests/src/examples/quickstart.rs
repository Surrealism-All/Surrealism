use surrealism::{DefaultInitService, InitService, SurrealID, SurrealismCommit, SurrealismConnector, SurrealismRes, Table, UseNSDB, parse_response};
use surrealism::builder::{BaseWrapperImpl, SQLBuilderFactory, TableImpl};
use surrealism::builder::create::{CreateWrapper, CreateWrapperImpl};
use serde::{Serialize, Deserialize};
use surrealism::builder::select::SelectWrapperImpl;

#[derive(Debug, Clone, Serialize, Deserialize)]
struct User {
    username: String,
    pwd:String,
    male: bool,
    age: u8,
}

/// create a new user table
/// table_name:user
/// table_id:surrealism
pub fn crate_user_table() -> CreateWrapper {
    // create a user data
    let user = User {
        username: "Tobie".to_string(),
        pwd: "Tobie001".to_string(),
        male: true,
        age: 23,
    };
    // create table with content
    let user_table = SQLBuilderFactory::create()
        .table("user")
        .id(SurrealID::from("surrealism"))
        .content(&user)
        .deref_mut();
    user_table
}

#[tokio::main]
async fn main() -> SurrealismRes<()> {
    // init service
    let mut service = DefaultInitService::new().init();
    // use ns:test and db:test
    let _ = service.use_commit("test", "test").await?;
    // get info from surrealdb
    // let info = SQLBuilderFactory::info().db().build();
    // let info_res = service.commit_sql(&info).await?;
    // dbg!(info_res);
    // create a table
    // let create_stmt = crate_user_table().build();
    // let create_res = service.commit_sql(&create_stmt).await?;
    // dbg!(create_res);
    // select user::surrealism table
    let select = SQLBuilderFactory::select().table("user").id(SurrealID::from("surrealism")).column("*").build();
    let select_res = service.commit_sql(&select).await?;
    //parse response to any type you want
    let res: User = parse_response(select_res);
    // [tests\src\main.rs:55] res = User {
    //     username: "Tobie",
    //     pwd: "Tobie001",
    //     male: true,
    //     age: 23,
    // }
    dbg!(&res);
    Ok(())
}

