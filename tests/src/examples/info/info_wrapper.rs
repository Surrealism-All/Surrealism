use surrealism::{SurrealismRes, SurrealID, SurrealValue, Condition, Criteria, CriteriaSign, ConditionSign, TimeUnit, Order};
use surrealism::builder::*;
use surrealism::functions::Function;

// [tests\src\main.rs:13] info1.build() = "INFO FOR DB;"
// [tests\src\main.rs:14] info2.build() = "INFO FOR NS;"
// [tests\src\main.rs:15] info3.build() = "INFO FOR SCOPE name;"
// [tests\src\main.rs:16] info4.build() = "INFO FOR TABLE user;"
#[tokio::main]
async fn main() -> SurrealismRes<()> {
    let info1 = SQLBuilderFactory::info().db();
    let info2= SQLBuilderFactory::info().ns();
    let info3 = SQLBuilderFactory::info().scope("name");
    let info4 = SQLBuilderFactory::info().table("user");
    dbg!(info1.build());
    dbg!(info2.build());
    dbg!(info3.build());
    dbg!(info4.build());
    Ok(())
}
