# UseWrapper

> UseWrapper的作用是：帮助我们构建使用命名空间和数据库
>
> UseWrapper：Help us build and use namespaces and databases

USE语句指定用于后续SurrealQL语句的命名空间和/或数据库。

The USE statement specifies a namespace and / or a database to use for the subsequent SurrealQL statements.

## function

| fun name | params:type     | return          | des                                                          |
| -------- | --------------- | --------------- | ------------------------------------------------------------ |
| use_ns   | namespace: &str | &mut UseWrapper | 设置SurrealDB使用的命名空间<br />Set the namespace used by SurrealDB |
| use_db   | database: &str  | &mut UseWrapper | 设置SurrealDB使用的数据库<br />Set up the database used by SurrealDB |

## Import

如果你想使用`UseWrapper`，你必须从`surrealism`导入`UseWrpper`和`Wrapper`

If you wanna use `UseWrapper` , you must import `UseWrapper` and `Wrapper` from `surrealism`

```rust
use surrealism::{UseWrapper, Wrapper};
```

## main.rs

```rust
use surrealism::{DefaultInitServiceImpl, SurrealRes, UseWrapper, Wrapper};

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
    Ok(())
}
```

## res

```bash
[src\main.rs:38] res_use = Ok(
    (),
)
```

## commit

Use语句的提交和其他语句并不相同，而是使用`SurrealDB(struct)`提供的`use_commit()`方法提交语句

The submission of the Use statement is not the same as other statements，we should use `use_commit()` which support by `SurrealDB(struct)`