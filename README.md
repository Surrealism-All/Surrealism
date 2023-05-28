# Surrealism

- author：syf20020816@outlook.com
- docName：Surrealism README
- createDate：20230506
- updateDate：20230528
- version：0.0.1
- des-tag：test
- email：syf20020816@outlook.com

## Surrealism Introduction

<img src="./README/imgs/logo.png" />

Surrealism依托于Surrealdb提供的Rust官方标准库:surrealdb,目的是构建一种更加统一，简单的方式对Surrealdb数据库进行各类操作

Surrealism relies on Surrealdb's official Rust standard library:surrealdb,The purpose is to build a more unified and simple way to perform various operations on Surrealdb database

## QuickStart

### add dependencies

```toml
[dependencies]
surrealism = {path="../../surrealism"}
tokio = { version = "1.28.0", features = ["macros", "rt-multi-thread"] }
surrealdb = "1.0.0-beta.9"
serde = { version = "1.0.162", features = ["derive"] }

```

### add configuration
配置：
- surreal:单机本地连接Single还是分布式连接Multi
- username:用户名
- password:密码
- url:连接地址
- port:连接端口
- mode:连接模式（Memory:表示内存,File:表示存到文件中）
- path:存储到文件中的文件地址，使用Memory设置为""即可
- logLevel:日志级别(error,warn,info,debug,trace)

可采用JSON或TOML两种配置文件方式

设置配置文件地址可以是：
- ./Surrealism.toml
- ./configs/Surrealism.toml
- ./templates/Surrealism.toml

<hr>

configuration：

- surreal:Single machine local connection or distributed connection Multi
- username:your username
- password:your password
- url:connection url
- port:connection port
- mode:connection mode（Memory:Instructions for storing in memory,File:Instructions stored in files）
- path:The file address stored in the file can be set to "" using Memory
- logLevel:log level(error,warn,info,debug,trace)

Two configuration file methods can be used: JSON or TOML

The configuration file address can be set to：

- ./Surrealism.toml
- ./configs/Surrealism.toml
- ./templates/Surrealism.toml

#### Surrealism.json(JSON)
```json
{
  "surreal": "Single",
  "username": "root",
  "password": "syf20020816",
  "url": "127.0.0.1",
  "port": "10086",
  "mode": "Memory",
  "path": "E:/Rust/surreal",
  "logLevel": "info"
}
```
#### Surrealism.toml(TOML)
```toml
[default]
surreal = "Single"
username = "root"
password = "syf20020816"
url = "127.0.0.1"
port = "10086"
mode = "Memory"
path = "E:/Rust/surreal"
logLevel="info"
```

### main.rs

```rust
use surrealism::{InitServiceImpl, SurrealRes};

#[tokio::main]
async fn main() -> SurrealRes<()> {
    ///初始化连接
    ///init connection
    let db = InitServiceImpl::new().init().unwrap();
    Ok(())
}

```

### terminal res

```bash
   ▄▄▄▄                                                      ▄▄▄▄         ██
 ▄█▀▀▀▀█                                                     ▀▀██         ▀▀
 ██▄       ██    ██   ██▄████   ██▄████   ▄████▄    ▄█████▄    ██       ████     ▄▄█████▄  ████▄██▄
  ▀████▄   ██    ██   ██▀       ██▀      ██▄▄▄▄██   ▀ ▄▄▄██    ██         ██     ██▄▄▄▄ ▀  ██ ██ ██
      ▀██  ██    ██   ██        ██       ██▀▀▀▀▀▀  ▄██▀▀▀██    ██         ██      ▀▀▀▀██▄  ██ ██ ██
 █▄▄▄▄▄█▀  ██▄▄▄███   ██        ██       ▀██▄▄▄▄█  ██▄▄▄███    ██▄▄▄   ▄▄▄██▄▄▄  █▄▄▄▄▄██  ██ ██ ██
  ▀▀▀▀▀     ▀▀▀▀ ▀▀   ▀▀        ▀▀         ▀▀▀▀▀    ▀▀▀▀ ▀▀     ▀▀▀▀   ▀▀▀▀▀▀▀▀   ▀▀▀▀▀▀   ▀▀ ▀▀ ▀▀

2023-05-28T08:41:12.568Z INFO  [surrealism::creator::services::init_service] Configuration Initialization over(配置初始化完成)
2023-05-28T08:41:12.568Z INFO  [surrealism::creator::services::init_service] Connection Initialization start(初始化连接检测开始)
2023-05-28T08:41:12.579Z INFO  [surrealism::creator::services::init_service] Version {
    router: Ok(
        Router {
            conn: PhantomData<surrealdb::api::engine::remote::ws::Client>,
            sender: Sender,
            last_id: 2,
            features: {
                Auth,
            },
        },
    ),
}
2023-05-28T08:41:12.609Z INFO  [surrealism::creator::services::init_service] Connection Initialization over , Pay attention to checking the connection detection information above(初始化连接检测结束,注意查看上方连接检测信息)
```

## Add your Logo！

我们可以在根目录中添加banner.txt来添加自己的Logo

We can add banner.txt to the root directory to add our own logo

<img src="./README/imgs/image-20230528171345821.svg" />

### banner.txt

```
             ,        ,
            /(_,    ,_)\
            \ _/    \_ /
            //        \\
            \\ (@)(@) //
             \'="=="='/
         ,===/        \===,
        ",===\        /===,"
        " ,==='------'===, "
         "                "

```

## UseWrapper

> UseWrapper的作用是：帮助我们构建使用命名空间和数据库
>
> UseWrapper：Help us build and use namespaces and databases

### function

| fun name | params:type     | return          | des                                                          |
| -------- | --------------- | --------------- | ------------------------------------------------------------ |
| use_ns   | namespace: &str | &mut UseWrapper | 设置SurrealDB使用的命名空间<br />Set the namespace used by SurrealDB |
| use_db   | database: &str  | &mut UseWrapper | 设置SurrealDB使用的数据库<br />Set up the database used by SurrealDB |

### Import

如果你想使用`UseWrapper`，你必须从`surrealism`导入`UseWrpper`和`Wrapper`

If you wanna use `UseWrapper` , you must import `UseWrapper` and `Wrapper` from `surrealism`

```rust
use surrealism::{UseWrapper, Wrapper};
```

### main.rs

```rust
use surrealism::{InitServiceImpl, SurrealRes, UseWrapper, Wrapper};

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
    Ok(())
}
```

### res

```bash
[src\main.rs:38] res_use = Ok(
    (),
)
```

## CreateWrapper

> CreateWrapper的作用：帮助我们构造Create语句
>
> CreateWrapper：Help us construct the Create statement

### function

| fun name      | params:type                                            | return             | des                                                          |
| ------------- | ------------------------------------------------------ | ------------------ | ------------------------------------------------------------ |
| create        | table_name: &str                                       | &mut CreateWrapper | 创建表名称<br />create table name                            |
| id            | table_id: `TableId<T>`                                 | &mut CreateWrapper | 创建表的ID , ID使用TableId进行构建!<br />create table with id , use TableId enum to create! |
| set           | 1. field_name: &str<br />2. value: T(`<T: Serialize>`) | &mut CreateWrapper | SET方式构建字段<br />SET method for constructing fields      |
| content       | content_obj: T (`<T: Serialize + SQLParser>`)          | &mut CreateWrapper | CONTENT方式构建字段<br />CONTENT method for constructing fields |
| return_none   |                                                        | &mut CreateWrapper | 返回NONE<br />RETURN NONE                                    |
| return_diff   |                                                        | &mut CreateWrapper | 返回DIFF<br />RETURN DIFF                                    |
| return_before |                                                        | &mut CreateWrapper | 返回BEFORE<br />RETURN BEFORE                                |
| return_after  |                                                        | &mut CreateWrapper | 返回AFTER<br />RETURN AFTER                                  |
| return_field  | field_name: &str                                       | &mut CreateWrapper | 返回某个字段<br />Return a certain field                     |

### Import

#### set statement

如果你想使用`CreateWrapper`，你必须从`surrealism`导入`CreateWrapper`，`Wrapper`，`TableId`

If you wanna use `CreateWrapper` , you must import `CreateWrapper` ，`Wrapper` ，`TableId`from `surrealism`

```rust
use surrealism::{Wrapper, CreateWrapper, TableId};
```

#### content statement

如果使用content语句来构建则需要从`surrealism`导入`Wrapper, CreateWrapper, TableId, ParseSQL, SQLParser`和`serde`中的`Serialize, Deserialize`

If using the content statement to build, it needs to be imported：`Wrapper, CreateWrapper, TableId, ParseSQL, SQLParser`from `surrealism`and import`Serialize, Deserialize`from serde

```
use surrealism::{Wrapper, CreateWrapper, TableId, ParseSQL, SQLParser};
use serde::{Serialize, Deserialize};
```

### use SET

让我使用CreateWrapper构建如下语句（Let me use CreateWrapper to build the following statement）：

`CREATE user:t10086 SET name='Jack',userId='jack001' RETURN NONE;`

#### main.rs

```rust
use surrealism::{InitServiceImpl, SurrealRes, UseWrapper, Wrapper, CreateWrapper, TableId};

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
    let create_res = db.commit(create_wrapper).await;
    dbg!(create_res.unwrap());
    Ok(())
}
```

#### res

```bash
[src\main.rs:33] create_res.unwrap() = Response(
    {
        0: Ok(
            [],
        ),
    },
)

```

### use CONTENT

让我使用CreateWrapper构建如下语句（Let me use CreateWrapper to build the following statement）：

`CREATE user:t10088 CONTENT {userId:'mark001',name:'Mark',email:'mark@outlook.com'} RETURN AFTER;`

#### main.rs

```rust
use surrealism::{InitServiceImpl, SurrealRes, UseWrapper, Wrapper, CreateWrapper, TableId, ParseSQL, SQLParser};
use serde::{Serialize, Deserialize};

///构建结构体,需要使用surrealism提供的宏:ParseSQL
/// build struct,Need to use the macro provided by surrealism: ParseSQL
/// 请注意:ParseSQl宏和SQLParser trait要同时导入
/// Please note that the ParseSQl macro and SQLParser trait need to be imported simultaneously
#[derive(Debug, Serialize, Deserialize, ParseSQL)]
struct User {
    pub userId: String,
    pub name: String,
    pub email: String,
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
    let create_res = db.commit(create_wrapper).await;
    dbg!(create_res.unwrap());
    Ok(())
}
```

#### res

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



## SelectWrapper

### function

| fun name        | params:type                            | return          | des                                                          |
| --------------- | -------------------------------------- | --------------- | ------------------------------------------------------------ |
| select          | query: &str                            | &mut UseWrapper | 通用查询，需要自己写SQL语句<br />Universal query, requires writing SQL statements yourself |
| select_fields   | field: &Field                          | &mut UseWrapper | 设置查询字段<br />Set select fields                          |
| select_field    | fields: `&Vec<Field>`                  | &mut UseWrapper | 设置单个查询字段<br />Set select field （use `"*"` means select * from ...） |
| from            | table_name: &str                       | &mut UseWrapper | 设置查询的表<br />set table name which you wanna select      |
| where_condition | condition: &Criteria                   | &mut UseWrapper | 构建where子句<br />build a where statement                   |
| order_by        | conditions: `&mut Vec<OrderCondition>` | &mut UseWrapper | 构建OrderBy子句<br />build an OrderBy statement              |
| group_by        | conditions: &Vec<&str>                 | &mut UseWrapper | 构建GroupBy子句<br />build a GroupBy statement               |
| split_at        | conditions: &Vec<&str>                 | &mut UseWrapper | 构建SplitAt子句<br />build a SplitAt statement               |
| fetch           | fields: &Vec<&str>                     | &mut UseWrapper | 构建Fetch子句<br />build a Fetch statement                   |
| timeout         | 1. time: usize<br />2. unit: TimeUnit  | &mut UseWrapper | 构建延时Timeout子句<br />build a timeout statement           |
| limit_by        | pieces: usize                          | &mut UseWrapper | 构建limit子句<br />build an Limit statement                  |
| start_at        | pieces: usize                          | &mut UseWrapper | 构建Start子句<br />build a Start statement                   |

### Import

如果你想使用`SelectWrapper`，你必须从`surrealism`导入`SelectWrapper`和`Wrapper`

If you wanna use `SelectWrapper` , you must import `SelectWrapper` and `Wrapper` from `surrealism`

```rust
use surrealism::{Wrapper, SelectWrapper};
```

### select with whole statement

```rust
use surrealism::{InitServiceImpl, SurrealRes, UseWrapper, Wrapper, SelectWrapper};

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
    /// 最简单的查询也是最复杂的查询,需要自己手写SQL语句
    /// 用于构建一些SelectWrapper目前做不到的查询
    /// 例如:SELECT count() AS total, math::sum(age), gender, country FROM person GROUP BY gender, country;
    /// The simplest and most complex queries require handwritten SQL statements
    /// Used to build queries that SelectWrapper currently cannot perform
    /// example:SELECT count() AS total, math::sum(age), gender, country FROM person GROUP BY gender, country;
    let mut select_wrapper = SelectWrapper::new();
    select_wrapper.select("SELECT * FROM user");
    /// 提交语句
    /// commit statement
    let create_res = db.commit(select_wrapper).await;
    dbg!(create_res.unwrap());
    Ok(())
}
```

### res

```bash
[src\main.rs:49] create_res.unwrap() = Response(
    {
        0: Ok(
            [
                Object(
                    Object(
                        {
                            "id": Thing(
                                Thing {
                                    tb: "user",
                                    id: String(
                                        "t10086",
                                    ),
                                },
                            ),
                            "name": Strand(
                                Strand(
                                    "Jack",
                                ),
                            ),
                            "userId": Strand(
                                Strand(
                                    "jack001",
                                ),
                            ),
                        },
                    ),
                ),
                Object(
                    Object(
                        {
                            "id": Thing(
                                Thing {
                                    tb: "user",
                                    id: String(
                                        "t10087",
                                    ),
                                },
                            ),
                            "name": Strand(
                                Strand(
                                    "Rose",
                                ),
                            ),
                            "userId": Strand(
                                Strand(
                                    "rose001",
                                ),
                            ),
                        },
                    ),
                ),
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

### common use

#### main.rs

```rust
use surrealism::{InitServiceImpl, SurrealRes, UseWrapper, Wrapper, SelectWrapper, Field, Criteria};


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
    ///准备查询条件
    /// prepare select conditions
    let mut field1 = Field::new("userId");
    field1.as_name("stuId");
    let field2 = Field::new("name");
    let fields = vec![field1, field2];
    let mut condition = Criteria::new();
    condition.eq("name", "'Rose'");

    /// 构建SelectWrapper
    /// SELECT userId AS stuId , name FROM user WHERE name = 'Rose' ;
    let mut select_wrapper = SelectWrapper::new();
    select_wrapper.select_fields(&fields)
        .from("user")
        .where_condition(&condition);

    /// 提交语句
    /// commit statement
    let create_res = db.commit(select_wrapper).await;
    dbg!(create_res.unwrap());
    Ok(())
}
```

#### res

```
[src\main.rs:57] create_res.unwrap() = Response(
    {
        0: Ok(
            [
                Object(
                    Object(
                        {
                            "name": Strand(
                                Strand(
                                    "Rose",
                                ),
                            ),
                            "stuId": Strand(
                                Strand(
                                    "rose001",
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

### more example

import `use surrealism::{OrderCondition, Ordered,TimeUnit}`

```rust
    let mut f_v = Vec::new();
    let mut f1 = Field::new("userId");
    f1.as_name("stuID");
    let mut f2 = Field::new("name");
    f2.as_name("stuName");
    f_v.push(f1);
    f_v.push(f2);
    let mut cri = Criteria::new();
    cri.gte(&cri.and(&cri.or(&cri.and("a", "b"), "c"), "d"), "12345");
    let handles = vec!["userId", "name"];
    let mut order_handles = Vec::new();
    let mut order_handle1 = OrderCondition::new_no_args();
    order_handle1.field("name").ordered(Ordered::DESC);
    let mut order_handle2 = OrderCondition::new_no_args();
    order_handle2.field("userId").ordered(Ordered::ASC);
    order_handles.push(order_handle1);
    order_handles.push(order_handle2);
    //查询
    queryWrapper.select_fields(&f_v)
        .from("user")
        .where_condition(&cri)
        .group_by(&handles)
        .split_at(&handles)
        .order_by(&mut order_handles)
        .start_at(15)
        .limit_by(30)
        .fetch(&vec!["user.name"])
        .timeout(50, TimeUnit::SECOND);
```

