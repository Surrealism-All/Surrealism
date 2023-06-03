///    ▄▄▄▄                                                      ▄▄▄▄         ██
///  ▄█▀▀▀▀█                                                     ▀▀██         ▀▀
///  ██▄       ██    ██   ██▄████   ██▄████   ▄████▄    ▄█████▄    ██       ████     ▄▄█████▄  ████▄██▄
///   ▀████▄   ██    ██   ██▀       ██▀      ██▄▄▄▄██   ▀ ▄▄▄██    ██         ██     ██▄▄▄▄ ▀  ██ ██ ██
///       ▀██  ██    ██   ██        ██       ██▀▀▀▀▀▀  ▄██▀▀▀██    ██         ██      ▀▀▀▀██▄  ██ ██ ██
///  █▄▄▄▄▄█▀  ██▄▄▄███   ██        ██       ▀██▄▄▄▄█  ██▄▄▄███    ██▄▄▄   ▄▄▄██▄▄▄  █▄▄▄▄▄██  ██ ██ ██
///   ▀▀▀▀▀     ▀▀▀▀ ▀▀   ▀▀        ▀▀         ▀▀▀▀▀    ▀▀▀▀ ▀▀     ▀▀▀▀   ▀▀▀▀▀▀▀▀   ▀▀▀▀▀▀   ▀▀ ▀▀ ▀▀

use surrealism::{InitServiceImpl, SurrealRes, Wrapper, DeleteWrapper, UseWrapper, Criteria, TimeUnit, TableId};

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
    /// 构建条件
    let mut condition = Criteria::new();
    condition.lt("age", "26");
    ///构建DeleteWrapper
    /// DELETE  user:10086 WHERE age  <  26 RETURN AFTER TIMEOUT 5s;
    let mut delete_wrapper = DeleteWrapper::new();
    delete_wrapper
        .from("user:10086")
        .where_condition(&condition)
        .return_after()
        .timeout(5, TimeUnit::SECOND);
    dbg!(delete_wrapper.commit());
    /// 提交语句
    /// commit statement
    // let create_res = db.commit(delete_wrapper).await;
    // dbg!(create_res.unwrap());
    Ok(())
}