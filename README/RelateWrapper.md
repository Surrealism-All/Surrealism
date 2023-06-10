# RelateWrapper

> RelateWrapper的作用是：帮助我们构建Relate语句
>
> RelateWrapper：Help us build the Relate statement

RELATE语句可用于在数据库中的两个记录之间生成图形边

The RELATE statement can be used to generate graph edges between two records in the database.

## function

| fun name      | params:type                                                  | return             | des                                                          |
| ------------- | ------------------------------------------------------------ | ------------------ | ------------------------------------------------------------ |
| from          | from_table: &str                                             | &mut RelateWrapper | 设置起始的表<br />Set starting table                         |
| to            | to_table: &str                                               | &mut RelateWrapper | 设置结果存储表<br />Set result storage table                 |
| with          | with_table: &str                                             | &mut RelateWrapper | 设置关联表<br />Set Association Table                        |
| set           | 1. field_name: &'static str<br />2. value: T `(<T: Serialize>)` | &mut RelateWrapper | SET方式构建字段<br />SET method for constructing fields      |
| content       | content_obj: T`(<T: Serialize>)`                             | &mut RelateWrapper | CONTENT方式构建字段<br />CONTENT method for constructing fields |
| return none   |                                                              | &mut RelateWrapper | RETURN NONE                                                  |
| return_diff   |                                                              | &mut RelateWrapper | RETURN DIFF                                                  |
| return_before |                                                              | &mut RelateWrapper | RETURN BEFORE                                                |
| return_after  |                                                              | &mut RelateWrapper | RETURN AFTER                                                 |
| return_field  | field_names: `Vec<&str>`                                     | &mut RelateWrapper | RETURN FIELD                                                 |
| timeout       | 1. time: usize<br />2. unit: TimeUnit                        | &mut RelateWrapper | TIMEOUT                                                      |
| parallel      |                                                              | &mut RelateWrapper | 设置并行<br />Set Parallelism                                |
| define        | stmt: &str                                                   | （）               | 自定义语句<br />define Statement                             |

## Import

如果你想使用`RelateWrapper`，你必须从`surrealism`导入`RelateWrapper`和`Wrapper`

If you wanna use `RelateWrapper` , you must import `RelateWrapper` and `Wrapper` from `surrealism`

```rust
use surrealism::{RelateWrapper, Wrapper};
```

## main.rs

```rust
use surrealism::{DefaultInitServiceImpl, SurrealRes, Wrapper, UseWrapper, RelateWrapper, TimeUnit};
use serde::{Serialize};

#[derive(Debug, Serialize)]
struct Test {
    source: String,
    tags: Vec<&'static str>,
}

#[tokio::main]
async fn main() -> SurrealRes<()> {
    ///初始化连接
    ///init connection
    let db = DefaultInitServiceImpl::new().init().unwrap();
    ///创建UseWrapper
    /// new UseWrapper
    let mut use_wrapper = UseWrapper::new();
    /// 设置命名空间和数据库
    /// Set namespace and database
    use_wrapper.use_ns("test").use_db("test");
    /// 提交语句
    /// commit statement
    let res_use = db.use_commit(&mut use_wrapper).await;
    dbg!(res_use);
    /// RelateWrapper
    /// RELATE user:tobie -> write -> article:surreal CONTENT {source:'surrealism',tags:['surrealism','surrealdb']} RETURN AFTER TIMEOUT 10ms PARALLEL;
    let mut relate_wrapper = RelateWrapper::new();
    relate_wrapper
        .from("user:tobie")
        .to("write")
        .with("article:surreal")
        .content(Test {
            source: "surrealism".to_string(),
            tags: vec!["surrealism", "surrealdb"],
        })
        .timeout(10, TimeUnit::MILLISECOND)
        .return_after()
        .parallel();
    /// 提交事务
    /// commit
    let res = db.commit(&mut relate_wrapper).await;
    dbg!(res.unwrap());
    Ok(())
}
```

