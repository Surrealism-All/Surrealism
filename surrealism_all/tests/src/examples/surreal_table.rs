use surrealism::{SurrealismRes, IDNumber, SurrealID, Table};

// [tests\src\main.rs:18] table1 = "test:surrealdb"
// [tests\src\main.rs:19] table2 = "temperature:17493"
// [tests\src\main.rs:20] table3 = "temperature:['London', 'New York']"
// [tests\src\main.rs:21] table4 = "user:rand()"
#[tokio::main]
async fn main() -> SurrealismRes<()> {
    let table1 = Table::new("test", SurrealID::<String>::Str("surrealdb".to_string())).build();
    let table2 = Table::new_no_arg().table("temperature").id(SurrealID::<IDNumber>::Number(IDNumber::Int(17493))).build();
    let table3 = Table::<String>::new_into("temperature", "['London', 'New York']").build();
    let table4 = Table::new("user", SurrealID::<String>::RAND).build();
    dbg!(table1);
    dbg!(table2);
    dbg!(table3);
    dbg!(table4);
    Ok(())
}