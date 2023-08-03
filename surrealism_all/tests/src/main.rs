use surrealism::{DefaultInitService, InitService,SurrealismConnector,SurrealismRes};


#[tokio::main]
async fn main()->Result<(),&'static str>{
    let mut service = DefaultInitService::new().init();
    Ok(())
}
