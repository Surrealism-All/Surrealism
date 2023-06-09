use surrealism::{DefaultInitServiceImpl, SurrealRes, Wrapper, UseWrapper, DefineWrapper};

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
    /// DEFINE LOGIN username ON NAMESPACE PASSWORD '123456';
    let mut define_wrapper = DefineWrapper::new();
    let mut define_login_ns = define_wrapper.define_login_namespace("username", "123456");
    /// commit
    let res = db.commit(define_login_ns).await;
    dbg!(res.unwrap());
    Ok(())
}