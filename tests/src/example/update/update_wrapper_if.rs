use surrealism::{InitServiceImpl, SurrealRes, Wrapper, UpdateWrapper, UseWrapper, IfElseWrapper, Criteria, CreateWrapper, TableId};

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
    /// 准备数据
    /// prepare data
    let mut data = CreateWrapper::new();
    data
        .create("user")
        .id(TableId::<String>::Str("101".to_string()))
        .set("name", "Jack")
        .set("age", 16)
        .set("railcard", "none")
        .return_after();
    let data_res = db.commit(data).await;
    dbg!(data_res.unwrap());
    ///条件
    let mut cri1 = Criteria::new();
    cri1.lte("age", "10");
    let mut cri2 = Criteria::new();
    cri2.lte("age", "21");
    ///IF—ELSE
    let mut if_wrapper = IfElseWrapper::new();
    if_wrapper
        .if_condition_str(&cri1, "junior")
        .else_if_condition_str(&cri2, "student")
        .else_condition_str("senior");
    ///构建Wrapper
    let mut update_wrapper = UpdateWrapper::new();
    update_wrapper
        .from("user")
        .set_condition("railcard", &mut if_wrapper)
        .return_after();
    /// 提交事务
    let res = db.commit(update_wrapper).await;
    dbg!(res.unwrap());
    Ok(())
}