use surrealism::{DefaultInitServiceImpl, SurrealRes, Wrapper, UseWrapper, DefineWrapper, FieldType};

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

    ///DEFINE FUNCTION fn::greet($name: string) {
    /// 	RETURN "Hello, " + $name + "!";
    /// }
    let mut define_wrapper = DefineWrapper::new();
    let mut define_fn = define_wrapper.define_function();
    define_fn
        .fn_name("greet")
        .fn_params("name", &FieldType::String)
        .fn_content(r#"RETURN "Hello, " + $name + "!";"#);
    /// commit
    let res = db.commit(&mut define_fn).await;
    dbg!(res.unwrap());
    let res2 = db.run_fn(&mut define_fn, &vec!["Tobie"]).await;
    dbg!(res2.unwrap());
    Ok(())
}