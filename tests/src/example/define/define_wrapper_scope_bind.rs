use surrealism::{InitServiceImpl, SurrealRes, Wrapper, UseWrapper, DefineWrapper, TokenType, TimeUnit};

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
    /// 采用相对固定的语句构建Scope
    /// Using relatively fixed statements to build a Scope
    let mut define_wrapper = DefineWrapper::new();
    let mut define_scope = define_wrapper.define_scope_bind("account", 24, TimeUnit::HOUR, "surrealism@outlook.com", "surrealism");
    /// commit
    let res = db.commit(define_scope).await;
    dbg!(res.unwrap());
    Ok(())
}