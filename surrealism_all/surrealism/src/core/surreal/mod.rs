//! # Surreal Feature Model
//! if you want to use surrealism with surrealDB
//! you need to add feature: `features=["surreal"]`
//! ```txt
//! @author:syf20020816@Outlook.com
//! @date:2023/10/18
//! @version:0.0.1
//! @description:
//! ```
use serde::de::DeserializeOwned;
use surrealdb::{Response};

#[cfg(feature = "surreal")]
mod config;
#[cfg(feature = "surreal")]
mod connector;

#[cfg(feature = "surreal")]
pub use config::{SurrealismConfig, DefaultInitService, InitService};
#[cfg(feature = "surreal")]
pub use connector::{UseNSDB, SurrealismCommit, SurrealismConnector};

/// #Surreal返回值
/// ``` rust
/// use surrealism::surreal::SurrealismRes;
/// #[tokio::main]
/// async fn main()->SurrealismRes<()>{
///     //....
///     Ok(())
/// }
/// ```
#[cfg(feature = "surreal")]
pub type SurrealismRes<T> = surrealdb::Result<T>;


///解析 `surrealdb::Response`到`DeserializeOwned+Debug`
///
/// 当我们使用语句提交后我们得到的是`SurrealDB`库提供`Response`结构体，parse_response方法能够帮助我们将Response转化为Rust可用类型
///
/// When we submit using statements, we obtain the 'SurrealDB' library providing the 'Response' structure, parse_ The response method can help us convert the response into a Rust available type
/// > 结果需要调用者自己设置转化类型
/// ```rust
/// use surrealism::surreal::parse_response;
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
#[cfg(feature = "surreal")]
pub fn parse_response<T: DeserializeOwned + std::fmt::Debug>(mut response: Response) -> T
{
    let res: Option<T> = response.take(0).unwrap();
    res.unwrap()
}
