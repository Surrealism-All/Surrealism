# DefineWrapper

> DefineWrapper的作用：帮助我们构造Define语句
>
> DefineWrapper：Help us construct the Define statement

DEFINE语句可用于指定身份验证访问和行为、全局参数、表配置、表事件、模式定义和索引。

The DEFINE statement can be used to specify authentication access and  behaviour, global parameters, table configurations, table events, schema definitions, and indexes.

## function

| fun name               | params:type                                                  | return          | des                                                          |
| ---------------------- | ------------------------------------------------------------ | --------------- | ------------------------------------------------------------ |
| define_namespace       | namespace: &str                                              | DefineNamespace | 构建DEFINE NAMESPACE<br />build DEFINE NAMESPACE             |
| define_database        | database: &str                                               | DefineDatabase  | 构建DEFINE DATABASE<br />build DEFINE DATABASE               |
| define_login_namespace | 1. username: &str<br />2. password: &str                     | DefineLogin     | 构建DEFINE LOGIN (Namespace)<br />build DEFINE LOGIN (Namespace) |
| define_login_database  | 1. username: &str<br />2. password: &str                     | DefineLogin     | 构建DEFINE LOGIN (Database)<br />build DEFINE LOGIN (Database) |
| define_token_namespace | 1. token_name: &str<br />2. token_type: TokenType<br />3. value: &str | DefineToken     | DEFINE TOKEN ... ON NAMESPACE                                |
| define_token_database  | 1. token_name: &str<br />2. token_type: TokenType<br />3. value: &str | DefineToken     | DEFINE TOKEN ... ON DATABASE                                 |
| define_token_scope     | 1. token_name: &str<br />2. token_type: TokenType<br />3. scope_name: &str<br />4. value: &str | DefineToken     | DEFINE TOKEN ... ON SCOPE                                    |
| define_scope           | 1. scope_name: &str<br />2. session: usize<br />3. unit: TimeUnit<br />4. sign_up: &str<br />5. sign_in: &str | DefineScope     | DEFINE SCOPE @name SESSION @duration SIGNUP @expression SIGNIN @expression |
| define_scope_bind      | 1. scope_name: &str<br />2. session: usize<br />3. unit: TimeUnit<br />4. email: &str<br />5. password: &str | DefineScope     | DEFINE SCOPE @name SESSION @duration SIGNUP @expression SIGNIN @expression |

## Import

如果你想使用`DefineWrapper`，你必须从`surrealism`导入`DefineWrapper`，`Wrapper`

If you wanna use `DefineWrapper` , you must import `DefineWrapper` ，`Wrapper` from `surrealism`

```rust
use surrealism::{Wrapper, DefineWrapper};
```

## Define Namespace


SurrealDB有一个多租户模型，它允许您将数据库的范围限定到一个名称空间。数据库的数量没有限制 可以在名称空间中，也没有对允许的名称空间的数量的限制。只有root用户有权 创建命名空间。
 - 您必须作为root用户进行身份验证，才能使用`DEFINE NAMESPACE`声明。


SurrealDB has a multi-tenancy model which allows you to scope  databases to a namespace. There is no limit to the number of databases 	that can be in a namespace, nor is there a limit to the number of  namespaces allowed. Only users root users are authorized to 	create namespaces. 

- You must be authenticated as a root user to use the `DEFINE NAMESPACE` statement.

### login in with root account

```bash
surreal start --user root --pass surrealism --bind 127.0.0.1:10086
```

### main.rs

```rust
use surrealism::{DefaultInitServiceImpl, SurrealRes, Wrapper, UseWrapper, DefineWrapper};

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
    /// 构建DefineWrapper
    /// build DefineWrapper
    let mut define_wrapper = DefineWrapper::new();
    /// 通过define_namespace()转为DefineNamespace
    /// use define_namespace() to DefineNamespace
    let mut define_ns = define_wrapper.define_namespace("test");
    /// 提交
    /// commit
    let res = db.commit(&mut define_ns).await;
    dbg!(res.unwrap());
    Ok(())
}
```

### res

```bash
[src\main.rs:31] res.unwrap() = Response(
    {
        0: Ok(
            [],
        ),
    },
)
```

## Define Database

该DEFINE DATABASE 语句使您可以实例化命名数据库，从而可以指定 安全和配置选项。
 - 必须以root用户或命名空间用户身份进行身份验证，然后才能使用DEFINE DATABASE 声明。
 - 必须选择命名空间 才能使用DEFINE DATABASE 声明。

​		The `DEFINE DATABASE` statement allows you to instantiate a named database, enabling you to specify 	security and configuration options. 

- You must be authenticated as a root or namespace user before you can use the `DEFINE DATABASE` statement.
- [You must select your namespace](https://surrealdb.com/docs/surrealql/statements/use) before you can use the `DEFINE DATABASE` statement.

### main.rs

```rust
use surrealism::{DefaultInitServiceImpl, SurrealRes, Wrapper, UseWrapper, DefineWrapper};

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
    /// 通过define_database()转为DefineDatabase
    /// use define_database() to DefineDatabase
    /// DEFINE DATABASE test;
    let mut define_wrapper = DefineWrapper::new();
    let mut define_db = define_wrapper.define_database("test");
    /// 提交事务
    /// commit
    let res = db.commit(&mut define_db).await;
    dbg!(res.unwrap());
    Ok(())
}
```

### res

```bash
[src\main.rs:32] res.unwrap() = Response(
    {
        0: Ok(
            [],
        ),
    },
)

```

## Define Login

使用DEFINE LOGIN 语句在SurrealDB上创建用户帐户

- 必须以root或命名空间用户身份进行身份验证，才能使用DEFINE LOGIN 声明。

- 必须以root、命名空间或数据库用户身份进行身份验证，才能使用DEFINE LOGIN 声明。

- 必须选择命名空间和/或数据库 才能使用DEFINE LOGIN 声明。

> 注意：您不能使用DEFINE LOGIN 语句创建根或SCOPE 用户。

Use the `DEFINE LOGIN` statement to create user accounts on SurrealDB.

- You must be authenticated as a root or Namespace user to create a Namespace level account using the `DEFINE LOGIN` statement.
- You must be authenticated as a root, Namespace, or Database user to create a Database level account using the `DEFINE LOGIN` statement.
- [You must select your namespace and/or database](https://surrealdb.com/docs/surrealql/statements/use) before you can use the `DEFINE LOGIN` statement.

> Note: You cannot use the `DEFINE LOGIN` statement to create a root or `SCOPE` user. 

### Define login namespace

```rust
use surrealism::{DefaultInitServiceImpl, SurrealRes, Wrapper, UseWrapper, DefineWrapper};

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
    /// DEFINE LOGIN username ON NAMESPACE PASSWORD '123456';
    let mut define_wrapper = DefineWrapper::new();
    let mut define_login_ns = define_wrapper.define_login_namespace("username", "123456");
    /// commit
    let res = db.commit(&mut define_login_ns).await;
    dbg!(res.unwrap());
    Ok(())
}
```

### Define login database

```rust
use surrealism::{DefaultInitServiceImpl, SurrealRes, Wrapper, UseWrapper, DefineWrapper};

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
    /// DEFINE LOGIN username ON DATABASE PASSWORD '123456';
    let mut define_wrapper = DefineWrapper::new();
    let mut define_login_db = define_wrapper.define_login_database("username", "123456");
    /// commit
    let res = db.commit(&mut define_login_db).await;
    dbg!(res.unwrap());
    Ok(())
}
```

## Define token

### Define token namespace

```rust
use surrealism::{DefaultInitServiceImpl, SurrealRes, Wrapper, UseWrapper, DefineWrapper, TokenType};

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
    let token_value = "sNSYneezcr8kqphfOC6NwwraUHJCVAt0XjsRSNmssBaBRh3WyMa9TRfq8ST7fsU2H2kGiOpU4GbAF1bCiXmM1b3JGgleBzz7rsrz6VvYEM4q3CLkcO8CMBIlhwhzWmy8";
    /// DEFINE TOKEN token_name ON NAMESPACE TYPE HS512 VALUE 'sNSYneezcr8kqphfOC6NwwraUHJCVAt0XjsRSNmssBaBRh3WyMa9TRfq8ST7fsU2H2kGiOpU4GbAF1bCiXmM1b3JGgleBzz7rsrz6VvYEM4q3CLkcO8CMBIlhwhzWmy8';
    let mut define_wrapper = DefineWrapper::new();
    let mut define_token_ns = define_wrapper.define_token_namespace("token_name", TokenType::HS512, token_value);
    /// commit
    let res = db.commit(&mut define_token_ns).await;
    dbg!(res.unwrap());
    Ok(())
}
```

### Define token database

```rust
use surrealism::{DefaultInitServiceImpl, SurrealRes, Wrapper, UseWrapper, DefineWrapper, TokenType};

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
    let token_value = "sNSYneezcr8kqphfOC6NwwraUHJCVAt0XjsRSNmssBaBRh3WyMa9TRfq8ST7fsU2H2kGiOpU4GbAF1bCiXmM1b3JGgleBzz7rsrz6VvYEM4q3CLkcO8CMBIlhwhzWmy8";
    /// DEFINE TOKEN token_name ON DATABASE TYPE HS512 VALUE 'sNSYneezcr8kqphfOC6NwwraUHJCVAt0XjsRSNmssBaBRh3WyMa9TRfq8ST7fsU2H2kGiOpU4GbAF1bCiXmM1b3JGgleBzz7rsrz6VvYEM4q3CLkcO8CMBIlhwhzWmy8';
    let mut define_wrapper = DefineWrapper::new();
    let mut define_token_ns = define_wrapper.define_token_database("token_name", TokenType::HS512, token_value);
    /// commit
    let res = db.commit(&mut define_token_ns).await;
    dbg!(res.unwrap());
    Ok(())
}
```

### Define token scope

```rust
use surrealism::{DefaultInitServiceImpl, SurrealRes, Wrapper, UseWrapper, DefineWrapper, TokenType};

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
    let token_value = "sNSYneezcr8kqphfOC6NwwraUHJCVAt0XjsRSNmssBaBRh3WyMa9TRfq8ST7fsU2H2kGiOpU4GbAF1bCiXmM1b3JGgleBzz7rsrz6VvYEM4q3CLkcO8CMBIlhwhzWmy8";
    /// DEFINE TOKEN token_name ON SCOPE account TYPE HS512 VALUE 'sNSYneezcr8kqphfOC6NwwraUHJCVAt0XjsRSNmssBaBRh3WyMa9TRfq8ST7fsU2H2kGiOpU4GbAF1bCiXmM1b3JGgleBzz7rsrz6VvYEM4q3CLkcO8CMBIlhwhzWmy8';
    let mut define_wrapper = DefineWrapper::new();
    let mut define_token_scope = define_wrapper.define_token_scope("token_name", TokenType::HS512, "account", token_value);
    /// commit
    let res = db.commit(&mut define_token_scope).await;
    dbg!(res.unwrap());
    Ok(())
}
```

## Define Scope

在 SurrealDB 中，`DEFINE SCOPE` 语句用于定义一个作用域（Scope），该作用域可以被后续的查询或数据操作所引用。

具体来说，在 `DEFINE SCOPE` 语句中，可以定义多个属性（Property），包括：

- `@name`：该属性表示作用域的名称，用于标识该作用域，并在后续的查询或数据操作中引用该作用域。例如，`@name` 属性可以定义为 `my_scope`。
- `SESSION @duration`：该属性表示作用域的持续时间（Duration），可以指定一个正整数值，单位为秒。作用域在创建之后将保持活动状态，并持续指定的时间长度，直到超时或显式地关闭为止。例如，`SESSION @duration` 属性可以定义为 `3600`，表示作用域的持续时间为 1 小时。
- `SIGNUP @expression`：该属性表示注册（Sign Up）表达式（Expression），用于定义作用域中的新用户注册规则。根据不同的需求，`@expression` 可以是一个布尔表达式、函数或程序代码等。例如，`SIGNUP @expression` 属性可以定义为 `user.age >= 18 && user.email.ends_with("@example.com")`，表示只有年满 18 岁且电子邮件地址以 "@example.com" 结尾的用户才能注册。
- `SIGNIN @expression`：该属性表示登录（Sign In）表达式（Expression），用于定义作用域中的用户登录规则。根据不同的需求，`@expression` 可以是一个布尔表达式、函数或程序代码等。例如，`SIGNIN @expression` 属性可以定义为 `user.email == "john@example.com" && user.password == "s3cr3t"`，表示只有邮箱地址为 "[john@example.com](mailto:john@example.com)"，密码为 "s3cr3t" 的用户才能登录。

通过使用 `DEFINE SCOPE` 语句，可以定义一个作用域，并指定作用域的名称、持续时间、注册和登录规则等属性。在后续的查询或数据操作中，可以使用作用域的名称来引用该作用域，并使用作用域中定义的注册和登录规则来筛选和操作数据。

### Scope bind

```rust
use surrealism::{DefaultInitServiceImpl, SurrealRes, Wrapper, UseWrapper, DefineWrapper, TokenType, TimeUnit};

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
    /// 采用相对固定的语句构建Scope
    /// Using relatively fixed statements to build a Scope
    let mut define_wrapper = DefineWrapper::new();
    let mut define_scope = define_wrapper.define_scope_bind("account", 24, TimeUnit::HOUR, "surrealism@outlook.com", "surrealism");
    /// commit
    let res = db.commit(&mut define_scope).await;
    dbg!(res.unwrap());
    Ok(())
}
```

### Scope

```rust
use surrealism::{DefaultInitServiceImpl, SurrealRes, Wrapper, UseWrapper, DefineWrapper, TokenType, TimeUnit};

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
    ///Define Scope
    let sign_up = "CREATE user SET email = surrealism@outlook.com, pass = crypto::argon2::generate(surrealism)";
    let sign_in ="SELECT * FROM user WHERE email = surrealism@outlook.com AND crypto::argon2::compare(pass, surrealism)";
    let mut define_wrapper = DefineWrapper::new();
    let mut define_scope = define_wrapper.define_scope("account", 24, TimeUnit::HOUR, sign_up, sign_in);
    /// commit
    let res = db.commit(&mut define_scope).await;
    dbg!(res.unwrap());
    Ok(())
}
```

## Define event

在 SurrealDB 中，`DEFINE EVENT` 语句用于定义一个事件（Event），该事件可以在满足指定条件时触发，并执行指定的操作。具体来说，在 `DEFINE EVENT` 语句中，可以定义多个属性（Property），包括：

- `@name`：该属性表示事件的名称，用于标识该事件，并在后续的查询或数据操作中引用该事件。例如，`@name` 属性可以定义为 `my_event`。
- `ON [ TABLE ] @table`：该属性表示事件所关联的数据表（Table），可以指定一个或多个数据表，当数据表中的数据发生变化时，该事件将被触发。例如，`ON @table` 属性可以定义为 `ON my_table`，表示该事件与名为 `my_table` 的数据表关联。
- `WHEN @expression`：该属性表示事件发生的条件（Condition），可以指定一个布尔表达式、函数或程序代码等。当条件满足时，该事件将被触发。例如，`WHEN @expression` 属性可以定义为 `score >= 90`，表示只有当分数大于等于 90 时，该事件才会被触发。
- `THEN @expression`：该属性表示事件触发后执行的操作（Action），可以指定一个或多个操作，包括修改数据、添加数据、删除数据等。例如，`THEN @expression` 属性可以定义为 `UPDATE my_table SET status = 'pass' WHERE id = 1`，表示当事件被触发时，将更新名为 `my_table` 的数据表中 id 为 1 的数据，将其状态设置为 'pass'。

```rust
use surrealism::{DefaultInitServiceImpl, SurrealRes, Wrapper, UseWrapper, DefineWrapper, Criteria};

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
    /// Define Event
    /// DEFINE EVENT my_event ON TABLE my_table WHEN surrealism >= 90 THEN ( UPDATE my_table SET status = 'pass' WHERE id = 1 );
    let then_stmt = "UPDATE my_table SET status = 'pass' WHERE id = 1";
    let mut cri = Criteria::new();
    cri.gte("surrealism", "90");
    let mut define_wrapper = DefineWrapper::new();
    let mut define_event = define_wrapper.define_event("my_event", "my_table", &cri, then_stmt);
    /// commit
    let res = db.commit(&mut define_event).await;
    dbg!(res.unwrap());
    Ok(())
}
```

## Define Function

### main.rs

```rust
use surrealism::{DefaultInitServiceImpl, SurrealRes, Wrapper, UseWrapper, DefineWrapper, FieldType};

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

    ///DEFINE FUNCTION fn::greet($name: string) {
    /// 	RETURN "Hello, " + $name + "!";
    /// }
    let mut define_wrapper = DefineWrapper::new();
    let mut define_fn = define_wrapper.define_function();
    define_fn
        .fn_name("greet")
        .fn_params("name", &FieldType::String)
        .fn_content(r#"RETURN "Hello, " + $name + "!";"#);
    /// commit
    let res = db.commit(&mut define_fn).await;
    dbg!(res.unwrap());
    let res2 = db.run_fn(&mut define_fn, &vec!["Tobie"]).await;
    dbg!(res2.unwrap());
    Ok(())
}
```

### res

```bash
[src\main.rs:41] res2.unwrap() = Response(
    {
        0: Ok(
            [
                Strand(
                    Strand(
                        "Hello, Tobie!",
                    ),
                ),
            ],
        ),
    },
)

```

## Define field

```rust
use surrealism::{DefaultInitServiceImpl, SurrealRes, Wrapper, UseWrapper, DefineWrapper, FieldType};

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
    /// DEFINE FIELD countrycode ON TABLE user TYPE string ASSERT $value != NONE AND $value = /[A-Z]{3}/ VALUE $value OR 'GBR';
    let mut define_wrapper = DefineWrapper::new();
    let mut define_field = define_wrapper.define_field();
    define_field
        .field("countrycode")
        .table("user")
        .field_type(&FieldType::String)
        .variable("value")
        .field_assert("$value != NONE AND $value = /[A-Z]{3}/")
        .default_value("GBR");
    /// commit
    let res = db.commit(&mut define_field).await;
    dbg!(res.unwrap());
    Ok(())
}
```

## Define index

```

```

