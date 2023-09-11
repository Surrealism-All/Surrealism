<img src="https://img.shields.io/badge/surrealism-0.2.1-orange?style=flat-square&logo=rust&logoColor=%23fff&labelColor=%23DEA584&color=%23DEA584">  <img src="https://img.shields.io/badge/License-MIT-orange?style=flat-square&logoColor=%23fff&labelColor=%2323B898&color=%2323B898">

# Surrealism

- author：syf20020816@outlook.com
- docName：Surrealism README
- createDate：20230506
- updateDate：20230904
- version：0.2.1
- email：syf20020816@outlook.com

## LICEMSE

MIT

## Surrealism Introduction

<img src="https://github.com/syf20020816/Surrealism/blob/main/logo.png" />

Surrealism依托于Surrealdb提供的Rust官方标准库:surrealdb,目的是构建一种更加统一，简单的方式对Surrealdb数据库进行各类操作

Surrealism relies on Surrealdb's official Rust standard library:surrealdb,The purpose is to build a more unified and simple way to perform various operations on Surrealdb database

## QuickStart

### add dependencies

```toml
[dependencies]
surrealism = {version="0.2.1"}
tokio = { version = "1.28.0", features = ["macros", "rt-multi-thread"] }
```

### add configuration
配置：

 -  surreal:单机本地连接Single还是分布式连接Multi
 -  username:用户名
 -  password:密码
 -  auth:连接鉴权方式(Root,NS,DB)
 -  url:连接地址
 -  port:连接端口
 -  mode:连接模式（Memory表示内存File表示存到文件中）
 -  path:存储到文件中的文件地址，使用Memory设置为""即可
 -  log:日志
 -  ns:命名空间名称 (auth = NS)⛔
 -  db:数据库名称 (auth = DB)⛔

可采用JSON或TOML两种配置文件方式

设置配置文件地址可以是：
- ./Surrealism.toml
- ./configs/Surrealism.toml
- ./templates/Surrealism.toml

<hr>

configuration：

- surreal: Single machine local connection or distributed connection Multi
- username: username
- password: Password
- auth: Connection authentication method (Root, NS, DB)
- url: Connection address
- port: Connection port
- mode: Connection mode (Memory represents memory, File represents saving to a file)
- path: The file address stored in the file can be set to '' using 'Memory'
- log: log
- ns: namespace name (auth=NS) ⛔
- db: Database name (auth=DB) ⛔

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
	"surreal" : "Single"
	"auth" : "Root"
	"username" : "root"
	"password" : "syf20020816"
	"url" : "127.0.0.1"
	"port" : 10086
	"mode" : "Memory"
	"path" : "E:/Rust/surreal"
	"log" : {"level" : "Info", "print" : true,"path" : "E:/surrealism/log" }
}
```
#### Surrealism.toml(TOML)
```toml
[default]
surreal = "Single"
auth = "Root"
username = "root"
password = "syf20020816"
url = "127.0.0.1"
port = 10086
mode = "Memory"
path = "E:/Rust/surreal"
log = { level = "Info", print = true, path = "E:/surrealism/log" }
```

### main.rs

```rust
use surrealism::{DefaultInitService, InitService, SurrealID, SurrealismCommit, SurrealismConnector, SurrealismRes, Table, UseNSDB, parse_response};
use surrealism::builder::{BaseWrapperImpl, SQLBuilderFactory, TableImpl};
use surrealism::builder::create::{CreateWrapper, CreateWrapperImpl};
use serde::{Serialize, Deserialize};
use surrealism::builder::select::SelectWrapperImpl;

#[derive(Debug, Clone, Serialize, Deserialize)]
struct User {
    username: String,
    pwd:String,
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
        .id(SurrealID::from("surrealism"))
        .content(&user)
        .deref_mut();
    user_table
}

#[tokio::main]
async fn main() -> SurrealismRes<()> {
    // init service
    let mut service = DefaultInitService::new().init();
    // use ns:test and db:test
    let _ = service.use_commit("test", "test").await?;
    // get info from surrealdb
    // let info = SQLBuilderFactory::info().db().build();
    // let info_res = service.commit_sql(&info).await?;
    // dbg!(info_res);
    // create a table
    // let create_stmt = crate_user_table().build();
    // let create_res = service.commit_sql(&create_stmt).await?;
    // dbg!(create_res);
    // select user::surrealism table
    let select = SQLBuilderFactory::select().table("user").id(SurrealID::from("surrealism")).column("*").build();
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

2023-08-30T03:06:57.875Z INFO  [surrealism::core::config::init::default] Welcome to use Surrealism!
2023-08-30T03:06:57.878Z INFO  [surrealism::core::config::init::default] Init Service : `Config Service` Successfully!
2023-08-30T03:06:57.878Z INFO  [surrealism::core::config::init::default] Init Service : `Log Service` Successfully!
2023-08-30T03:06:57.886Z INFO  [surrealism::core::config::init::default] Please focus following print to check!
Version {
    router: Ok(
        Router {
            conn: PhantomData<surrealdb::api::engine::remote::ws::Client>,
            sender: Sender,
            last_id: 0,
            features: {
                Auth,
            },
        },
    ),
}
2023-08-30T03:06:57.888Z INFO  [surrealism::core::config::init::default] Init Service : `Connection Service` Successfully!
[tests\src\main.rs:60] &res = User {
    username: "Tobie",
    pwd: "Tobie001",
    male: true,
    age: 23,
}
```

## Update Des

- 0.2.2（预计发布时间：9月21日前）：
  
  - 添加SelectWrapper向LiveSelectWrapper的转变
  - 添加Field::Diff，针对LiveSelect语句的构建
  - 添加SurrealValue对Geometries的支持,GeoJSON
  - 添加所有内置方法Function（未实现）
  - 取消ValueType、ValueConstructor和SurrealValue之间的隔阂，合并功能SurrealValue（未实现）
  
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
