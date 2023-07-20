# InfoWrapper

> InfoWrapper的作用是：帮助我们构建Info语句
>
> InfoWrapper：Help us build Info statement

The`INFO` 命令输出有关SurrealDB系统设置的信息。有许多不同的`INFO` 用于在数据库的不同级别检索配置的命令。

The `INFO` command outputs information about the setup of the SurrealDB system. There are a number of different `INFO` commands for retrieving the configuration at the different levels of the database.

## function

| fun name | params:type | return           | des                                              |
| -------- | ----------- | ---------------- | ------------------------------------------------ |
| kv       |             | &mut InfoWrapper | 获取系统信息<br /> get sys info                  |
| ns       |             | &mut InfoWrapper | 获取命名空间信息<br />get namespace info         |
| db       |             | &mut InfoWrapper | 获取数据库信息<br />get database info            |
| scope    | value: &str | &mut InfoWrapper | 获取范围信息<br />get scope info                 |
| table    | value: &str | &mut InfoWrapper | 获取表信息<br />get table info                   |
| next     |             | &mut InfoWrapper | 切换下一条语句<br />Switch to the next statement |

## Import

如果你想使用`InfoWrapper`，你必须从`surrealism`导入`InfoWrapper`和`Wrapper`

If you wanna use `InfoWrapper` , you must import `InfoWrapper` and `Wrapper` from `surrealism`

```rust
use surrealism::{InfoWrapper, Wrapper};
```

## main.rs

```rust
use surrealism::{DefaultInitServiceImpl, SurrealRes, Wrapper, InfoWrapper};

#[tokio::main]
async fn main() -> SurrealRes<()> {
    ///初始化连接
    ///init connection
    let db = DefaultInitServiceImpl::new().init().unwrap();
    /// 构建InfoWrapper
    /// build InfoWrapper
    let mut info = InfoWrapper::new();
    info.kv();
    /// 提交语句
    /// commit statement
    let res = db.commit(&mut info).await;
    dbg!(res.unwrap());
    Ok(())
}
```

## res


```rust
[src\main.rs:33] res.unwrap() = Response(
    {
        0: Ok(
            [
                Object(
                    Object(
                        {
                            "ns": Object(
                                Object(
                                    {
                                        "test": Strand(
                                            Strand(
                                                "DEFINE NAMESPACE test",
                                            ),
                                        ),
                                    },
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

## examples

### System information

The top-level `KV` command returns information regarding the namespaces which exists within the SurrealDB system.

> You must be authenticated as a top-level root user to execute this command.

```sql
INFO FOR KV;
```

#### surrealism

```rust
let mut info = InfoWrapper::new();
info.kv();
```

### Namespace information

The `NS` or `NAMESPACE` command returns information regarding the logins, tokens, and databases under a specific Namespace.

> You must be authenticated as a top-level root user, or a namespace user to execute this command.

> You must have a `NAMESPACE` selected before running this command.

```sql
INFO FOR NS;
```

#### surrealism

```rust
let mut info = InfoWrapper::new();
info.ns();
```

### Database information

The `DB` or `DATABASE` command returns information regarding the logins, tokens, and scopes, and tables under a specific Database.

> You must be authenticated as a top-level root user, a namespace user, or a database user to execute this command.

> You must have a `NAMESPACE` and a `DATABASE` selected before running this command.

```sql
INFO FOR DB;
```

#### surrealism

```rust
let mut info = InfoWrapper::new();
info.ns();
```

### Scope information

The `SCOPE` command returns information regarding the tokens configured under a specific Scope.

> You must be authenticated as a top-level root user, a namespace user, or a database user to execute this command.

> You must have a `NAMESPACE` and a `DATABASE` selected before running this command.

```sql
INFO FOR SCOPE user;
```

#### surrealism

```rust
    let mut info = InfoWrapper::new();
    info.scope("user");
```

### Table information

The `TABLE` command returns information regarding the events, fields, indexes, and foreign table configurations on a specific Table.

> You must be authenticated as a top-level root user, a namespace user, or a database user to execute this command.

> You must have a `NAMESPACE` and a `DATABASE` selected before running this command.

```sql
INFO FOR TABLE user;
```

#### surrealism

```rust
    let mut info = InfoWrapper::new();
    info.table("user");
```

## How to use Next

```rust
info.next()
```

### main.rs

```rust
use surrealism::{DefaultInitServiceImpl, SurrealRes, Wrapper, InfoWrapper,UseWrapper};

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
    ///构建InfoWrapper
    /// 这里使用next方法切换到了下一条语句,也就是INFO FOR NS;
    /// Here, the next method is used to switch to the next statement, which is INFO FOR NS;
    let mut info = InfoWrapper::new();
    info.table("user")
        .ns()
        .kv()
        .next();
    /// 提交语句
    /// commit statement
    let res = db.commit(&mut info).await;
    dbg!(res.unwrap());
    Ok(())
}
```

