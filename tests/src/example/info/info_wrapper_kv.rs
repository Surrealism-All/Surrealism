use surrealism::{InitServiceImpl, SurrealRes, Wrapper, InfoWrapper};

#[tokio::main]
async fn main() -> SurrealRes<()> {
    ///初始化连接
    ///init connection
    let db = InitServiceImpl::new().init().unwrap();
    /// 构建InfoWrapper
    /// build InfoWrapper
    let mut info = InfoWrapper::new();
    info.kv();
    /// 提交语句
    /// commit statement
    let res = db.commit(info).await;
    dbg!(res.unwrap());
    Ok(())
}