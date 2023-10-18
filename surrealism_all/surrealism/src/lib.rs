//! # Surrealism V0.3.0
//! - core : 核心代码模块
//! - error : 错误模块
//! - util : 工具模块
//! - info : 信息模块
//! ```txt
//! @author:syf20020816@Outlook.com
//! @date:2023/10/18
//! @version:0.3.0
//! @description:
//! ```
//!

mod core;
pub mod error;
mod util;
mod info;

// pub use crate::core::{
//     builder, db::*,
//     RowSql, DefaultInitService, InitService, UseNSDB, SurrealismCommit, SurrealismConnector,
// };

#[cfg(feature = "row")]
pub use crate::core::row;
#[cfg(feature = "surreal")]
pub use crate::core::surreal;
#[cfg(feature = "builder")]
pub use crate::core::builder;

/// # Default Result
/// 作用于tokio的默认返回，在不开启`features=["surreal"]`时它是一种更好的、更灵活的选择
/// The default return for tokio which is a better and more flexible option when `features=["surreal"]` is not enabled
pub type DefaultRes<T> = Result<T, Box<dyn std::error::Error>>;
