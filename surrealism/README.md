<img src="./README/imgs/logo.svg"> <img src="./README/imgs/license.svg">

# Surrealism

- author：syf20020816@outlook.com
- docName：Surrealism README
- createDate：20230506
- updateDate：20230617
- version：0.1.1
- des-tag：test
- email：syf20020816@outlook.com

## LICEMSE

MIT

## Surrealism Introduction

<img src="https://github.com/syf20020816/Surrealism/blob/main/README/imgs/logo.png" />

Surrealism依托于Surrealdb提供的Rust官方标准库:surrealdb,目的是构建一种更加统一，简单的方式对Surrealdb数据库进行各类操作

Surrealism relies on Surrealdb's official Rust standard library:surrealdb,The purpose is to build a more unified and simple way to perform various operations on Surrealdb database

## QuickStart

### add dependencies

```toml
[dependencies]
surrealism = "0.1.1"
tokio = { version = "1.28.0", features = ["macros", "rt-multi-thread"] }
```

### add configuration
配置：
- surreal:单机本地连接Single还是分布式连接Multi
- auth:连接鉴权方式(Root,NS,DB)
- username:用户名
- password:密码
- ns:命名空间名称
- db:数据库名称  
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
- auth:connection authentication method(Root,NS,DB)
- username:your username
- password:your password
- ns:namespace
- db:database
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

> ❗note：当前SurrealDB仍处于开发阶段，对应连接鉴权方式：NS和DB并为支持，所以使用Surrealism进行配置时请以Root方式进行鉴权连接，不要设置ns和db！
>
> ❗note：Currently, SurrealDB is still in the development stage, and the corresponding connection authentication methods are supported: NS and DB. Therefore, when using Surrealsm for configuration, please use Root mode for authentication connections and do not set ns and db!

#### Surrealism.json(JSON)
```json
{
  "surreal": "Single",
  "auth": "Root",
  "username": "root",
  "password": "syf20020816",
  "ns": "",
  "db": "",
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
auth = "Root"
username = "root"
password = "syf20020816"
ns = ""
db = ""
url = "127.0.0.1"
port = "10086"
mode = "Memory"
path = "E:/Rust/surreal"
logLevel="info"
```

### main.rs

```rust
use surrealism::{DefaultInitServiceImpl, SurrealRes};

#[tokio::main]
async fn main() -> SurrealRes<()> {
    ///初始化连接
    ///init connection
    let db = DefaultInitServiceImpl::new().init().unwrap();
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

2023-06-09T13:27:51.921Z INFO  [surrealism::config::core::init::default_init_service_impl] Configuration Initialization over(配置初始化完成)
2023-06-09T13:27:51.921Z INFO  [surrealism::config::core::init::default_init_service_impl] Connection Initialization start(初始化连接检测开始)
2023-06-09T13:27:51.993Z INFO  [surrealism::config::core::init::default_init_service_impl] Version {
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
2023-06-09T13:27:51.994Z INFO  [surrealism::config::core::init::default_init_service_impl] Connection Initialization over , Pay attention to checking the connection detection information above(初始化连接检测结束,注意查看上方连接检测
信息)
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

## Update Des

- 0.1.1：更新配置，增加基于Namespace和Database的支持，但是基于当前SurrealDB无法支持，所以并不能使用🥲（Update the configuration and add support based on Namespace and Database, but it cannot be used due to the current SurrealDB support 🥲）
