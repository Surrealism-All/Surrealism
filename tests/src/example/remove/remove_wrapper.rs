use surrealism::{DefaultInitServiceImpl, SurrealRes, Wrapper, UseWrapper, RemoveWrapper};

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
    /// Remove
    let mut remove_wrapper = RemoveWrapper::new();
    remove_wrapper.namespace("test");
    dbg!(remove_wrapper.commit());
    remove_wrapper.scope("test");
    dbg!(remove_wrapper.commit());
    remove_wrapper.index("userEmail", "user");
    dbg!(remove_wrapper.commit());
    /// 提交事务
    /// commit
    let res = db.commit(&mut remove_wrapper).await;
    dbg!(res.unwrap());
    Ok(())
}