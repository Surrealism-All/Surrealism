use surrealism::{SurrealismRes, SurrealID, TimeOut, SurrealValue, TimeUnit, ReturnType, Object, DefaultInitService, InitService, UseNSDB, SurrealismCommit};
use surrealism::builder::*;
use serde::{Serialize, Deserialize};
use surrealism::builder::create::CreateWrapperImpl;

#[derive(Debug, Serialize, Deserialize)]
struct User<'a> {
    name: &'a str,
    age: u32,
    works: Vec<&'a str>,
}

#[tokio::main]
async fn main() -> SurrealismRes<()> {
    let service = DefaultInitService::new().init();
    let _ = service.use_commit("test", "test").await?;

    // use set : CREATE surrealism:10086 SET name = 'Mat' RETURN AFTER TIMEOUT 5s ;
    let mut create = SQLBuilderFactory::create()
        .table("surrealism")
        .id(SurrealID::Int(10086))
        .set()
        .add("name", "Mat")
        .timeout(TimeOut::new(5, TimeUnit::SECOND))
        .return_type(ReturnType::After)
        .deref_mut();
    // use content : CREATE surrealdb:10087 CONTENT { age : 16 , name : 'Mat' , works : ['cook'] } RETURN name;
    let user = User {
        name: "Mat",
        age: 16,
        works: vec!["cook"],
    };
    let mut create2 = SQLBuilderFactory::create()
        .table("surrealdb")
        .id(SurrealID::Int(10087))
        .content(&user)
        .return_type(ReturnType::Field("name"))
        .deref_mut();

    let res1 = service.commit_sql(&create.build()).await;
    dbg!(res1);
    let res2 = service.commit_sql(&create2.build()).await;
    dbg!(res2);
    Ok(())
}

