mod lib;

use surrealism::{DefaultInitService, SurrealismRes, InitService, UseNSDB, SurrealismCommit};

#[tokio::main]
async fn main() -> SurrealismRes<()> {
    let mut service = DefaultInitService::new().init();
    let res = service.use_commit("test", "test").await;
    dbg!(res);
    let commit1 = service.commit_sql("select * from test;").await;
    dbg!(commit1);
    Ok(())
}

// let mut create = SQLBuilderFactory::create()
//     .table("surrealism")
//     .id(SurrealID::RAND)
//     .set()
//     .add("name", SurrealValue::Str(String::from("Mat")))
//     .timeout(TimeOut::new(5, TimeUnit::SECOND))
//     .return_type(ReturnType::After)
//     .parallel()
//     .deref_mut();
// dbg!(&create.build());
// dbg!(&create.build());