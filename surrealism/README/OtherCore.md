# Other Core

在本篇中将讲解Surrealism中一些帮助构建语句的助手

In this article, we will explain some assistants in Surrealism that help build statements

- SurrealRes
- Wrapper
- TableId
- IdFunction
- IdRange
- TimeUnit
- Criteria
- parse_response

## SurrealRes

Surreal返回值，用于代替SurrealDB库提供的返回值

Surreal return value，Used to replace the return value provided by the SurrealDB crate

## Wrapper

所有包装器都需要实现这个顶级包装器trait

All wrappers need to implement this top-level wrapper trait

## TableId,IdRange,IdFunction

TableID枚举,当使用CreateWrapper中的id()方法,该方法的入参需要使用TableID enum进行指定

- Num: 数字类型
- Str: 字符串类型
- Object: 对象类型
- Array: 数组类型
- Range: 范围类型(使用IdRange进行指定)
- Fun: 对应Surreal的内置生成ID的方法,包含:rand(),uuid(),ulid()三种

### import 

```rust
use surrealism::{TableId,IdRange,IdFunction};
```

### example

```rust
TableId::<String>::Str("t10088".to_string())
```


## TimeUnit

在SurrealDB数据库中，Timeout子句可以用于设置查询的超时时间。它接受一个时间间隔作为参数，并支持以下单位：

- ms：毫秒
- s：秒
- m：分钟
- h：小时
- d：天

### import

```rust
use surrealism::{TimeUnit};
```

```rust
pub enum TimeUnit {
    MILLISECOND,
    SECOND,
    MINUTE,
    HOUR,
    DAY,
}
```

## Criteria

Where子句的条件构造

### function

| fun name | params:type                            | return        | des                                         |
| -------- | -------------------------------------- | ------------- | ------------------------------------------- |
| new      |                                        | Criteria      | 构建一个Criteria<br />build a Criteria      |
| define   | define_str: &str                       | ()            | 自定义写入条件<br />Custom Write Conditions |
| eq       | 1. core: &str<br />2. comparator: &str | &mut Criteria | 相等条件<br />Equality condition            |
| gt       | 1. core: &str<br />2. comparator: &str | &mut Criteria | 大于条件<br />Greater than condition        |
| lt       | 1. core: &str<br />2. comparator: &str | &mut Criteria | 小于条件<br />Less than condition           |
| neq      | 1. core: &str<br />2. comparator: &str | &mut Criteria | 不相等条件<br />Inequality condition        |
| lte      | 1. core: &str<br />2. comparator: &str | &mut Criteria | 小于等于<br />less than or equal            |
| gte      | 1. core: &str<br />2. comparator: &str | &mut Criteria | 大于等于<br />greater than or equal         |
| and      | 1. left: &str<br />2. right: &str      | &mut Criteria | AND                                         |
| or       | 1. left: &str<br />2. right: &str      | &mut Criteria | OR                                          |

### import

```rust
use surrealism::{Criteria};
```

### example

```rust
use surrealism::{Criteria};

let mut condition = Criteria::new();
condition.eq("name", "'Rose'");
```

## parse_response

解析 `surrealdb::Response`到`DeserializeOwned+Debug`

当我们使用语句提交后我们得到的是`SurrealDB`库提供`Response`结构体，parse_response方法能够帮助我们将Response转化为Rust可用类型

When we submit using statements, we obtain the 'SurrealDB' library providing the 'Response' structure, parse_ The response method can help us convert the response into a Rust available type

> ❗note：结果需要调用者自己设置转化类型
>
> ❗note：The result requires the caller to set their own conversion type

### import

```rust
use surrealism::{parse_response};
```

### example

```rust
let mut param_res = db.return_param("$endpointBase").await?;
/// 将返回的参数解析为Rust可用类型
/// Resolve the returned parameters to Rust available types
let response_parse: String = parse_response(param_res);
dbg!(&response_parse);
```

