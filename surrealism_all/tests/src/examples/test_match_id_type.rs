use surrealism::{SurrealismRes, SurrealID, Array, SurrealValue, Object, Range, ParamCombine, Table, match_id_type};

#[tokio::main]
async fn main() -> SurrealismRes<()> {
    let res1 = match_id_type(&5);
    let res2 = match_id_type(&56.455_f32);
    let range = Range::new(SurrealValue::Int(2), SurrealValue::Int(65), false);
    let res3 = match_id_type(&range);
    dbg!(res1);
    dbg!(res2);
    dbg!(res3);
    Ok(())
}
