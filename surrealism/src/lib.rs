mod core;
pub mod error;
mod util;
mod info;

pub use crate::core::{
    builder, db::*,
    RowSql, DefaultInitService, InitService, UseNSDB, SurrealismCommit, SurrealismConnector,
};

/// #Surreal返回值
/// ``` rust
/// use surrealism::{SurrealismRes};
/// #[tokio::main]
/// async fn main()->SurrealismRes<()>{
///     //....
///     Ok(())
/// }
/// ```
pub type SurrealismRes<T> = surrealdb::Result<T>;

use serde::de::DeserializeOwned;
use surrealdb::{Response};

///解析 `surrealdb::Response`到`DeserializeOwned+Debug`
///
/// 当我们使用语句提交后我们得到的是`SurrealDB`库提供`Response`结构体，parse_response方法能够帮助我们将Response转化为Rust可用类型
///
/// When we submit using statements, we obtain the 'SurrealDB' library providing the 'Response' structure, parse_ The response method can help us convert the response into a Rust available type
/// > 结果需要调用者自己设置转化类型
/// ```rust
/// use surrealism::{parse_response};
/// #[derive(Debug, Clone, Serialize, Deserialize)]
/// struct User {
///     username: String,
///     pwd:String,
///     male: bool,
///     age: u8,
/// }
/// //...commit response
/// let select_res = service.commit_sql(&select).await?;
/// let res: User = parse_response(select_res);
/// ```
pub fn parse_response<T: DeserializeOwned + std::fmt::Debug>(mut response: Response) -> T
{
    let res: Option<T> = response.take(0).unwrap();
    res.unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {}
}
