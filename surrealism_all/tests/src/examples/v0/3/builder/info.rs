use std::ops::DerefMut;
use surrealism::builder::SQLBuilderFactory;
use surrealism::DefaultRes;

// [tests\src\main.rs:14] info1.build() = "INFO FOR DB;"
// [tests\src\main.rs:15] info2.build() = "INFO FOR NS;"
// [tests\src\main.rs:16] info3.build() = "INFO FOR SCOPE name;"
// [tests\src\main.rs:17] info4.build() = "INFO FOR TABLE user;"
// [tests\src\main.rs:18] info5.build() = "INFO FOR KV;"
#[tokio::main]
async fn main() -> DefaultRes<()> {
    let info1 = SQLBuilderFactory::info().db();
    let info2= SQLBuilderFactory::info().ns();
    let info3 = SQLBuilderFactory::info().scope("name");
    let info4 = SQLBuilderFactory::info().table("user");
    let info5 = SQLBuilderFactory::info().kv();
    dbg!(info1.build());
    dbg!(info2.build());
    dbg!(info3.build());
    dbg!(info4.build());
    dbg!(info5.build());
    Ok(())
}