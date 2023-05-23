use surrealdb::engine::remote::ws::Client;
use surrealdb::Surreal;
use surrealism::{SurrealRes, InitServiceImpl, SurrealDB, UseWrapper, Wrapper, TableId, IdFunction, SQLParser, Field, ParseSQL, CreateWrapper, SelectWrapper};
use serde::{Serialize, Deserialize};


#[derive(Debug, Serialize, Deserialize, ParseSQL)]
struct User {
    pub userId: String,
    pub name: String,
    pub email: String,
}


#[tokio::main]
async fn main() -> SurrealRes<()> {
    //初始化数据库连接
    let db: SurrealDB = InitServiceImpl::new().init().unwrap();
    //选择命名空间和数据库
    let mut stmt = UseWrapper::new();
    stmt.use_ns("test").use_db("test");
    //提交
    let res = db.use_commit(stmt).await;
    dbg!(res);
    //创建表
    let mut create_table = CreateWrapper::new();
    //使用SET方式
    // create_table.create("user")
    //     .id(TableId::<IdFunction>::Fun(IdFunction::RAND))
    //     .set("name", "zhangsan")
    //     .set("email", "syf2002@out.com")
    //     .return_diff();


    //使用CONTENT方式
    create_table.create("user")
        .id(TableId::<IdFunction>::Fun(IdFunction::RAND))
        .content(User {
            userId: "123".to_string(),
            name: "zhangsan".to_string(),
            email: "syf20020816".to_string(),
        })
        .return_after();


    //提交
    let res = db.commit(create_table).await?;
    dbg!(res);
    // let mut queryWrapper = SelectWrapper::new();
    // let mut f_v = Vec::new();
    // let mut f1= Field::new("userId");
    // f1.as_name("stuID");
    // let mut f2= Field::new("name");
    // f2.as_name("stuName");
    // f_v.push(f1);
    // f_v.push(f2);
    // queryWrapper.select_fields(&f_v).and().from("user").build();
    // dbg!(queryWrapper.commit());
    // let query_res = db.commit(queryWrapper).await?;
    // dbg!(query_res);
    Ok(())
}

