mod lib;

use surrealism::{SurrealismRes, SQLBuilderFactory, SurrealID, SurrealValue, TimeOut, TimeUnit,ReturnType};

#[tokio::main]
async fn main() -> SurrealismRes<()> {
    let create = SQLBuilderFactory::create()
        .table("surrealism")
        .id(SurrealID::RAND)
        .set()
        .add("name",SurrealValue::Str(String::from("Mat")))
        .timeout(TimeOut::new(5,TimeUnit::SECOND))
        .return_type(ReturnType::After)
        .parallel()
        .build();
    dbg!(create);
    Ok(())
}
