use surrealism::{InitServiceImpl, SurrealRes, Wrapper, UseWrapper, DefineWrapper};

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
    /// 构建DefineWrapper
    /// build DefineWrapper
    let mut define_wrapper = DefineWrapper::new();
    /// 通过define_namespace()转为DefineNamespace
    /// use define_namespace() to DefineNamespace
    let mut define_ns = define_wrapper.define_namespace("test");
    /// 提交
    /// commit
    let res = db.commit(define_ns).await;
    dbg!(res.unwrap());
    Ok(())
}

