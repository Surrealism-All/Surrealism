use std::ops::DerefMut;
use surrealism::builder::define::OnType;
use surrealism::builder::SQLBuilderFactory;
use surrealism::DefaultRes;

// [tests\src\main.rs:22] remove1 = "REMOVE NAMESPACE surrealdb;"
// [tests\src\main.rs:23] remove2 = "REMOVE DATABASE blog;"
// [tests\src\main.rs:24] remove3 = "REMOVE LOGIN writer ON NAMESPACE;"
// [tests\src\main.rs:25] remove4 = "REMOVE TOKEN writer_token ON NAMESPACE;"
// [tests\src\main.rs:26] remove5 = "REMOVE SCOPE documentation;"
// [tests\src\main.rs:27] remove6 = "REMOVE TABLE article;"
// [tests\src\main.rs:28] remove7 = "REMOVE EVENT new_post ON TABLE article;"
// [tests\src\main.rs:29] remove8 = "REMOVE FUNCTION fn::update_author"
// [tests\src\main.rs:30] remove9 = "REMOVE FIELD tags ON TABLE article;"
// [tests\src\main.rs:31] remove10 = "REMOVE INDEX authors ON TABLE article;"
// [tests\src\main.rs:32] remove11 = "REMOVE PARAM $author;"
// [tests\src\main.rs:33] remove12 = "REMOVE USER Matt ON ROOT;"
#[tokio::main]
async fn main() -> DefaultRes<()> {
    let remove1 = SQLBuilderFactory::remove().ns("surrealdb").build();
    let remove2 = SQLBuilderFactory::remove().db("blog").build();
    let remove3 = SQLBuilderFactory::remove().login("writer", OnType::NS).build();
    let remove4 = SQLBuilderFactory::remove().token("writer_token", OnType::NS).build();
    let remove5 = SQLBuilderFactory::remove().scope("documentation").build();
    let remove6 = SQLBuilderFactory::remove().table("article").build();
    let remove7 = SQLBuilderFactory::remove().event("new_post", "article").build();
    let remove8 = SQLBuilderFactory::remove().function("update_author").build();
    let remove9 = SQLBuilderFactory::remove().field("tags", "article").build();
    let remove10 = SQLBuilderFactory::remove().index("authors", "article").build();
    let remove11 = SQLBuilderFactory::remove().param("author").build();
    let remove12 = SQLBuilderFactory::remove().user("Matt",OnType::ROOT).build();
    dbg!(remove1);
    dbg!(remove2);
    dbg!(remove3);
    dbg!(remove4);
    dbg!(remove5);
    dbg!(remove6);
    dbg!(remove7);
    dbg!(remove8);
    dbg!(remove9);
    dbg!(remove10);
    dbg!(remove11);
    dbg!(remove12);
    Ok(())
}