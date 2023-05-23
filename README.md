# Surrealism README

- author：syf20020816@outlook.com
- docName：Surrealism README
- createDate：20230506
- updateDate：20230514
- version：0.0.1
- des-tag：test
- email：syf20020816@outlook.com

## Surrealism Introduction
Surrealism依托于Surrealdb提供的Rust官方标准库:surrealdb,目的是构建一种更加统一，简单的方式对Surrealdb数据库进行各类操作

Surrealism relies on Surrealdb's official Rust standard library:surrealdb,The purpose is to build a more unified and simple way to perform various operations on Surrealdb database

## design

## QuickStart

### add dependencies

### add configuration
配置：
- surreal:单机本地连接Single还是分布式连接Multi
- username:用户名
- password:密码
- url:连接地址
- port:连接端口
- mode:连接模式（Memory表示内存File表示存到文件中）
- path:存储到文件中的文件地址，使用Memory设置为""即可
- logLevel:日志级别

可采用JSON或TOML两种配置文件方式

设置配置文件地址可以是：
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
use surrealdb::engine::remote::ws::Client;
use surrealdb::Surreal;
use surrealism::{SurrealRes, InitServiceImpl, SurrealDB, UseWrapper, Wrapper, TableId, IdFunction, SQLParser, ParseSQL, CreateWrapper};
use serde::{Serialize, Deserialize};


#[derive(Debug, Serialize, Deserialize, ParseSQL)]
struct User {
    pub userId: String,
    pub name: String,
    pub email: String,
}


#[tokio::main]
async fn main() -> SurrealRes<()> {
    //初始化数据库连接
    let db: SurrealDB = InitServiceImpl::new().init().unwrap();
    //选择命名空间和数据库
    let mut stmt = UseWrapper::new();
    stmt.use_ns("test").and().use_db("test").build();
    //提交
    let r = db.use_commit(stmt).await;
    dbg!(r);

    //创建表
    let mut create_table = CreateWrapper::new();
    //使用SET方式
    // create_table.create("user")
    //     .id(TableId::<IdFunction>::Fun(IdFunction::RAND))
    //     .and()
    //     .set("name", "zhangsan")
    //     .set("email", "syf2002@out.com")
    //     .and()
    //     .return_field("name")
    //     .build();

    //使用CONTENT方式
    create_table.create("user")
        .id(TableId::<IdFunction>::Fun(IdFunction::RAND))
        .and()
        .content(User {
            userId: "123".to_string(),
            name: "zhangsan".to_string(),
            email: "syf20020816".to_string(),
        })
        .and()
        .return_after()
        .build();

    //提交
    let res = db.commit(create_table).await?;
    dbg!(res);
    let q = db.core.cn.query("SELECT * FROM user").await?;
    dbg!(q);
    Ok(())
}
```