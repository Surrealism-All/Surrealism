use surrealism::{DefaultInitServiceImpl, SurrealRes, Wrapper, UpdateWrapper, UseWrapper, TableId, CreateWrapper};

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
    /// 构建UpdateWrapper
    /// UPDATE user:1008 MERGE { name:'StuWie' , age:3 } RETURN AFTER;
    let mut update_wrapper = UpdateWrapper::new();
    update_wrapper
        .from("user")
        .id(TableId::<String>::Str("1008".to_string()))
        .merge("name", "StuWie")
        .merge("age",3)
        .return_after();

    /// 提交语句
    /// commit statement
    let update_res = db.commit(&mut update_wrapper).await;
    dbg!(update_res.unwrap());
    Ok(())
}