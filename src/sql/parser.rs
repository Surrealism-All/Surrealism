//! 解析
//! ```txt
//! @author:syf20020816@outlook.com
//! @version:0.0.1
//! @date:20230609
//! ```
use serde::de::DeserializeOwned;
use surrealdb::{Response};

///解析 `surrealdb::Response`到`DeserializeOwned+Debug`
pub fn parse_response<T: DeserializeOwned + std::fmt::Debug>(mut response: Response) -> T
{
    let res: Option<T> = response.take(0).unwrap();
    res.unwrap()
}