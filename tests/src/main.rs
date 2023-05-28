use surrealism::{InitServiceImpl, SurrealRes, UseWrapper, Wrapper, CreateWrapper, TableId, ParseSQL, SQLParser, SelectWrapper, Field,Criteria};
use serde::{Serialize, Deserialize};

///    ▄▄▄▄                                                      ▄▄▄▄         ██
///  ▄█▀▀▀▀█                                                     ▀▀██         ▀▀
///  ██▄       ██    ██   ██▄████   ██▄████   ▄████▄    ▄█████▄    ██       ████     ▄▄█████▄  ████▄██▄
///   ▀████▄   ██    ██   ██▀       ██▀      ██▄▄▄▄██   ▀ ▄▄▄██    ██         ██     ██▄▄▄▄ ▀  ██ ██ ██
///       ▀██  ██    ██   ██        ██       ██▀▀▀▀▀▀  ▄██▀▀▀██    ██         ██      ▀▀▀▀██▄  ██ ██ ██
///  █▄▄▄▄▄█▀  ██▄▄▄███   ██        ██       ▀██▄▄▄▄█  ██▄▄▄███    ██▄▄▄   ▄▄▄██▄▄▄  █▄▄▄▄▄██  ██ ██ ██
///   ▀▀▀▀▀     ▀▀▀▀ ▀▀   ▀▀        ▀▀         ▀▀▀▀▀    ▀▀▀▀ ▀▀     ▀▀▀▀   ▀▀▀▀▀▀▀▀   ▀▀▀▀▀▀   ▀▀ ▀▀ ▀▀

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
    ///准备查询条件
    /// prepare select conditions
    let mut field1 = Field::new("userId");
    field1.as_name("stuId");
    let field2 = Field::new("name");
    let fields = vec![field1,field2];
    let mut condition = Criteria::new();
    condition.eq("name","'Rose'");

    /// 构建SelectWrapper
    /// SELECT userId AS stuId , name FROM user WHERE name = 'Rose' ;
    let mut select_wrapper = SelectWrapper::new();
    select_wrapper.select_fields(&fields)
        .from("user")
        .where_condition(&condition);

    /// 提交语句
    /// commit statement
    let create_res = db.commit(select_wrapper).await;
    dbg!(create_res.unwrap());
    Ok(())
}

