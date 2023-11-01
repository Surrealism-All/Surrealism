use surrealism::DefaultRes;
use surrealism::builder::{SQLBuilderFactory};

#[tokio::main]
async fn main()->DefaultRes<()>{
    let define_ns = SQLBuilderFactory::define().ns().name("test").to_string();
    dbg!(define_ns);
    Ok(())
}