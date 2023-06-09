use surrealism::{DefaultInitServiceImpl, SurrealRes, Wrapper, DeleteWrapper, UseWrapper, Criteria, TimeUnit, TableId};

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
    /// 构建条件
    let mut condition = Criteria::new();
    condition.lt("age", "26");
    ///构建DeleteWrapper
    /// DELETE  user:2incz3ad74jlza71m0jq RETURN AFTER;
    let mut delete_wrapper = DeleteWrapper::new();
    delete_wrapper
        .from("user")
        .id(TableId::<String>::Str("2incz3ad74jlza71m0jq".to_string()))
        .return_after();
    /// 提交语句
    /// commit statement
    let create_res = db.commit(&mut delete_wrapper).await;
    dbg!(create_res.unwrap());
    Ok(())
}