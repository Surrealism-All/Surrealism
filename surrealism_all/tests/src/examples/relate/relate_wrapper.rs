use surrealism::{SurrealismRes,SurrealID};
use surrealism::builder::*;
use surrealism::builder::relate::RelateWrapperImpl;


#[tokio::main]
async fn main() -> SurrealismRes<()> {
    // RELATE person:l19zjikkw1p1h9o6ixrg->wrote->article:8nkk6uj4yprt49z7y3zm;
    let mut relate1 = SQLBuilderFactory::relate()
        .table_from("person", SurrealID::from("l19zjikkw1p1h9o6ixrg"))
        .table_with("wrote", SurrealID::Default)
        .table_to("article", SurrealID::from("8nkk6uj4yprt49z7y3zm"))
        .deref_mut();
    dbg!(relate1.build());
    Ok(())
}

