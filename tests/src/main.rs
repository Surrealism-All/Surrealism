use surrealism::builder::SQLBuilderFactory;
use surrealism::DefaultRes;

#[tokio::main]
async fn main() -> DefaultRes<()> {
    let kill = SQLBuilderFactory::kill("734732kjsjd37");
    dbg!(kill);
    Ok(())
}