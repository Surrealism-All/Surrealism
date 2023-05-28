use surrealism::{InitServiceImpl, SurrealRes, UseWrapper, Wrapper, CreateWrapper, TableId, ParseSQL, SQLParser};
use serde::{Serialize, Deserialize};

///构建结构体,需要使用surrealism提供的宏:ParseSQL
/// build struct,Need to use the macro provided by surrealism: ParseSQL
/// 请注意:ParseSQl宏和SQLParser trait要同时导入
/// Please note that the ParseSQl macro and SQLParser trait need to be imported simultaneously
#[derive(Debug, Serialize, Deserialize, ParseSQL)]
struct User {
    pub userId: String,
    pub name: String,
    pub email: String,
}

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
    ///创建CreateWrapper
    /// new CreateWrapper
    let mut create_wrapper = CreateWrapper::new();
    /// 设置构建语句
    /// set create statement
    /// CREATE user:t10088 CONTENT {userId:'mark001',name:'Mark',email:'mark@outlook.com'} RETURN AFTER;
    create_wrapper.create("user")
        .id(TableId::<String>::Str("t10088".to_string()))
        .content(User {
            userId: String::from("mark001"),
            name: String::from("Mark"),
            email: String::from("mark@outlook.com"),
        })
        .return_after();
    /// 提交语句
    /// commit statement
    let create_res = db.commit(create_wrapper).await;
    dbg!(create_res.unwrap());
    Ok(())
}
