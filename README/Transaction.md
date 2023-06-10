# Transaction

> Transaction的作用是：帮助我们构建使用命名空间和数据库
>
> Transaction：Help us build and use namespaces and databases

USE语句指定用于后续SurrealQL语句的命名空间和/或数据库。

The USE statement specifies a namespace and / or a database to use for the subsequent SurrealQL statements.

## function

| fun name  | params:type             | return           | des                                                          |
| --------- | ----------------------- | ---------------- | ------------------------------------------------------------ |
| add       | item: &mut impl Wrapper | &mut Transaction | 添加一条语句<br />Add a statement                            |
| add_stmt  | stmt: &str              | &mut Transaction | 添加一条语句<br />Add a statement                            |
| add_stmts | stmts: Vec<&str>        | &mut Transaction | 添加一组语句<br />Add a set of statements                    |
| get       | index: usize            | &str             | 根据索引获取一条语句<br />Retrieve a statement based on the index |
| commit    |                         | （）             | 提交一组事务<br />Commit a set of transactions               |
| cancel    |                         | （）             | 取消一组事务<br />Cancel a set of transactions               |

## Import

如果你想使用`Transaction`，你必须从`surrealism`导入`Transaction`和`Wrapper`

If you wanna use `Transaction` , you must import `Transaction` and `Wrapper` from `surrealism`

```rust
use surrealism::{Transaction, Wrapper};
```

## main.rs

```rust
use surrealism::{DefaultInitServiceImpl, SurrealRes, Wrapper, SelectWrapper, UseWrapper, Transaction};

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
    ///构建Wrapper
    let mut select_wrapper1 = SelectWrapper::new();
    select_wrapper1.select("SELECT name FROM user;");
    let mut select_wrapper2 = SelectWrapper::new();
    select_wrapper2.select("SELECT * FROM user;");
    let mut select_wrapper3 = SelectWrapper::new();
    select_wrapper3.select("SELECT userId FROM user;");
    ///构建事务
    let mut transaction = Transaction::new();
    transaction
        .add(&mut select_wrapper1)
        .add(&mut select_wrapper2)
        .add(&mut select_wrapper3)
        .commit();
    dbg!(transaction.get(1));
    /// 提交事务
    let res = db.commit_transaction(&transaction).await;
    dbg!(res.unwrap());
    Ok(())
}
```

## commit

事务的提交和其他语句并不相同，而是使用`SurrealDB(struct)`提供的`commit_transaction()`方法提交语句

The submission of the Use statement is not the same as other statements，we should use `commit_transaction()` which support by `SurrealDB(struct)`