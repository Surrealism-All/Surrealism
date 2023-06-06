# Surrealism Wrapper

如果你想要使用Surrealism提供的各种Wrapper，你需要引入`Wrapper trait`

If you want to use the various Wrappers provided by Surrealism, you need to introduce the `Wrapper trait`

```rust
use surrealism::{Wrapper}
```

## Wrapper compare

| wrapper       | surrealdb                                                    |
| ------------- | ------------------------------------------------------------ |
| UseWrapper    | https://surrealdb.com/docs/surrealql/statements/use          |
| CreateWrapper | https://surrealdb.com/docs/surrealql/statements/create       |
| SelectWrapper | https://surrealdb.com/docs/surrealql/statements/select       |
| InsertWrapper | https://surrealdb.com/docs/surrealql/statements/insert       |
| DeleteWrapper | https://surrealdb.com/docs/surrealql/statements/delete       |
| UpdateWrapper | https://surrealdb.com/docs/surrealql/statements/update       |
| IfElseWrapper | https://surrealdb.com/docs/surrealql/statements/ifelse       |
| Transaction   | 1. https://surrealdb.com/docs/surrealql/statements/begin<br />2. https://surrealdb.com/docs/surrealql/statements/cancel<br />3. https://surrealdb.com/docs/surrealql/statements/commit |
| InfoWrapper   | https://surrealdb.com/docs/surrealql/statements/info         |
| RelateWrapper | https://surrealdb.com/docs/surrealql/statements/relate       |
| DefineWrapper | https://surrealdb.com/docs/surrealql/statements/define       |
| RemoveWrapper | https://surrealdb.com/docs/surrealql/statements/remove       |

## How to check the statement

如果我们需要查看通过Surrealism的各种Wrapper构建出来的语句是否正确，可以使用每个Wrapper中的commit方法进行语句输出

If we need to check if the statements constructed through various Wrappers of Surrealism are correct, we can use the commit method in each Wrapper to output the statements

```rust
/// for example , we have a SelectWrapper
dbg!(select_wrapper.commit())
```

> ❗note: 请记住在提交语句之前请把该语句输出去除
>
> ❗note: please remember to remove the output of the statement before submitting it
