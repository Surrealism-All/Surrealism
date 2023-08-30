use surrealism::{DefaultInitService, SurrealismRes, InitService, UseNSDB, SurrealismCommit};

#[tokio::main]
async fn main() -> SurrealismRes<()> {
    let mut service = DefaultInitService::new().init();
    let res = service.use_commit("test", "test").await?;
    dbg!(res);
    let commit1 = service.commit_sql("select * from test;").await?;
    dbg!(commit1);
    Ok(())
}