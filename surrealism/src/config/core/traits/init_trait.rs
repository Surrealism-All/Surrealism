//! 初始化服务的trait
//! - 初始化服务必须实现InitService
//! - init_banner：初始化Banner
//! - init_log：初始化日志，日志使用simple_log-crate，默认级别为INFO
//! - init_connection：初始化连接，连接使用WS（websocket）进行连接
//! ```txt
//! @author:syf20020816@outlook.com
//! @version:0.0.1
//! @date:20230609
//! ```

use surrealdb::engine::remote::ws::Client;
use surrealdb::Surreal;
use crate::config::SurrealConfig;

/// 初始化服务Trait
/// - 默认实现DefaultInitServiceImpl
pub trait InitService {
    /// 初始化Banner
    /// 若使用者的项目中包含一个banner.txt既可以自定义打印banner
    fn init_banner(&self);
    /// 初始化日志
    /// 使用者在Surrealism.
    fn init_log(&self, config: &SurrealConfig);
    /// 初始化连接
    fn init_connection(&self, config: &SurrealConfig) -> Result<Surreal<Client>, surrealdb::Error>;
}
