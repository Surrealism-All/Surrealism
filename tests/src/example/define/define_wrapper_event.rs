use surrealism::{InitServiceImpl, SurrealRes, Wrapper, UseWrapper, DefineWrapper, Criteria};

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
    /// Define Event
    /// DEFINE EVENT my_event ON TABLE my_table WHEN surrealism >= 90 THEN ( UPDATE my_table SET status = 'pass' WHERE id = 1 );
    let then_stmt = "UPDATE my_table SET status = 'pass' WHERE id = 1";
    let mut cri = Criteria::new();
    cri.gte("surrealism", "90");
    let mut define_wrapper = DefineWrapper::new();
    let mut define_event = define_wrapper.define_event("my_event", "my_table", &cri, then_stmt);
    /// commit
    let res = db.commit(define_event).await;
    dbg!(res.unwrap());
    Ok(())
}