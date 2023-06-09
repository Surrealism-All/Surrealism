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
    let res_use = db.use_commit(&mut use_wrapper).await;
    dbg!(res_use);
    /// 通过define_database()转为DefineDatabase
    /// use define_database() to DefineDatabase
    /// DEFINE DATABASE test;
    let mut define_wrapper = DefineWrapper::new();
    let mut define_index = define_wrapper.define_index();
    define_index
        .index("userEmailIndex")
        .table("user")
        .field("email");
    /// 提交事务
    /// commit
    let res = db.commit(&mut define_index).await;
    dbg!(res.unwrap());
    Ok(())
}