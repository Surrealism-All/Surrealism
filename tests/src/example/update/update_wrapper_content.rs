use surrealism::{DefaultInitServiceImpl, SurrealRes, Wrapper, UpdateWrapper, UseWrapper, TableId, CreateWrapper};
use serde::{Serialize, Deserialize};

///构建结构体,需要使用serde提供的宏:Serialize, Deserialize
/// build struct,Need to use the macro provided by serde::{Serialize, Deserialize}
#[derive(Debug, Serialize, Deserialize)]
struct User {
    pub name: String,
    pub age: usize,
}

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
    /// 准备数据
    /// prepare data
    let data = User {
        name: "Mask".to_string(),
        age: 16,
    };
    /// 构建UpdateWrapper
    /// UPDATE user:1008 CONTENT {name:'Mask',age:16} RETURN AFTER;
    let mut update_wrapper = UpdateWrapper::new();
    update_wrapper
        .from("user")
        .id(TableId::<String>::Str("1008".to_string()))
        .content(&data)
        .return_after();
    /// 提交语句
    /// commit statement
    let update_res = db.commit(update_wrapper).await;
    dbg!(update_res.unwrap());
    Ok(())
}