# InsertWrapper

> InsertWrapper的作用：帮助我们构造Insert语句
>
> InsertWrapper：Help us construct the Insert statement

INSERT语句可用于将数据插入或更新到数据库中，使用与传统SQL INSERT语句相同的语句语法。

The INSERT statement can be used to insert or update data into the  database, using the same statement syntax as the traditional SQL Insert  statement.

## function

| fun name    | params:type              | return             | des                                                          |
| ----------- | ------------------------ | ------------------ | ------------------------------------------------------------ |
| insert_into | table_name: &str         | &mut InsertWrapper | 设置插入的表名<br />Set the inserted table name              |
| set         | key: &str<br /> value: T | &mut InsertWrapper | 插入单条记录单个字段<br />Insert a single record, a single field |
| insert_one  | obj: &T                  | &mut InsertWrapper | 使用非传统方式插入单条<br />Inserting a single record using non-traditional methods |
| insert_many | objs: `&Vec<T>`          | &mut InsertWrapper | 使用非传统方式插入多条<br />Inserting multiple records using non-traditional methods |
| insert      | stmt: &str               | &mut InsertWrapper | 通用插入自己写插入语句<br />Universal Insert Write Insert Statement on Your Own |

## Import

如果你想使用`InsertWrapper`，你必须从`surrealism`导入`InsertWrapper`和`Wrapper`

If you wanna use `InsertWrapper` , you must import `InsertWrapper` and `Wrapper` from `surrealism`

```rust
use surrealism::{Wrapper, InsertWrapper};
```

## Insert with traditional method (easy)

通过传统形式插入一条记录

Insert a record through traditional form

### main.rs

```rust
use surrealism::{DefaultInitServiceImpl, SurrealRes, Wrapper, InsertWrapper, UseWrapper};

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
    let res_use = db.use_commit(use_wrapper).await;
    dbg!(res_use);
    /// 构建InsertWrapper
    /// 通过键值对形式构建传统语句
    /// Constructing Traditional Statements through Key Value Pairs
    /// INSERT INTO user ( name , userId , age ) VALUES ( 'Kaye' , 'kaye001' , 56 );
    let mut insert_wrapper = InsertWrapper::new();
    insert_wrapper
        .insert_into("user")
        .set("name", "Kaye")
        .set("userId", "kaye001")
        .set("age", 56);
    /// 提交语句
    /// commit statement
    let create_res = db.commit(insert_wrapper).await;
    dbg!(create_res);
    Ok(())
}
```

### res

```bash
[src\main.rs:39] create_res = Ok(
    Response(
        {
            0: Ok(
                [
                    Object(
                        Object(
                            {
                                "age": Number(
                                    Int(
                                        56,
                                    ),
                                ),
                                "id": Thing(
                                    Thing {
                                        tb: "user",
                                        id: String(
                                            "reht5rqftwhljaon9cbb",
                                        ),
                                    },
                                ),
                                "name": Strand(
                                    Strand(
                                        "Kaye",
                                    ),
                                ),
                                "userId": Strand(
                                    Strand(
                                        "kaye001",
                                    ),
                                ),
                            },
                        ),
                    ),
                ],
            ),
        },
    ),
)

```
