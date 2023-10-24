use surrealism::surreal::{SurrealismRes,DefaultInitService,InitService};

#[tokio::main]
async fn main() -> SurrealismRes<()> {
    let mut service = DefaultInitService::new().init();
    dbg!(service.is_ready());
    Ok(())
}