use surrealism::{DefaultInitService, InitService, SurrealismConnector, SurrealismRes, UseNSDB};

#[tokio::main]
async fn main() -> SurrealismRes<()> {
    let mut service = DefaultInitService::new().init();
    let res = service.use_commit("test", "test").await?;
    dbg!(res);
    Ok(())
}