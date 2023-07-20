# CreateWrapper

> CreateWrapper的作用：帮助我们构造Create语句
>
> CreateWrapper：Help us construct the Create statement

CREATE语句可用于向数据库添加记录（如果这些记录尚不存在）

The CREATE statement can be used to add records to the database (if these records do not already exist)

## function

| fun name      | params:type                                            | return             | des                                                          |
| ------------- | ------------------------------------------------------ | ------------------ | ------------------------------------------------------------ |
| create        | table_name: &str                                       | &mut CreateWrapper | 创建表名称<br />create table name                            |
| id            | table_id: `TableId<T>`                                 | &mut CreateWrapper | 创建表的ID , ID使用TableId进行构建!<br />create table with id , use TableId enum to create! |
| set           | 1. field_name: &str<br />2. value: T(`<T: Serialize>`) | &mut CreateWrapper | SET方式构建字段<br />SET method for constructing fields      |
| content       | content_obj: T (`<T: Serialize`)                       | &mut CreateWrapper | CONTENT方式构建字段<br />CONTENT method for constructing fields |
| return_none   |                                                        | &mut CreateWrapper | 返回NONE<br />RETURN NONE                                    |
| return_diff   |                                                        | &mut CreateWrapper | 返回DIFF<br />RETURN DIFF                                    |
| return_before |                                                        | &mut CreateWrapper | 返回BEFORE<br />RETURN BEFORE                                |
| return_after  |                                                        | &mut CreateWrapper | 返回AFTER<br />RETURN AFTER                                  |
| return_field  | field_name: &str                                       | &mut CreateWrapper | 返回某个字段<br />Return a certain field                     |

## Import

### set statement

如果你想使用`CreateWrapper`，你必须从`surrealism`导入`CreateWrapper`，`Wrapper`，`TableId`

If you wanna use `CreateWrapper` , you must import `CreateWrapper` ，`Wrapper` ，`TableId`from `surrealism`

```rust
use surrealism::{Wrapper, CreateWrapper, TableId};
```

### content statement

如果使用content语句来构建则需要从`surrealism`导入`Wrapper, CreateWrapper, TableId`和`serde`中的`Serialize, Deserialize`

If using the content statement to build, it needs to be imported：`Wrapper, CreateWrapper, TableId` import`Serialize, Deserialize`from serde

```
use surrealism::{Wrapper, CreateWrapper, TableId};
use serde::{Serialize, Deserialize};
```

## use SET

让我使用CreateWrapper构建如下语句（Let me use CreateWrapper to build the following statement）：

`CREATE user:t10086 SET name='Jack',userId='jack001' RETURN NONE;`

### main.rs

```rust
use surrealism::{DefaultInitServiceImpl, SurrealRes, UseWrapper, Wrapper, CreateWrapper, TableId};

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
    ///创建CreateWrapper
    /// new CreateWrapper
    let mut create_wrapper = CreateWrapper::new();
    /// 设置构建语句
    /// set create statement
    /// CREATE user:t10086 SET name='Jack',userId='jack001' RETURN NONE;
    create_wrapper.create("user")
        .id(TableId::<String>::Str("t10086".to_string()))
        .set("name", "Jack")
        .set("userId", "jack001")
        .return_none();
    /// 提交语句
    /// commit statement
    let create_res = db.commit(&mut create_wrapper).await;
    dbg!(create_res.unwrap());
    Ok(())
}
```

### res

```bash
[src\main.rs:33] create_res.unwrap() = Response(
    {
        0: Ok(
            [],
        ),
    },
)

```

## use CONTENT

让我使用CreateWrapper构建如下语句（Let me use CreateWrapper to build the following statement）：

`CREATE user:t10088 CONTENT {userId:'mark001',name:'Mark',email:'mark@outlook.com'} RETURN AFTER;`

### main.rs

```rust
use surrealism::{DefaultInitServiceImpl, SurrealRes, UseWrapper, Wrapper, CreateWrapper, TableId};
use serde::{Serialize, Deserialize};

///构建结构体,需要使用serde提供的宏:Serialize, Deserialize
/// build struct,Need to use the macro provided by serde::{Serialize, Deserialize}
#[derive(Debug, Serialize, Deserialize)]
struct User {
    pub userId: String,
    pub name: String,
    pub email: String,
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
    ///创建CreateWrapper
    /// new CreateWrapper
    let mut create_wrapper = CreateWrapper::new();
    /// 设置构建语句
    /// set create statement
    /// CREATE user:t10088 CONTENT {userId:'mark001',name:'Mark',email:'mark@outlook.com'} RETURN AFTER;
    create_wrapper.create("user")
        .id(TableId::<String>::Str("t10088".to_string()))
        .content(User {
            userId: String::from("mark001"),
            name: String::from("Mark"),
            email: String::from("mark@outlook.com"),
        })
        .return_after();
    /// 提交语句
    /// commit statement
    let create_res = db.commit(&mut create_wrapper).await;
    dbg!(create_res.unwrap());
    Ok(())
}
```

### res

```bash
[src\main.rs:51] create_res.unwrap() = Response(
    {
        0: Ok(
            [
                Object(
                    Object(
                        {
                            "email": Strand(
                                Strand(
                                    "mark@outlook.com",
                                ),
                            ),
                            "id": Thing(
                                Thing {
                                    tb: "user",
                                    id: String(
                                        "t10088",
                                    ),
                                },
                            ),
                            "name": Strand(
                                Strand(
                                    "Mark",
                                ),
                            ),
                            "userId": Strand(
                                Strand(
                                    "mark001",
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
