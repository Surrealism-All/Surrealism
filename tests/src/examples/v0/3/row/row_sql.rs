use surrealism::{DefaultRes};
use surrealism::row::RowSql;

#[tokio::main]
async fn main()->DefaultRes<()>{
    let sql1 = RowSql::new("SELECT {} FROM {}")
        .bind("username")
        .bind("user")
        .build();
    let sql2 = RowSql::new("SELECT {} FROM {} WHERE {} = 56;")
        .bind_index(0,"*")
        .bind_index(2,"age")
        .bind_index(1,"test")
        .build();
    dbg!(sql1.unwrap());
    dbg!(sql2.unwrap());
    Ok(())
}