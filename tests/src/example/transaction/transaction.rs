use surrealism::{InitServiceImpl, SurrealRes, Wrapper, SelectWrapper, UseWrapper, Transaction};

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
    ///构建Wrapper
    let mut select_wrapper1 = SelectWrapper::new();
    select_wrapper1.select("SELECT name FROM user;");
    let mut select_wrapper2 = SelectWrapper::new();
    select_wrapper2.select("SELECT * FROM user;");
    let mut select_wrapper3 = SelectWrapper::new();
    select_wrapper3.select("SELECT userId FROM user;");
    ///构建事务
    let mut transaction = Transaction::new();
    transaction
        .add(&mut select_wrapper1)
        .add(&mut select_wrapper2)
        .add(&mut select_wrapper3)
        .commit();
    dbg!(transaction.get(1));
    /// 提交事务
    let res = db.commit_transaction(&transaction).await;
    dbg!(res.unwrap());
    Ok(())
}