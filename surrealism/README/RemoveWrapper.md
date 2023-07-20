# RemoveWrapper

> RemoveWrapper的作用是：帮助我们构建Remove语句
>
> RemoveWrapper：Help us build the Remove statement

Remove语句可用来删除数据库中定义的表、Database、命名空间、参数、方法等

The Remove statement can be used to delete tables, databases, namespaces, parameters, methods, etc. defined in the database

## function

| fun name  | params:type                       | return | des                       |
| --------- | --------------------------------- | ------ | ------------------------- |
| namespace | name: &str                        | ()     | REMOVE NAMESPACE          |
| database  | name: &str                        | ()     | REMOVE DATABASE           |
| login_ns  | name: &str                        | ()     | REMOVE LOGIN ON NAMESPACE |
| login_db  | name: &str                        | ()     | REMOVE LOGIN ON DATABASE  |
| token_ns  | name: &str                        | ()     | REMOVE TOKEN ON NAMESPACE |
| token_db  | name: &str                        | ()     | REMOVE TOKEN ON DATABASE  |
| scope     | name: &str                        | ()     | REMOVE SCOPE              |
| table     | name: &str                        | ()     | REMOVE TABLE              |
| event     | 1. name: &str<br />2. table: &str | ()     | REMOVE EVENT              |
| function  | name: &str                        | ()     | REMOVE FUNCTION           |
| field     | 1. name: &str<br />2. table: &str | ()     | REMOVE FIELD              |
| index     | 1. name: &str<br />2. table: &str | ()     | REMOVE INDEX              |
| param     | name: &str                        | ()     | REMOVE PARAM              |
| define    | stmt: &str                        | ()     | REMOVE ANY                |

## Import

如果你想使用`RemoveWrapper`，你必须从`surrealism`导入`RemoveWrapper`和`Wrapper`

If you wanna use `RemoveWrapper` , you must import `RemoveWrapper` and `Wrapper` from `surrealism`

```rust
use surrealism::{RemoveWrapper, Wrapper};
```

## main.rs

```rust
use surrealism::{DefaultInitServiceImpl, SurrealRes, Wrapper, UseWrapper, RemoveWrapper};

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
    /// Remove
    let mut remove_wrapper = RemoveWrapper::new();
    remove_wrapper.namespace("test");
    dbg!(remove_wrapper.commit());
    remove_wrapper.scope("test");
    dbg!(remove_wrapper.commit());
    remove_wrapper.index("userEmail", "user");
    dbg!(remove_wrapper.commit());
    /// 提交事务
    /// commit
    let res = db.commit(&mut remove_wrapper).await;
    dbg!(res.unwrap());
    Ok(())
}
```
