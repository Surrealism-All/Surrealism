use surrealism::{SurrealismRes, SurrealID};
use surrealism::builder::*;
use surrealism::builder::insert::InsertWrapperImpl;
use surrealism::builder::relate::RelateWrapperImpl;
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
struct Person<'a> {
    name: &'a str,
    company: &'a str,
    skills: Vec<&'a str>,
}

// [tests\src\main.rs:23] insert1.build() = "INSERT INTO company (founded , name) VALUES ('2021-09-10' , 'SurrealDB') , ('2023-01-01' , 'Surrealism');"
// [tests\src\main.rs:38] insert2.build() = "INSERT INTO company CONTENT { company : 'SurrealDB' , name : 'Tobie' , skills : ['Rust', 'Go', 'JS'] };"
// [tests\src\main.rs:44] insert3.build() = "INSERT INTO company CONTENT [ { company : 'SurrealDB' , name : 'Tobie' , skills : ['Rust', 'Go', 'JS'] } , { company : 'Surrealism' , name : 'Mat' , skills : ['TS'] } ];"
// [tests\src\main.rs:49] insert4.build() = "INSERT INTO company ( SELECT * FROM temperature WHERE city = 'San Francisco' );"
#[tokio::main]
async fn main() -> SurrealismRes<()> {
    let mut insert1 = SQLBuilderFactory::insert()
        .table("company")
        .add_set("name", "SurrealDB")
        .add_set("founded", "2021-09-10")
        .add_set("name", "Surrealism")
        .add_set("founded", "2023-01-01")
        .deref_mut();
    dbg!(insert1.build());
    let person1 = Person {
        name: "Tobie",
        company: "SurrealDB",
        skills: vec!["Rust", "Go", "JS"],
    };
    let person2 = Person {
        name: "Mat",
        company: "Surrealism",
        skills: vec!["TS"],
    };
    let mut insert2 = SQLBuilderFactory::insert()
        .table("company")
        .add_content(&person1)
        .deref_mut();
    dbg!(insert2.build());
    let mut insert3 = SQLBuilderFactory::insert()
        .table("company")
        .add_content(&person1)
        .add_content(&person2)
        .deref_mut();
    dbg!(insert3.build());
    let mut insert4 = SQLBuilderFactory::insert()
        .table("company")
        .add_stmt("SELECT * FROM temperature WHERE city = 'San Francisco'")
        .deref_mut();
    dbg!(insert4.build());
    Ok(())
}

