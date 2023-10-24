use surrealism::{DefaultInitService, InitService, SurrealismConnector, SurrealismRes, SurrealismCommit, UseNSDB, RowSql};

#[tokio::main]
async fn main() -> SurrealismRes<()> {
    let mut service = DefaultInitService::new().init();
    let _ = service.use_commit("test", "test").await;
    let sql1 = RowSql::new("SELECT {} FROM {};")
        .bind("*")
        .bind("test")
        .build()
        .unwrap();
    // use bind_index() function
    // let sql2 = RowSql::new("SELECT {} FROM {} WHERE {} = 56;")
    //     .bind_index(0,"*")
    //     .bind_index(2,"age")
    //     .bind_index(1,"test")
    //     .build();
    let commit2 = service.commit_sql(&sql1).await?;
    dbg!(commit2);
    Ok(())
}
