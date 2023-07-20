use surrealism::{DefaultInitServiceImpl, SurrealRes, Wrapper, InfoWrapper};

#[tokio::main]
async fn main() -> SurrealRes<()> {
    ///初始化连接
    ///init connection
    let db = DefaultInitServiceImpl::new().init().unwrap();
    /// 构建InfoWrapper
    /// build InfoWrapper
    let mut info = InfoWrapper::new();
    info.kv();
    /// 提交语句
    /// commit statement
    let res = db.commit(&mut info).await;
    dbg!(res.unwrap());
    Ok(())
}