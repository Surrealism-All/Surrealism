use surrealism::{DefaultInitService, InitService, SurrealismConnector, SurrealismRes, SurrealismCommit, UseNSDB, RowSql, row_sql};

#[tokio::main]
async fn main() -> SurrealismRes<()> {
    let mut service = DefaultInitService::new().init();
    let _ = service.use_commit("test", "test").await;
    let sql1 = row_sql!("SELECT {} FROM {};")
        .bind("*")
        .bind("test")
        .build()
        .unwrap();
    let commit2 = service.commit_sql(&sql1).await?;
    dbg!(commit2);
    Ok(())
}