use surrealism::{InitServiceImpl, SurrealRes, Wrapper, InfoWrapper,UseWrapper};

#[tokio::main]
async fn main() -> SurrealRes<()> {
    ///初始化连接
    ///init connection
    let db = InitServiceImpl::new().init().unwrap();
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
    ///构建InfoWrapper
    let mut info = InfoWrapper::new();
    info.db();
    /// 提交语句
    /// commit statement
    let res = db.commit(info).await;
    dbg!(res.unwrap());
    Ok(())
}