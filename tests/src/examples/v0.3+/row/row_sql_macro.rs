use surrealism::{DefaultRes,row_sql};
use surrealism::row::RowSql;


#[tokio::main]
async fn main()->DefaultRes<()>{
    let sql = row_sql!("SELECT {} FROM {};")
        .bind("*")
        .bind("user")
        .build();
    dbg!(sql.unwrap());
    Ok(())
}