use surrealism::{DefaultInitServiceImpl, SurrealRes, Wrapper, InsertWrapper, UseWrapper};

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
    let res_use = db.use_commit(use_wrapper).await;
    dbg!(res_use);
    /// 构建InsertWrapper
    /// 通过键值对形式构建传统语句
    /// Constructing Traditional Statements through Key Value Pairs
    /// INSERT INTO user (name , userId , age) VALUES ('Kaye' , 'kaye001' , 56);
    let mut insert_wrapper = InsertWrapper::new();
    insert_wrapper
        .insert_into("user")
        .set("name", "Kaye")
        .set("userId", "kaye001")
        .set("age", 56);
    /// 提交语句
    /// commit statement
    let create_res = db.commit(insert_wrapper).await;
    dbg!(create_res);
    Ok(())
}