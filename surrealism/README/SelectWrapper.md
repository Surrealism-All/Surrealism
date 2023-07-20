# SelectWrapper

> SelectWrapper的作用：帮助我们构造Select语句
>
> SelectWrapper：Help us construct the Select statement

SELECT语句可用于选择和查询数据库中的数据。每个SELECT语句都支持从多个目标中进行选择，这些目标可以包括表、记录、边、子查询、参数、数组、对象和其他值。

The SELECT statement can be used for selecting and querying data in a  database. Each SELECT statement supports selecting from multiple  targets, which can include tables, records, edges, subqueries,  paramaters, arrays, objects, and other values.

## function

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

## Import

如果你想使用`SelectWrapper`，你必须从`surrealism`导入`SelectWrapper`和`Wrapper`

If you wanna use `SelectWrapper` , you must import `SelectWrapper` and `Wrapper` from `surrealism`

```rust
use surrealism::{Wrapper, SelectWrapper};
```

## select with whole statement

### main.rs

```rust
use surrealism::{DefaultInitServiceImpl, SurrealRes, UseWrapper, Wrapper, SelectWrapper};

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
    let create_res = db.commit(&mut select_wrapper).await;
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

## common use

### main.rs

```rust
use surrealism::{DefaultInitServiceImpl, SurrealRes, UseWrapper, Wrapper, SelectWrapper, Field, Criteria};


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
    let create_res = db.commit(&mut select_wrapper).await;
    dbg!(create_res.unwrap());
    Ok(())
}
```

### res

```bash
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

## more example

import `use surrealism::{OrderCondition, Ordered , TimeUnit}`

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
