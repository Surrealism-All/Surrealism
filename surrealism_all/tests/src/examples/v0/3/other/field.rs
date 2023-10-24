
use surrealism::DefaultRes;
use surrealism::db::Field;

// [tests\src\main.rs:12] field_all = "*"
// [tests\src\main.rs:13] field_diff = "DIFF"
// [tests\src\main.rs:14] field = "username AS name"
#[tokio::main]
async fn main() -> DefaultRes<()> {
    let field_all = Field::all().to_string();
    let field_diff = Field::diff().to_string();
    let field = Field::new_field("username").as_field("name").to_string();
    dbg!(field_all);
    dbg!(field_diff);
    dbg!(field);
    Ok(())
}