# DeleteWrapper

> DeleteWrapper的作用：帮助我们构造Delete语句
>
> DeleteWrapper：Help us construct the delete statement

DELETE语句可用于从数据库中删除记录。

The DELETE statement can be used to delete records from the database.

## function

| fun name        | params:type            | return             | des                                                          |
| --------------- | ---------------------- | ------------------ | ------------------------------------------------------------ |
| from            | table_name: &str       | &mut DeleteWrapper | 设置删除的表名<br />Set the delete table name                |
| id              | table_id: `TableId<T>` | &mut DeleteWrapper | 设置要删除的表的ID<br />Set the ID of the table to be deleted |
| where_condition | condition: &Criteria   | &mut DeleteWrapper | 构建where子句<br />build a where statement                   |
| timeout         | unit: TimeUnit         | &mut DeleteWrapper | 构建延时Timeout子句<br />build a timeout statement           |
| delete          | stmt: &str             | &mut DeleteWrapper | 通用插入自己写插入语句<br />Universal Insert Write Insert Statement on Your Own |
| return_none     |                        | &mut DeleteWrapper | 返回NONE<br />RETURN NONE                                    |
| return_diff     |                        | &mut DeleteWrapper | 返回DIFF<br />RETURN DIFF                                    |
| return_before   |                        | &mut DeleteWrapper | 返回BEFORE<br />RETURN BEFORE                                |
| return_after    |                        | &mut DeleteWrapper | 返回AFTER<br />RETURN AFTER                                  |
| return_field    | field_name: &str       | &mut DeleteWrapper | 返回某个字段<br />Return a certain field                     |

## Import

如果你想使用`DeleteWrapper`，你必须从`surrealism`导入`DeleteWrapper`和`Wrapper`

If you wanna use `DeleteWrapper` , you must import `DeleteWrapper` and `Wrapper` from `surrealism`

```rust
use surrealism::{Wrapper, DeleteWrapper};
```

## delete with define statement(easy)

### main.rs

```rust
use surrealism::{DefaultInitServiceImpl, SurrealRes, Wrapper, DeleteWrapper, UseWrapper};

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
    ///构建DeleteWrapper
    let mut delete_wrapper = DeleteWrapper::new();
    delete_wrapper.delete("DELETE user:t10086;");
    /// 提交语句
    /// commit statement
    let create_res = db.commit(&mut delete_wrapper).await;
    dbg!(create_res.unwrap());
    Ok(())
}
```

### res

```bash
[src\main.rs:39] create_res.unwrap() = Response(
    {
        0: Ok(
            [
                None,
            ],
        ),
    },
)

```

## delete easy

### main.rs

```rust
use surrealism::{DefaultInitServiceImpl, SurrealRes, Wrapper, DeleteWrapper, UseWrapper, Criteria, TimeUnit, TableId};

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
    /// 构建条件
    let mut condition = Criteria::new();
    condition.lt("age", "26");
    ///构建DeleteWrapper
    /// DELETE  user:2incz3ad74jlza71m0jq RETURN AFTER;
    let mut delete_wrapper = DeleteWrapper::new();
    delete_wrapper
        .from("user")
        .id(TableId::<String>::Str("2incz3ad74jlza71m0jq".to_string()))
        .return_after();
    /// 提交语句
    /// commit statement
    let create_res = db.commit(&mut delete_wrapper).await;
    dbg!(create_res.unwrap());
    Ok(())
}
```

## more examples

```rust
use surrealism::{DefaultInitServiceImpl, SurrealRes, Wrapper, DeleteWrapper, UseWrapper, Criteria, TimeUnit, TableId};
    ///构建DeleteWrapper
    /// DELETE  user:10086 WHERE age  <  26 RETURN AFTER TIMEOUT 5s;
    let mut delete_wrapper = DeleteWrapper::new();
    delete_wrapper
        .from("user:10086")
        .where_condition(&condition)
        .return_after()
        .timeout(5, TimeUnit::SECOND);
```
