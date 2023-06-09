///    ▄▄▄▄                                                      ▄▄▄▄         ██
///  ▄█▀▀▀▀█                                                     ▀▀██         ▀▀
///  ██▄       ██    ██   ██▄████   ██▄████   ▄████▄    ▄█████▄    ██       ████     ▄▄█████▄  ████▄██▄
///   ▀████▄   ██    ██   ██▀       ██▀      ██▄▄▄▄██   ▀ ▄▄▄██    ██         ██     ██▄▄▄▄ ▀  ██ ██ ██
///       ▀██  ██    ██   ██        ██       ██▀▀▀▀▀▀  ▄██▀▀▀██    ██         ██      ▀▀▀▀██▄  ██ ██ ██
///  █▄▄▄▄▄█▀  ██▄▄▄███   ██        ██       ▀██▄▄▄▄█  ██▄▄▄███    ██▄▄▄   ▄▄▄██▄▄▄  █▄▄▄▄▄██  ██ ██ ██
///   ▀▀▀▀▀     ▀▀▀▀ ▀▀   ▀▀        ▀▀         ▀▀▀▀▀    ▀▀▀▀ ▀▀     ▀▀▀▀   ▀▀▀▀▀▀▀▀   ▀▀▀▀▀▀   ▀▀ ▀▀ ▀▀

use surrealism::{DefaultInitServiceImpl, SurrealRes, Wrapper, UseWrapper, DefineWrapper, parse_response};

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
    let mut define_param = define_wrapper.define_param();
    define_param
        .param("endpointBase")
        .value("surrealism");
    /// 提交事务
    /// commit
    let res = db.commit(&mut define_param).await;
    dbg!(res.unwrap());
    let mut param_res = db.return_param("$endpointBase").await?;
    /// 将返回的参数解析为Rust可用类型
    let response_parse: String = parse_response(param_res);
    dbg!(&response_parse);
    Ok(())
}