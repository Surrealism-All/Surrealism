<img src="https://img.shields.io/badge/surrealism-0.3.0-orange?style=flat-square&logo=rust&logoColor=%23fff&labelColor=%23DEA584&color=%23DEA584">  <img src="https://img.shields.io/badge/License-MIT-orange?style=flat-square&logoColor=%23fff&labelColor=%2323B898&color=%2323B898">

# Surrealism

- author：syf20020816@outlook.com
- docName：Surrealism README
- createDate：20230506
- updateDate：20231024
- version：0.3.0
- email：syf20020816@outlook.com

## LICEMSE

MIT

## Surrealism Introduction

<img src="https://github.com/Surrealism-All/Surrealism/blob/0.3.0/README/imgs/logo.png" />

Surrealism依托于Surrealdb提供的Rust官方标准库:surrealdb,目的是构建一种更加统一，简单的方式对Surrealdb数据库进行各类操作

Surrealism relies on Surrealdb's official Rust standard library:surrealdb,The purpose is to build a more unified and simple way to perul various operations on Surrealdb database

## QuickStart

### add dependencies

```toml
[dependencies]
surrealism = {version="0.3.0"}
tokio = { version = "1.28.0", features = ["macros", "rt-multi-thread"] }
```

### add configuration
配置：

```
username:用户名
password:密码
local:本机连接(本机使用ws,远程使用wss)
bind: 连接地址,
auth:开启权限认证
tick_interval:运行节点代理tick的间隔（包括垃圾收集），默认为10秒
strict:严格模式
mode:连接模式（Memory表示内存File表示存到文件中，Tikv表示tikv集群地址）
path:存储到文件中的文件地址，使用Memory则无需设置
log:日志级别
query_timeout:设置查询超时时间
transaction_timeout: 事务超时时间
no_banner: 打印Banner
db_connection: 数据库连接行为
http_server: 服务器行为
capabilities: 能力
```

可采用JSON或TOML两种配置文件方式

设置配置文件地址可以是：
- ./Surrealism.toml
- ./configs/Surrealism.toml
- ./templates/Surrealism.toml

<hr>

configuration：

```
username: db username
password: db password
local: Local connection (using ws locally, using wss remotely)
bind: Connection address,
auth:Enable permission authentication
tick_interval:The interval between running node agent tickets (including garbage collection), which defaults to 10 seconds
strict:strict mode
mode:Connection mode (Memory represents memory, File represents storage to file, Tikv represents Tikv cluster address)
path:The file address stored in the file, which does not need to be set when using Memory
log:log level
query_timeout:Set query timeout time
transaction_timeout: Transaction timeout time
no_banner: Print Banner
db_connection: database connection behavior
http_server: server behavior
capabilities: db Capabilities
```

Two configuration file methods can be used: JSON or TOML

The configuration file address can be set to：

- ./Surrealism.toml
- ./configs/Surrealism.toml
- ./templates/Surrealism.toml

> ❗note：当前SurrealDB仍处于开发阶段，对应连接鉴权方式：NS和DB并为支持，所以使用Surrealism进行配置时请以Root方式进行鉴权连接，不要设置ns和db！
>
> ❗note：Currently, SurrealDB is still in the development stage, and the corresponding connection authentication methods are supported: NS and DB. Therefore, when using Surrealsm for configuration, please use Root mode for authentication connections and do not set ns and db!

#### Surrealism.json(JSON)
```json
{
	"username" : "root"
	"password" : "syf20020816"
	"bind" : "127.0.0.1:10086"
	"mode" : "Memory"
	"log" : "Info",
    "local": true
}
```
#### Surrealism.toml(TOML)
```toml
[default]
username = "root"
password = "syf20020816"
bind = "127.0.0.1:10086"
mode = "Memory"
log = "Info"
local = true
```

### surrealdb

#### open surrealdb

![image-20231024165545355](https://github.com/Surrealism-All/Surrealism/blob/0.3.0/README/imgs/image-20231024165545355.png)

#### define ns,db,table first

![image-20231024165607517](https://github.com/Surrealism-All/Surrealism/blob/0.3.0/README/imgs/image-20231024165607517.png)

### main.rs

```rust
use surrealism::db::{SurrealID, Table};
use surrealism::builder::{BaseWrapperImpl, SQLBuilderFactory, TableImpl};
use surrealism::builder::create::{CreateWrapper, CreateWrapperImpl};
use serde::{Serialize, Deserialize};
use surrealism::builder::select::SelectWrapperImpl;
use surrealism::surreal::{parse_response, SurrealismRes,DefaultInitService,UseNSDB,InitService,SurrealismCommit};

#[derive(Debug, Clone, Serialize, Deserialize)]
struct User {
    username: String,
    pwd: String,
    male: bool,
    age: u8,
}

/// create a new user table
/// table_name:user
/// table_id:surrealism
pub fn crate_user_table() -> CreateWrapper {
    // create a user data
    let user = User {
        username: "Tobie".to_string(),
        pwd: "Tobie001".to_string(),
        male: true,
        age: 23,
    };
    // create table with content
    let user_table = SQLBuilderFactory::create()
        .table("user")
        .id("surrealism".into())
        .content(&user)
        .deref_mut();
    user_table
}

//strict!
#[tokio::main]
async fn main() -> SurrealismRes<()> {
    // init service
    let mut service = DefaultInitService::new().init();
    // you have already define test namespace and test database!
    // use ns:test and db:test
    let _ = service.use_commit("test", "test").await?;
    // get info from surrealdb
    // let info = SQLBuilderFactory::info().db().build();
    // let info_res = service.commit_sql(&info).await?;
    // dbg!(info_res);
    // create a table (you should define user table first!)
    let create_stmt = crate_user_table().build();
    let create_res = service.commit_sql(&create_stmt).await?;
    // dbg!(create_res);
    // select user::surrealism table
    let select = SQLBuilderFactory::select().table("user").id("surrealism".into()).column("*", None).build();
    let select_res = service.commit_sql(&select).await?;
    //parse response to any type you want
    let res: User = parse_response(select_res);
    // [tests\src\main.rs:55] res = User {
    //     username: "Tobie",
    //     pwd: "Tobie001",
    //     male: true,
    //     age: 23,
    // }
    dbg!(&res);
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

2023-10-24T08:54:49.323Z INFO  [surrealism::core::surreal::config::init::default] Welcome to use Surrealism!
2023-10-24T08:54:49.323Z INFO  [surrealism::core::surreal::config::init::default] Init Service : `Config Service` Successfully!
2023-10-24T08:54:49.329Z INFO  [surrealism::core::surreal::config::init::default] Please focus following print to check!
Version {
    router: Ok(
        Router {
            conn: PhantomData<surrealdb::api::engine::remote::ws::Client>,
            sender: Sender,
            last_id: 1,
            features: {},
        },
    ),
}
2023-10-24T08:54:49.329Z INFO  [surrealism::core::surreal::config::init::default] Init Service : `Connection Service` Successfully!
[tests\src\main.rs:63] &res = User {
    username: "Tobie",
    pwd: "Tobie001",
    male: true,
    age: 23,
}

```

# Surrealism ALL Supports

## Features

```toml
default = ["builder"]
row = []
builder = []
surreal = ["builder"]
full = ["row", "builder", "surreal"]
```

### Configuration配置文件

<ul>
    <li><input type="checkbox" checked disabled />  <strong>Surrealism.json支持</strong> </li>
    <li><input type="checkbox" checked disabled />  <strong>Surrealism.toml支持</strong> </li>
    <li><input type="checkbox" />  <strong>自定义构建支持(SurrealismConfig)</strong> </li>
</ul>


### Init 初始化服务

<ul>
    <li><input type="checkbox" checked disabled />  <strong>DefaultInitService 默认初始化服务的支持</strong> </li>
    <li><input type="checkbox" />  <strong>自定义初始化服务的支持</strong> </li>
</ul>


### ID 表ID

<ul>
    <li><li><input type="checkbox" checked disabled />  <strong>SurrealID::Default的支持</strong> </li></li>
    <li><input type="checkbox" checked disabled/>  <strong>SurrealID::Int的支持</strong> </li>
    <li><input type="checkbox" checked disabled/>  <strong>SurrealID::Float的支持</strong> </li>
    <li><input type="checkbox" checked disabled/>  <strong>SurrealID::String的支持</strong> </li>
    <li><input type="checkbox" checked disabled/>  <strong>SurrealID::Array的支持</strong> </li>
    <li><input type="checkbox" checked disabled/>  <strong>SurrealID::UUID的支持</strong> </li>
    <li><input type="checkbox" checked disabled/>  <strong>SurrealID::ULID的支持</strong> </li>
    <li><input type="checkbox" checked disabled/>  <strong>SurrealID::RAND的支持</strong> </li>
    <li><input type="checkbox" checked disabled/>  <strong>SurrealID::Range的支持</strong> </li>
</ul>



### Value 数据类型

<ul>
    <li><input type="checkbox" checked disabled />  <strong>SurrealValue::None的支持</strong> </li>
    <li><input type="checkbox" checked disabled />  <strong>SurrealValue::Null的支持</strong> </li>
    <li><input type="checkbox" checked disabled />  <strong>SurrealValue::Int的支持</strong> </li>
    <li><input type="checkbox" checked disabled />  <strong>SurrealValue::Float的支持</strong> </li>
    <li><input type="checkbox" checked disabled />  <strong>SurrealValue::Decimal的支持</strong> </li>
    <li><input type="checkbox" checked disabled />  <strong>SurrealValue::String的支持</strong> </li>
    <li><input type="checkbox" checked disabled />  <strong>SurrealValue::Object的支持</strong> </li>
    <li><input type="checkbox" checked disabled />  <strong>SurrealValue::Datetime的支持(DatetimeAdapter)</strong> </li>
    <li><input type="checkbox" checked disabled />  <strong>SurrealValue::Duration的支持(DurationAdapter)</strong> </li>
    <li><input type="checkbox" checked disabled />  <strong>SurrealValue::Array的支持</strong> </li>
    <li><input type="checkbox" />  <strong>SurrealValue::Set的支持</strong> </li>
    <li><input type="checkbox" />  <strong>SurrealValue::Option的支持</strong> </li>
    <li><input type="checkbox" checked disabled />  <strong>SurrealValue::Geo的支持</strong> </li>
    <li><input type="checkbox" />  <strong>SurrealValue::Record的支持</strong> </li>
    <li><input type="checkbox" />  <strong>SurrealValue::Future的支持</strong> </li>
    <li><input type="checkbox" checked disabled />  <strong>数学常数构建</strong> </li>
    <li><input type="checkbox" checked disabled />  <strong>数学常数支持</strong> </li>
</ul>


## Builder

<ul>
    <li><input type="checkbox" checked disabled />  <strong>USE STMT</strong> </li>
    <li><input type="checkbox" checked disabled />  <strong>CREATE STMT</strong> </li>
    <li><input type="checkbox" checked disabled />  <strong>SELECT STMT</strong> </li>
    <li><input type="checkbox" checked disabled />  <strong>LIVE SELECT STMT</strong> </li>
    <li><input type="checkbox" checked disabled />  <strong>RELATE STMT</strong> </li>
    <li><input type="checkbox" checked disabled />  <strong>UPDATE STMT</strong> </li>
    <li><input type="checkbox" checked disabled />  <strong>INSERT STMT</strong> </li>
    <li><input type="checkbox" checked disabled />  <strong>DELETE STMT</strong> </li>
    <li><input type="checkbox" checked disabled />  <strong>INFO STMT</strong> </li>
    <li><input type="checkbox" checked disabled />  <strong>TRANSACTION STMT</strong> </li>
    <li><input type="checkbox" checked disabled />  <strong>DEFINE STMT</strong> </li>
    <li><input type="checkbox" checked disabled />  <strong>REMOVE STMT</strong> </li>
    <li><input type="checkbox" checked disabled />  <strong>SLEEP STMT</strong> </li>
    <li><input type="checkbox" disabled />  <strong>LET STMT</strong> </li>
    <li><input type="checkbox" disabled />  <strong>BEGIN STMT</strong> </li>
    <li><input type="checkbox" disabled />  <strong>CANCEL STMT</strong> </li>
    <li><input type="checkbox" disabled />  <strong>COMMIT STMT</strong> </li>
    <li><input type="checkbox" disabled />  <strong>IF ELSE STMT</strong> </li>
    <li><input type="checkbox" disabled />  <strong>FOR STMT</strong> </li>
    <li><input type="checkbox" disabled />  <strong>BREAK STMT</strong> </li>
    <li><input type="checkbox" disabled />  <strong>CONTINUE STMT</strong> </li>
    <li><input type="checkbox" disabled />  <strong>KILL STMT</strong> </li>
    <li><input type="checkbox" disabled />  <strong>THROW STMT</strong> </li>
</ul>


### Use

<ul>
    <li><input type="checkbox" checked disabled />  <strong>USE NS STMT</strong> </li>
    <li><input type="checkbox" checked disabled />  <strong>USE DB STMT</strong> </li>
</ul>


### Create

<ul>
    <li><input type="checkbox" checked disabled />  <strong>CREATE CONTENT STMT</strong> </li>
    <li><input type="checkbox" checked disabled />  <strong>CREATE SET STMT</strong> </li>
    <li><input type="checkbox" checked disabled />  <strong>RETURN STMT</strong> </li>
    <li><input type="checkbox" checked disabled />  <strong>TIMEOUT STMT</strong> </li>
    <li><input type="checkbox" checked disabled />  <strong>PARALLEL STMT</strong> </li>
</ul>


### Insert

<ul>
    <li><input type="checkbox" checked disabled />  <strong>INSERT INTO STMT</strong> </li>
    <li><input type="checkbox" checked disabled />  <strong>ON DUPLICATE KEY UPDATE STMT</strong> </li>
</ul>

### Select

<ul>
    <li><input type="checkbox" checked disabled />  <strong>FIELD</strong> </li>
    <li><input type="checkbox" checked disabled />  <strong>OMIT</strong> </li>
    <li><input type="checkbox" checked disabled />  <strong>WITH INDEX|NOINDEX</strong> </li>
    <li><input type="checkbox" checked disabled />  <strong>FROM</strong> </li>
    <li><input type="checkbox" checked disabled />  <strong>WHERE</strong> </li>
    <li><input type="checkbox" checked disabled />  <strong>SPLIT</strong> </li>
    <li><input type="checkbox" checked disabled />  <strong>GROUP</strong> </li>
    <li><input type="checkbox" checked disabled />  <strong>ORDER</strong> </li>
    <li><input type="checkbox" checked disabled />  <strong>LIMIT</strong> </li>
    <li><input type="checkbox" checked disabled />  <strong>START</strong> </li>
    <li><input type="checkbox" checked disabled />  <strong>FETCH</strong> </li>
    <li><input type="checkbox" checked disabled />  <strong>TIMEOUT</strong> </li>
    <li><input type="checkbox" checked disabled />  <strong>PARALLEL</strong> </li>
    <li><input type="checkbox" checked disabled />  <strong>EXPLAIN [FULL]</strong> </li>
</ul>

### Live Select

<ul>
    <li><input type="checkbox" checked disabled />  <strong>FIELD</strong> </li>
    <li><input type="checkbox" checked disabled />  <strong>FROM</strong> </li>
    <li><input type="checkbox" checked disabled />  <strong>WHERE</strong> </li>
    <li><input type="checkbox" checked disabled />  <strong>FETCH</strong> </li>
</ul>

### Delete

<ul>
    <li><input type="checkbox" checked disabled />  <strong>DELETE WHERE</strong> </li>
    <li><input type="checkbox" checked disabled />  <strong>RETURN</strong> </li>
    <li><input type="checkbox" checked disabled />  <strong>TIMEOUT</strong> </li>
    <li><input type="checkbox" checked disabled />  <strong>PARALLEL</strong> </li>
    <li><input type="checkbox" disabled />  <strong>DELETE WITH RELETE</strong> </li>
</ul>


### Remove

<ul>
    <li><input type="checkbox" checked disabled />  <strong>NAMESPACE</strong> </li>
    <li><input type="checkbox" checked disabled />  <strong>DATABASE</strong> </li>
    <li><input type="checkbox" checked disabled />  <strong>USER</strong> </li>
    <li><input type="checkbox" checked disabled />  <strong>LOGIN</strong> </li>
    <li><input type="checkbox" checked disabled />  <strong>TOKEN</strong> </li>
    <li><input type="checkbox" checked disabled />  <strong>SCOPE</strong> </li>
    <li><input type="checkbox" checked disabled />  <strong>TABLE</strong> </li>
    <li><input type="checkbox" checked disabled />  <strong>EVENT</strong> </li>
    <li><input type="checkbox" checked disabled />  <strong>FUNCTION</strong> </li>
    <li><input type="checkbox" checked disabled />  <strong>FIELD</strong> </li>
    <li><input type="checkbox" checked disabled />  <strong>INDEX</strong> </li>
    <li><input type="checkbox" checked disabled />  <strong>PARAM</strong> </li>
</ul>

### Update

<ul>
    <li><input type="checkbox" checked disabled />  <strong>CONTENT</strong> </li>
    <li><input type="checkbox" checked disabled />  <strong>MERGE</strong> </li>
    <li><input type="checkbox" checked disabled />  <strong>PATCH</strong> </li>
    <li><input type="checkbox" checked disabled />  <strong>SET</strong> </li>
    <li><input type="checkbox" checked disabled />  <strong>WHERE</strong> </li>
    <li><input type="checkbox" checked disabled />  <strong>RETURN</strong> </li>
    <li><input type="checkbox" checked disabled />  <strong>TIMEOUT</strong> </li>
    <li><input type="checkbox" checked disabled />  <strong>PARALLEL</strong> </li>
</ul>

### Define

<ul>
    <li><input type="checkbox" checked disabled />  <strong>NAMESPACE</strong> </li>
    <li><input type="checkbox" checked disabled />  <strong>DATABASE</strong> </li>
    <li><input type="checkbox" checked disabled />  <strong>USER</strong> </li>
    <li><input type="checkbox" checked disabled />  <strong>LOGIN</strong> </li>
    <li><input type="checkbox" checked disabled />  <strong>TOKEN</strong> </li>
    <li><input type="checkbox" checked disabled />  <strong>SCOPE</strong> </li>
    <li><input type="checkbox" checked disabled />  <strong>TABLE</strong> </li>
    <li><input type="checkbox" checked disabled />  <strong>EVENT</strong> </li>
    <li><input type="checkbox" checked disabled />  <strong>FUNCTION</strong> </li>
    <li><input type="checkbox" checked disabled />  <strong>FIELD</strong> </li>
    <li><input type="checkbox" checked disabled />  <strong>INDEX</strong> </li>
    <li><input type="checkbox" checked disabled />  <strong>PARAM</strong> </li>
    <li><input type="checkbox" disabled />  <strong>ANALYZER</strong> </li>
</ul>


### Info

<ul>
    <li><input type="checkbox" checked disabled />  <strong>KV</strong> </li>
    <li><input type="checkbox" checked disabled />  <strong>NS</strong> </li>
    <li><input type="checkbox" checked disabled />  <strong>DB</strong> </li>
    <li><input type="checkbox" checked disabled />  <strong>SCOPE</strong> </li>
    <li><input type="checkbox" checked disabled />  <strong>TABLE</strong> </li>
</ul>


### Show

<ul>
    <li><input type="checkbox" checked disabled />  <strong>SINCE</strong> </li>
    <li><input type="checkbox" checked disabled />  <strong>LIMIT</strong> </li>
</ul>


### Sleep

<ul>
    <li><input type="checkbox" checked disabled />  <strong>Duration</strong> </li>
</ul>


## Assert

<ul>
    <li><input type="checkbox"  disabled />  <strong>ASSERT</strong> </li>
    <li><input type="checkbox" checked disabled />  <strong>WHERE</strong> </li>
    <li><input type="checkbox" checked disabled />  <strong>Condition</strong> </li>
    <li><input type="checkbox" checked disabled />  <strong>Criteria</strong> </li>
</ul>


## Functions

<ul>
    <li><input type="checkbox" checked disabled />  <strong>Array</strong> </li>
    <li><input type="checkbox" checked disabled />  <strong>Count</strong> </li>
    <li><input type="checkbox" checked disabled />  <strong>Crypto</strong> </li>
    <li><input type="checkbox" disabled />  <strong>Duration</strong> </li>
    <li><input type="checkbox" disabled />  <strong>Geo</strong> </li>
    <li><input type="checkbox" disabled />  <strong>HTTP</strong> </li>
    <li><input type="checkbox" disabled />  <strong>Math</strong> </li>
    <li><input type="checkbox" disabled />  <strong>Meta</strong> </li>
    <li><input type="checkbox" disabled />  <strong>Parse</strong> </li>
    <li><input type="checkbox" disabled />  <strong>Rand</strong> </li>
    <li><input type="checkbox" disabled />  <strong>Search</strong> </li>
    <li><input type="checkbox" disabled />  <strong>Session</strong> </li>
    <li><input type="checkbox" disabled />  <strong>Sleep</strong> </li>
    <li><input type="checkbox" disabled />  <strong>String</strong> </li>
    <li><input type="checkbox" disabled />  <strong>Time</strong> </li>
    <li><input type="checkbox" disabled />  <strong>Type</strong> </li>
    <li><input type="checkbox" disabled />  <strong>Scripting</strong> </li>
    <li><input type="checkbox" disabled />  <strong>Vector</strong> </li>
</ul>


## Row

<ul>
    <li><input type="checkbox" checked disabled />  <strong>RowSql的支持</strong> </li>
    <li><input type="checkbox" checked disabled/>  <strong>row_sql!宏</strong> </li>
</ul>

## Operators

| Operator     | Description                                                  | Finish |
| ------------ | ------------------------------------------------------------ | ------ |
| && or AND    | Checks whether both of two values are truthy                 | ✅      |
| \|\| or OR   | Checks whether either of two values is truthy                | ✅      |
| ??           | Check whether either of two values are truthy and not `NULL` | ⛔      |
| ?:           | Check whether either of two values are truthy                | ⛔      |
| = or IS      | Check whether two values are equal                           | ✅      |
| != or IS NOT | Check whether two values are not equal                       | ✅      |
| ==           | Check whether two values are exactly equal                   | ✅      |
| ?=           | Check whether any value in a set is equal to a value         | ⛔      |
| *=           | Check whether all values in a set are equal to a value       | ⛔      |
| ~            | Compare two values for equality using fuzzy matching         | ⛔      |
| !~           | Compare two values for inequality using fuzzy matching       | ⛔      |
| ?~           | Check whether any value in a set is equal to a value using fuzzy matching | ⛔      |
| *~           | Check whether all values in a set are equal to a value using fuzzy matching | ⛔      |
| <            | Check whether a value is less than another value             | ✅      |
| <=           | Check whether a value is less than or equal to another value | ✅      |
| >            | Check whether a value is greater than another value          | ✅      |
| >=           | Check whether a value is greater than or equal to another value | ✅      |
| +            | Add two values together                                      | ✅      |
| -            | Subtract a value from another value                          | ✅      |
| * or ×       | Multiply two values together                                 | ⛔      |
| / or ÷       | Divide a value by another value                              | ⛔      |
| **           | Raises a base value by another value                         | ⛔      |
| IN           | Checks whether a value is contained within another value     | ⛔      |
| NOT IN       | Checks whether a value is not contained within another value | ⛔      |
| CONTAINS     | Checks whether a value contains another value                | ✅      |
| CONTAINSNOT  | Checks whether a value does not contain another value        | ⛔      |
| CONTAINSALL  | Checks whether a value contains all other values             | ⛔      |
| CONTAINSANY  | Checks whether a value contains any other value              | ⛔      |
| CONTAINSNONE | Checks whether a value contains none of the following values | ⛔      |
| INSIDE       | Checks whether a value is contained within another value     | ⛔      |
| NOTINSIDE    | Checks whether a value is not contained within another value | ⛔      |
| ALLINSIDE    | Checks whether all values are contained within other values  | ⛔      |
| ANYINSIDE    | Checks whether any value is contained within other values    | ⛔      |
| NONEINSIDE   | Checks whether no value is contained within other values     | ⛔      |
| OUTSIDE      | Checks whether a geometry type is outside of another geometry type | ⛔      |
| INTERSECTS   | Checks whether a geometry type intersects another geometry type | ⛔      |
| @@           | Checks whether the terms are found in a full-text indexed field | ⛔      |



## Update Des

- 0.3.0：
  - 重构init Service 和 config Service（Refactoring init service and config service）
  - 优化SurrealValue（Optimize SurrealValue）
  - 优化Field（Optimize Field）
  - 增加With（Add With）
  - 增加ShowWrapper（Add ShowWrapper）
  - 优化SurrealDB 0.1.0版本更新的基础语句语法（Optimize the basic statement syntax for SurrealDB version 0.1.0 update）

- 0.2.2：
  
  - 添加SelectWrapper向LiveSelectWrapper的转变 (Add the transition from SelectWrapper to LiveSelectWrapper)
  - 添加Field::Diff，针对LiveSelect语句的构建 (Add Field:: Diff to build the LiveSelect statement)
  - 添加SurrealValue对Geometries的支持,GeoJSON (Add SurrealValue support for Geometrics, GeoJSON)
  - 添加所有内置方法Function(突然有些困惑，延迟至下个版本) (Add all built-in method functions (suddenly confused, delayed to the next version))
  - 补充ValueTyped类型Geometries,Decimal,Option (Supplementing ValueTyped Types Geometry, Decimal, Option)
  - 添加ValueConstructor的new_infer()用于通过默认值推测值类型 (Add ValueConstructor::new_Infer() is used to infer the value type from the default value)
  
- 0.2.1：

  - 添加SurrealDB内置方法Function (Add SurrealDB built-in method Function)
    - `Function::array`
    - `Function::count`
    - `Function::crypto`
  - 修复 `RELATE` 语句构造错误的问题，感谢`timlagrande <notifications@github.com>` (Fix the issue of incorrect construction of the `RELATE` statement. Thank `timlagrande <notifications@github.com>`)
  - `SELECT`语句`Column`构建添加`AS`关键字功能 (`SELECT` statement `Column` construction adds `AS`keyword )
  - SurrealDB内置加密功能，见`surrealism::functions::{GenerateCompare, CryptoFunc}` (SurrealDB built-in encryption function，See `surrealism::functions::{GenerateCompare, CryptoFunc}`)

- 0.2.0：

  - 重构了各类Wrapper，使用简单统一的构造器+工厂模式（Reconstructed various Wrappers using a simple and unified constructor+factory pattern）

  - 增加row sql进行语句构建（Add row SQL for statement construction）

  - 启动与初始化更新，你可以基于框架提供的trait和struct自己构建初始化服务（Starting and initializing updates, you can build your own initialization services based on the traits and structs provided by the framework）

  - 增加大量构建工具（Add a large number of construction tools）

  - 分离语句构造和语句提交（Separate statement construction and statement submission）

- 0.1.1：更新配置，增加基于Namespace和Database的支持，但是基于当前SurrealDB无法支持，所以并不能使用🥲（Update the configuration and add support based on Namespace and Database, but it cannot be used due to the current SurrealDB support 🥲）
