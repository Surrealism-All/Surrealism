use surrealism::{ParamCombine, SurrealismRes, ReturnType};

#[tokio::main]
async fn main() -> SurrealismRes<()> {
    let return_none = ReturnType::None;
    let return_before = ReturnType::Before;
    let return_after = ReturnType::After;
    let return_field = ReturnType::Field("name");
    dbg!(&return_none);
    dbg!(&return_before);
    dbg!(&return_after);
    dbg!(&return_field);
    dbg!(&return_field.combine());
    dbg!(&return_after.combine());
    Ok(())
}