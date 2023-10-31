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

#### Surrealism.json(JSON)
```json
{
	"username" : "root",
	"password" : "syf20020816",
	"bind" : "127.0.0.1:10086",
	"mode" : "Memory",
	"log" : "Info",
    "local": true,
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


- [x]  <strong>Surrealism.json支持</strong> 
- [x]  <strong>Surrealism.toml支持</strong> 
- [ ]   <strong>自定义构建支持(SurrealismConfig)</strong> 



### Init 初始化服务


- [x]  <strong>DefaultInitService 默认初始化服务的支持</strong> 
- [ ]   <strong>自定义初始化服务的支持</strong> 



### ID 表ID

- [x] <strong>SurrealID::Default的支持</strong> 
- [x] <strong>SurrealID::Int的支持</strong> 
- [x] <strong>SurrealID::Float的支持</strong> 
- [x] <strong>SurrealID::String的支持</strong> 
- [x]  <strong>SurrealID::Array的支持</strong> 
- [x]  <strong>SurrealID::UUID的支持</strong> 
- [x]  <strong>SurrealID::ULID的支持</strong> 
- [x]  <strong>SurrealID::RAND的支持</strong> 
- [x]  <strong>SurrealID::Range的支持</strong> 




### Value 数据类型


- [x]  <strong>SurrealValue::None的支持</strong> 
- [x]  <strong>SurrealValue::Null的支持</strong> 
- [x]  <strong>SurrealValue::Int的支持</strong> 
- [x]  <strong>SurrealValue::Float的支持</strong> 
- [x]  <strong>SurrealValue::Decimal的支持</strong> 
- [x]  <strong>SurrealValue::String的支持</strong> 
- [x]  <strong>SurrealValue::Object的支持</strong> 
- [x]  <strong>SurrealValue::Datetime的支持(DatetimeAdapter)</strong> 
- [x]  <strong>SurrealValue::Duration的支持(DurationAdapter)</strong> 
- [x]  <strong>SurrealValue::Array的支持</strong> 
- [ ]   <strong>SurrealValue::Set的支持</strong> 
- [ ]   <strong>SurrealValue::Option的支持</strong> 
- [x]  <strong>SurrealValue::Geo的支持</strong> 
- [ ]   <strong>SurrealValue::Record的支持</strong> 
- [ ]   <strong>SurrealValue::Future的支持</strong> 
- [x]  <strong>数学常数构建</strong> 
- [x]  <strong>数学常数支持</strong> 



## Builder


- [x]  <strong>USE STMT</strong> 
- [x]  <strong>CREATE STMT</strong> 
- [x]  <strong>SELECT STMT</strong> 
- [x]  <strong>LIVE SELECT STMT</strong> 
- [x]  <strong>RELATE STMT</strong> 
- [x]  <strong>UPDATE STMT</strong> 
- [x]  <strong>INSERT STMT</strong> 
- [x]  <strong>DELETE STMT</strong> 
- [x]  <strong>INFO STMT</strong> 
- [x]  <strong>TRANSACTION STMT</strong> 
- [x]  <strong>DEFINE STMT</strong> 
- [x]  <strong>REMOVE STMT</strong> 
- [x]  <strong>SLEEP STMT</strong> 
- [ ] <strong>LET STMT</strong> 
- [ ] <strong>BEGIN STMT</strong> 
- [ ] <strong>CANCEL STMT</strong> 
- [ ] <strong>COMMIT STMT</strong> 
- [ ] <strong>IF ELSE STMT</strong> 
- [ ] <strong>FOR STMT</strong> 
- [ ] <strong>BREAK STMT</strong> 
- [ ] <strong>CONTINUE STMT</strong> 
- [ ] <strong>KILL STMT</strong> 
- [ ] <strong>THROW STMT</strong> 



### Use


- [x]  <strong>USE NS STMT</strong> 
- [x]  <strong>USE DB STMT</strong> 



### Create


- [x]  <strong>CREATE CONTENT STMT</strong> 
- [x]  <strong>CREATE SET STMT</strong> 
- [x]  <strong>RETURN STMT</strong> 
- [x]  <strong>TIMEOUT STMT</strong> 
- [x]  <strong>PARALLEL STMT</strong> 



### Insert


- [x]  <strong>INSERT INTO STMT</strong> 
- [x]  <strong>ON DUPLICATE KEY UPDATE STMT</strong> 


### Select


- [x]  <strong>FIELD</strong> 
- [x]  <strong>OMIT</strong> 
- [x]  <strong>WITH INDEX|NOINDEX</strong> 
- [x]  <strong>FROM</strong> 
- [x]  <strong>WHERE</strong> 
- [x]  <strong>SPLIT</strong> 
- [x]  <strong>GROUP</strong> 
- [x]  <strong>ORDER</strong> 
- [x]  <strong>LIMIT</strong> 
- [x]  <strong>START</strong> 
- [x]  <strong>FETCH</strong> 
- [x]  <strong>TIMEOUT</strong> 
- [x]  <strong>PARALLEL</strong> 
- [x]  <strong>EXPLAIN [FULL]</strong> 


### Live Select


- [x]  <strong>FIELD</strong> 
- [x]  <strong>FROM</strong> 
- [x]  <strong>WHERE</strong> 
- [x]  <strong>FETCH</strong> 


### Delete


- [x]  <strong>DELETE WHERE</strong> 
- [x]  <strong>RETURN</strong> 
- [x]  <strong>TIMEOUT</strong> 
- [x]  <strong>PARALLEL</strong> 
- [ ] <strong>DELETE WITH RELETE</strong> 



### Remove


- [x]  <strong>NAMESPACE</strong> 
- [x]  <strong>DATABASE</strong> 
- [x]  <strong>USER</strong> 
- [x]  <strong>LOGIN</strong> 
- [x]  <strong>TOKEN</strong> 
- [x]  <strong>SCOPE</strong> 
- [x]  <strong>TABLE</strong> 
- [x]  <strong>EVENT</strong> 
- [x]  <strong>FUNCTION</strong> 
- [x]  <strong>FIELD</strong> 
- [x]  <strong>INDEX</strong> 
- [x]  <strong>PARAM</strong> 


### Update


- [x]  <strong>CONTENT</strong> 
- [x]  <strong>MERGE</strong> 
- [x]  <strong>PATCH</strong> 
- [x]  <strong>SET</strong> 
- [x]  <strong>WHERE</strong> 
- [x]  <strong>RETURN</strong> 
- [x]  <strong>TIMEOUT</strong> 
- [x]  <strong>PARALLEL</strong> 


### Define


- [x]  <strong>NAMESPACE</strong> 
- [x]  <strong>DATABASE</strong> 
- [x]  <strong>USER</strong> 
- [x]  <strong>LOGIN</strong> 
- [x]  <strong>TOKEN</strong> 
- [x]  <strong>SCOPE</strong> 
- [x]  <strong>TABLE</strong> 
- [x]  <strong>EVENT</strong> 
- [x]  <strong>FUNCTION</strong> 
- [x]  <strong>FIELD</strong> 
- [x]  <strong>INDEX</strong> 
- [x]  <strong>PARAM</strong> 
- [x] <strong>ANALYZER</strong> 



### Info


- [x]  <strong>KV</strong> 
- [x]  <strong>NS</strong> 
- [x]  <strong>DB</strong> 
- [x]  <strong>SCOPE</strong> 
- [x]  <strong>TABLE</strong> 



### Show


- [x]  <strong>SINCE</strong> 
- [x]  <strong>LIMIT</strong> 



### Sleep


- [x]  <strong>Duration</strong> 



## Assert


- [ ] <strong>ASSERT</strong> 
- [x]  <strong>WHERE</strong> 
- [x]  <strong>Condition</strong> 
- [x]  <strong>Criteria</strong> 



## Functions


- [x]  <strong>Array</strong> 
- [x]  <strong>Count</strong> 
- [x]  <strong>Crypto</strong> 
- [ ] <strong>Duration</strong> 
- [ ] <strong>Geo</strong> 
- [ ] <strong>HTTP</strong> 
- [ ] <strong>Math</strong> 
- [ ] <strong>Meta</strong> 
- [ ] <strong>Parse</strong> 
- [ ] <strong>Rand</strong> 
- [ ] <strong>Search</strong> 
- [ ] <strong>Session</strong> 
- [ ] <strong>Sleep</strong> 
- [ ] <strong>String</strong> 
- [ ] <strong>Time</strong> 
- [ ] <strong>Type</strong> 
- [ ] <strong>Scripting</strong> 
- [ ] <strong>Vector</strong> 



## Row


- [x]  <strong>RowSql的支持</strong> 
- [x]  <strong>row_sql!宏</strong> 


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

- 0.3.1：
  - 重构Define语句且补全DEFINE ANALYSIS
  - 

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



