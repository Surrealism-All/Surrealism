# UpdateWrapper

> UpdateWrapper的作用：帮助我们构造Update语句
>
> UpdateWrapper：Help us construct the Update statement

UPDATE语句可用于更新或修改数据库中的记录。

The UPDATE statement can be used to update or modify records in the database.

## function

| fun name        | params:type                                                  | return             | des                                                          |
| --------------- | ------------------------------------------------------------ | ------------------ | ------------------------------------------------------------ |
| from            | table_name: &str                                             | &mut UpdateWrapper | 设置更新的表名<br />Set the update table name                |
| id              | table_id: `TableId<T>`                                       | &mut UpdateWrapper | 设置要更新的表的ID<br />Set the ID of the table to be updated |
| set             | 1. field_name: &'static str<br /> 2. value: T`(<T: Serialize>)` | &mut UpdateWrapper | SET方式构建字段 <br />SET method for constructing fields     |
| set_with_sign   | 1. field_name: &'static str<br />2. value: T<br />3. sign: &str`(<T: Serialize>)` | &mut UpdateWrapper | 构建使用 += , -= 的set<br />build set which use +=,-=        |
| set_add         | 1. field_name: &'static str<br />2. value: T<br />`(<T: Serialize>)` | &mut UpdateWrapper | build +=                                                     |
| set_minus       | 1. field_name: &'static str<br />2. value: T<br />`(<T: Serialize>)` | &mut UpdateWrapper | build -=                                                     |
| set_condition   | 1. field_name: &'static str<br />2. condition: &mut IfElseWrapper | &mut UpdateWrapper | 通过使用IfElseWrapper构建SET<br />Build SET by using IfElseWrapper |
| content         | content_obj: T `(<T: Serialize>)`                            | &mut UpdateWrapper | CONTENT方式构建字段<br />CONTENT method for constructing fields |
| merge           | 1. key:&str<br />2. value:T`(<T: Serialize>)`                | &mut UpdateWrapper | 将新的文档合并到旧文档中,如果旧文档中存在相同的字段，则用新文档中相应字段的值来覆盖旧文档中的值。<br />Merge the new document into the old document, and if the same fields exist in the old document, overwrite the values in the old document with the values of the corresponding fields in the new document. |
| patch(不启用)   | value: &str                                                  | &mut UpdateWrapper | 用 JSON patch 的方式更新文档<br />Updating documents using JSON patches |
| where_condition | condition: &Criteria                                         | &mut UpdateWrapper | 构建where子句<br />build a where statement                   |
| timeout         | unit: TimeUnit                                               | &mut UpdateWrapper | 构建延时Timeout子句<br />build a timeout statement           |
| update          | stmt: &str                                                   | &mut UpdateWrapper | 通用更新,自己写语句<br />Universal Update, Write Update Statement on Your Own |
| return_none     |                                                              | &mut UpdateWrapper | 返回NONE<br />RETURN NONE                                    |
| return_diff     |                                                              | &mut UpdateWrapper | 返回DIFF<br />RETURN DIFF                                    |
| return_before   |                                                              | &mut UpdateWrapper | 返回BEFORE<br />RETURN BEFORE                                |
| return_after    |                                                              | &mut UpdateWrapper | 返回AFTER<br />RETURN AFTER                                  |
| return_field    | field_name: &str                                             | &mut UpdateWrapper | 返回某个字段<br />Return a certain field                     |

## Import

如果你想使用`UpdateWrapper`，你必须从`surrealism`导入`UpdateWrapper`和`Wrapper`

If you wanna use `UpdateWrapper` , you must import `UpdateWrapper` and `Wrapper` from `surrealism`

```rust
use surrealism::{Wrapper, UpdateWrapper};
```

## Update with SET

### main.rs

```rust
use surrealism::{InitServiceImpl, SurrealRes, Wrapper, UpdateWrapper, UseWrapper,TableId, CreateWrapper};

#[tokio::main]
async fn main() -> SurrealRes<()> {
    ///初始化连接
    ///init connection
    let db = InitServiceImpl::new().init().unwrap();
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
    /// 准备数据
    /// prepare data
    let mut create_wrapper = CreateWrapper::new();
    create_wrapper.create("user")
        .id(TableId::<String>::Str("1008".to_string()))
        .set("name", "Kanye")
        .set("age", 36)
        .return_none();
    let create_res = db.commit(create_wrapper).await;
    dbg!(create_res);
    /// 构建UpdateWrapper
    /// UPDATE user:1008 SET age = 6 RETURN AFTER;
    let mut update_wrapper = UpdateWrapper::new();
    update_wrapper
        .from("user")
        .id(TableId::<String>::Str("1008".to_string()))
        .set("age", 6)
        .return_after();
    /// 提交语句
    /// commit statement
    let update_res = db.commit(update_wrapper).await;
    dbg!(update_res.unwrap());
    Ok(())
}
```

### res

```bash
[src\main.rs:47] update_res.unwrap() = Response(
    {
        0: Ok(
            [
                Object(
                    Object(
                        {
                            "age": Number(
                                Int(
                                    6,
                                ),
                            ),
                            "id": Thing(
                                Thing {
                                    tb: "user",
                                    id: Number(
                                        1008,
                                    ),
                                },
                            ),
                            "name": Strand(
                                Strand(
                                    "Kanye",
                                ),
                            ),
                        },
                    ),
                ),
            ],
        ),
    },
)

```

## Update with CONTENT

### main.rs

```rust
use surrealism::{InitServiceImpl, SurrealRes, Wrapper, UpdateWrapper, UseWrapper, TableId};
use serde::{Serialize, Deserialize};

///构建结构体,需要使用serde提供的宏:Serialize, Deserialize
/// build struct,Need to use the macro provided by serde::{Serialize, Deserialize}
#[derive(Debug, Serialize, Deserialize)]
struct User {
    pub name: String,
    pub age: usize,
}

#[tokio::main]
async fn main() -> SurrealRes<()> {
    ///初始化连接
    ///init connection
    let db = InitServiceImpl::new().init().unwrap();
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
    /// 准备数据
    /// prepare data
    let data = User {
        name: "Mask".to_string(),
        age: 16,
    };
    /// 构建UpdateWrapper
    /// UPDATE user:1008 CONTENT {name:'Mask',age:16} RETURN AFTER;
    let mut update_wrapper = UpdateWrapper::new();
    update_wrapper
        .from("user")
        .id(TableId::<String>::Str("1008".to_string()))
        .content(&data)
        .return_after();
    /// 提交语句
    /// commit statement
    let update_res = db.commit(update_wrapper).await;
    dbg!(update_res.unwrap());
    Ok(())
}
```

### res

```bash
[src\main.rs:53] update_res.unwrap() = Response(
    {
        0: Ok(
            [
                Object(
                    Object(
                        {
                            "age": Number(
                                Int(
                                    16,
                                ),
                            ),
                            "id": Thing(
                                Thing {
                                    tb: "user",
                                    id: Number(
                                        1008,
                                    ),
                                },
                            ),
                            "name": Strand(
                                Strand(
                                    "Mask",
                                ),
                            ),
                        },
                    ),
                ),
            ],
        ),
    },
)

```

## Update with MERGE

### main.rs

```rust
use surrealism::{InitServiceImpl, SurrealRes, Wrapper, UpdateWrapper, UseWrapper, TableId};

#[tokio::main]
async fn main() -> SurrealRes<()> {
    ///初始化连接
    ///init connection
    let db = InitServiceImpl::new().init().unwrap();
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
    /// 构建UpdateWrapper
    /// UPDATE user:1008 MERGE { name:'StuWie' , age:3 } RETURN AFTER;
    let mut update_wrapper = UpdateWrapper::new();
    update_wrapper
        .from("user")
        .id(TableId::<String>::Str("1008".to_string()))
        .merge("name", "StuWie")
        .merge("age",3)
        .return_after();

    /// 提交语句
    /// commit statement
    let update_res = db.commit(update_wrapper).await;
    dbg!(update_res.unwrap());
    Ok(())
}
```

### res

```bash
[src\main.rs:49] update_res.unwrap() = Response(
    {
        0: Ok(
            [
                Object(
                    Object(
                        {
                            "age": Number(
                                Int(
                                    3,
                                ),
                            ),
                            "id": Thing(
                                Thing {
                                    tb: "user",
                                    id: Number(
                                        1008,
                                    ),
                                },
                            ),
                            "name": Strand(
                                Strand(
                                    "StuWie",
                                ),
                            ),
                        },
                    ),
                ),
            ],
        ),
    },
)

```

```bash
//create---------------------------------------------------------
[src\main.rs:37] data_res.unwrap() = Response(
    {
        0: Ok(
            [
                Object(
                    Object(
                        {
                            "age": Number(
                                Int(
                                    16,
                                ),
                            ),
                            "id": Thing(
                                Thing {
                                    tb: "user",
                                    id: Number(
                                        101,
                                    ),
                                },
                            ),
                            "name": Strand(
                                Strand(
                                    "Jack",
                                ),
                            ),
                            "railcard": Strand(
                                Strand(
                                    "none",
                                ),
                            ),
                        },
                    ),
                ),
            ],
        ),
    },
)
//after update--------------------------------------------------
[src\main.rs:57] res.unwrap() = Response(
    {
        0: Ok(
            [
                Object(
                    Object(
                        {
                            "age": Number(
                                Int(
                                    16,
                                ),
                            ),
                            "id": Thing(
                                Thing {
                                    tb: "user",
                                    id: Number(
                                        101,
                                    ),
                                },
                            ),
                            "name": Strand(
                                Strand(
                                    "Jack",
                                ),
                            ),
                            "railcard": Strand(
                                Strand(
                                    "student",
                                ),
                            ),
                        },
                    ),
                ),
            ],
        ),
    },
)
```

## Update with if

```rust
use surrealism::{InitServiceImpl, SurrealRes, Wrapper, UpdateWrapper, UseWrapper, IfElseWrapper, Criteria, CreateWrapper, TableId};

#[tokio::main]
async fn main() -> SurrealRes<()> {
    ///初始化连接
    ///init connection
    let db = InitServiceImpl::new().init().unwrap();
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
    /// 准备数据
    /// prepare data
    let mut data = CreateWrapper::new();
    data
        .create("user")
        .id(TableId::<String>::Str("101".to_string()))
        .set("name", "Jack")
        .set("age", 16)
        .set("railcard", "none")
        .return_after();
    let data_res = db.commit(data).await;
    dbg!(data_res.unwrap());
    ///条件
    let mut cri1 = Criteria::new();
    cri1.lte("age", "10");
    let mut cri2 = Criteria::new();
    cri2.lte("age", "21");
    ///IF—ELSE
    let mut if_wrapper = IfElseWrapper::new();
    if_wrapper
        .if_condition_str(&cri1, "junior")
        .else_if_condition_str(&cri2, "student")
        .else_condition_str("senior");
    ///构建Wrapper
    let mut update_wrapper = UpdateWrapper::new();
    update_wrapper
        .from("user")
        .set_condition("railcard", &mut if_wrapper)
        .return_after();
    /// 提交事务
    let res = db.commit(update_wrapper).await;
    dbg!(res.unwrap());
    Ok(())
}
```

