use surrealism::builder::s_use::UseWrapperImpl;
use surrealism::DefaultRes;
use surrealism::builder::{BaseWrapperImpl, SQLBuilderFactory, TableImpl};
use surrealism::db::Operator;
use surrealism::builder::insert::InsertWrapperImpl;


#[tokio::main]
async fn main() -> DefaultRes<()> {
    let insert1 = SQLBuilderFactory::insert()
        .table("company")
        .add_set("name","SurrealDB")
        .add_set("founded","2023-10-20")
        .build();
    let insert2 =  SQLBuilderFactory::insert()
        .table("product")
        .add_set("name","Salesforce")
        .add_set("url","salesforce.com")
        .upsert("tags","crm".into(),Operator::Add)
        .build();
    dbg!(insert1);
    dbg!(insert2);
    Ok(())
}