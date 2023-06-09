use surrealism::{DefaultInitServiceImpl, SurrealRes, UseWrapper, Wrapper, CreateWrapper, TableId};

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
    ///创建CreateWrapper
    /// new CreateWrapper
    let mut create_wrapper = CreateWrapper::new();
    /// 设置构建语句
    /// set create statement
    /// CREATE user:t10086 SET name='Jack',userId='jack001' RETURN NONE;
    create_wrapper.create("user")
        .id(TableId::<String>::Str("t10086".to_string()))
        .set("name", "Jack")
        .set("userId", "jack001")
        .return_none();
    /// 提交语句
    /// commit statement
    let create_res = db.commit(&mut create_wrapper).await;
    dbg!(create_res.unwrap());
    Ok(())
}
