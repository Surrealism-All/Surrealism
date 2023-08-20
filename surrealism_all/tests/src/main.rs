mod lib;

use surrealism::{DefaultInitService, InitService, SurrealismConnector, SurrealismRes};

#[tokio::main]
async fn main() -> SurrealismRes<()> {
    let mut service = DefaultInitService::new().init();
    Ok(())
}