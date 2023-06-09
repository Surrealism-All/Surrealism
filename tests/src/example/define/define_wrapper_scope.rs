use surrealism::{DefaultInitServiceImpl, SurrealRes, Wrapper, UseWrapper, DefineWrapper, TokenType, TimeUnit};

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
    ///Define Scope
    let sign_up = "CREATE user SET email = surrealism@outlook.com, pass = crypto::argon2::generate(surrealism)";
    let sign_in ="SELECT * FROM user WHERE email = surrealism@outlook.com AND crypto::argon2::compare(pass, surrealism)";
    let mut define_wrapper = DefineWrapper::new();
    let mut define_scope = define_wrapper.define_scope("account", 24, TimeUnit::HOUR, sign_up, sign_in);
    /// commit
    let res = db.commit(&mut define_scope).await;
    dbg!(res.unwrap());
    Ok(())
}