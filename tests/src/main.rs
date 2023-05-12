use surrealdb::engine::remote::ws::Client;
use surrealdb::Surreal;
use surrealism::{SurrealRes, InitServiceImpl, SurrealDB, UseWrapper, Wrapper, TableId, IdFunction, SQLParser, ParseSQL};
use serde::{Serialize, Deserialize};
use surrealdb::sql::Thing;
use lazy_static::lazy_static;


// lazy_static! {
//     static ref DB: Surreal<Client> = InitServiceImpl::new().init().unwrap();
// }
#[derive(Debug, Serialize, Deserialize, ParseSQL)]
struct User {
    pub userId: String,
    pub name: String,
    pub email: String,
}

#[derive(Debug, Deserialize)]
struct Record {
    #[allow(dead_code)]
    id: Thing,
}

#[tokio::main]
async fn main() -> SurrealRes<()> {
    //初始化数据库连接
    let db: SurrealDB = InitServiceImpl::new().init().unwrap();
    //选择命名空间和数据库
    let mut stmt = UseWrapper::new();
    stmt.use_ns("test").and().use_db("test").build();


    //提交
    let r = db.use_commit(stmt).await;
    dbg!(r);
    //
    // //创建表
    // let mut create_table = CreateWrapper::new();
    //
    // // create_table.create("user")
    // //     .id(TableId::<IdFunction>::Fun(IdFunction::RAND))
    // //     .and()
    // //     .set("name","zhangsan")
    // //     .set("email","syf2002@out.com")
    // //     .return_field("name")
    // //     .build();
    //
    // create_table.create("user")
    //     .id(TableId::<IdFunction>::Fun(IdFunction::RAND))
    //     .and()
    //     .content(User {
    //         userId: "123".to_string(),
    //         name: "zhangsan".to_string(),
    //         email: "syf20020816".to_string(),
    //     })
    //     .return_after()
    //     .build();
    //
    //
    // // let res = db.commit(create_table).await?;
    // // dbg!(res);
    // let q = db.core.cn.query("SELECT * FROM user").await?;
    // dbg!(q);
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