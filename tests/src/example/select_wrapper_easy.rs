use surrealism::{InitServiceImpl, SurrealRes, UseWrapper, Wrapper, CreateWrapper, TableId, ParseSQL, SQLParser, SelectWrapper};
use serde::{Serialize, Deserialize};

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
    /// 最简单的查询也是最复杂的查询,需要自己手写SQL语句
    /// 用于构建一些SelectWrapper目前做不到的查询
    /// 例如:SELECT count() AS total, math::sum(age), gender, country FROM person GROUP BY gender, country;
    /// The simplest and most complex queries require handwritten SQL statements
    /// Used to build queries that SelectWrapper currently cannot perform
    /// example:SELECT count() AS total, math::sum(age), gender, country FROM person GROUP BY gender, country;
    let mut select_wrapper = SelectWrapper::new();
    select_wrapper.select("SELECT * FROM user;");
    /// 提交语句
    /// commit statement
    let create_res = db.commit(select_wrapper).await;
    dbg!(create_res.unwrap());
    Ok(())
}

