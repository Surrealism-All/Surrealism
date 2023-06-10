use surrealism::{DefaultInitServiceImpl, SurrealRes, Wrapper, UseWrapper, RelateWrapper, TimeUnit};
use serde::{Serialize};

#[derive(Debug, Serialize)]
struct Test {
    source: String,
    tags: Vec<&'static str>,
}

#[tokio::main]
async fn main() -> SurrealRes<()> {
    ///初始化连接
    ///init connection
    let db = DefaultInitServiceImpl::new().init().unwrap();
    ///创建UseWrapper
    /// new UseWrapper
    let mut use_wrapper = UseWrapper::new();
    /// 设置命名空间和数据库
    /// Set namespace and database
    use_wrapper.use_ns("test").use_db("test");
    /// 提交语句
    /// commit statement
    let res_use = db.use_commit(&mut use_wrapper).await;
    dbg!(res_use);
    /// RelateWrapper
    /// RELATE user:tobie -> write -> article:surreal CONTENT {source:'surrealism',tags:['surrealism','surrealdb']} RETURN AFTER TIMEOUT 10ms PARALLEL;
    let mut relate_wrapper = RelateWrapper::new();
    relate_wrapper
        .from("user:tobie")
        .to("write")
        .with("article:surreal")
        .content(Test {
            source: "surrealism".to_string(),
            tags: vec!["surrealism", "surrealdb"],
        })
        .timeout(10, TimeUnit::MILLISECOND)
        .return_after()
        .parallel();
    /// 提交事务
    /// commit
    let res = db.commit(&mut relate_wrapper).await;
    dbg!(res.unwrap());
    Ok(())
}