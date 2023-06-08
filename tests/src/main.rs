///    ▄▄▄▄                                                      ▄▄▄▄         ██
///  ▄█▀▀▀▀█                                                     ▀▀██         ▀▀
///  ██▄       ██    ██   ██▄████   ██▄████   ▄████▄    ▄█████▄    ██       ████     ▄▄█████▄  ████▄██▄
///   ▀████▄   ██    ██   ██▀       ██▀      ██▄▄▄▄██   ▀ ▄▄▄██    ██         ██     ██▄▄▄▄ ▀  ██ ██ ██
///       ▀██  ██    ██   ██        ██       ██▀▀▀▀▀▀  ▄██▀▀▀██    ██         ██      ▀▀▀▀██▄  ██ ██ ██
///  █▄▄▄▄▄█▀  ██▄▄▄███   ██        ██       ▀██▄▄▄▄█  ██▄▄▄███    ██▄▄▄   ▄▄▄██▄▄▄  █▄▄▄▄▄██  ██ ██ ██
///   ▀▀▀▀▀     ▀▀▀▀ ▀▀   ▀▀        ▀▀         ▀▀▀▀▀    ▀▀▀▀ ▀▀     ▀▀▀▀   ▀▀▀▀▀▀▀▀   ▀▀▀▀▀▀   ▀▀ ▀▀ ▀▀

use surrealism::{InitServiceImpl, SurrealRes, Wrapper, UseWrapper, DefineWrapper, FieldType};

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
    let res_use = db.use_commit(&mut use_wrapper).await;
    dbg!(res_use);
    /// DEFINE FIELD countrycode ON TABLE user TYPE string ASSERT $value != NONE AND $value = /[A-Z]{3}/ VALUE $value OR 'GBR';
    let mut define_wrapper = DefineWrapper::new();
    let mut define_field = define_wrapper.define_field();
    define_field
        .field("countrycode")
        .table("user")
        .field_type(&FieldType::String)
        .variable("value")
        .field_assert("$value != NONE AND $value = /[A-Z]{3}/")
        .default_value("GBR");
    /// commit
    let res = db.commit(&mut define_field).await;
    dbg!(res.unwrap());
    Ok(())
}