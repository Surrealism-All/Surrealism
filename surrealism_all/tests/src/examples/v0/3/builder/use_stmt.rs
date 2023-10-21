use surrealism::builder::s_use::UseWrapperImpl;
use surrealism::DefaultRes;
use surrealism::builder::{BaseWrapperImpl, SQLBuilderFactory};

// [tests\src\main.rs:9] use_stmt1 = "USE NS test DB user;"
// [tests\src\main.rs:10] use_stmt2 = "USE NS surrealism;"
#[tokio::main]
async fn main() -> DefaultRes<()> {
    let use_stmt1 = SQLBuilderFactory::use_stmt().ns("test").db("user").build();
    let use_stmt2 = SQLBuilderFactory::use_stmt().ns("surrealism").build();
    dbg!(use_stmt1);
    dbg!(use_stmt2);
    Ok(())
}