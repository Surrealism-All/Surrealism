use surrealism::{SurrealismRes};
use surrealism::builder::*;

#[tokio::main]
async fn main() -> SurrealismRes<()> {
    let transaction = SQLBuilderFactory::transaction()
        .add("CREATE account:one SET balance = 135,605.16;")
        .add("CREATE account:two SET balance = 91,031.31;")
        .add("UPDATE account:one SET balance += 300.00;")
        .commit();
    dbg!(transaction);
    Ok(())
}

