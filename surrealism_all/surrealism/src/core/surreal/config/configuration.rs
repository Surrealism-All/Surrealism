//! # Surrealism Core Configuration For Start
//! Main: SurrealismConfig
//! ```txt
//! @author:syf20020816@Outlook.com
//! @date:2023/7/20
//! @version:0.2.0
//! @description:
//! ```

use std::fmt::{Display, Formatter};
use std::path::PathBuf;
use serde_json;
use serde::{Deserialize, Serialize};
use super::logger::LogLevel;

/// Surrealism configuration
/// username:用户名
/// password:密码
/// local:本机连接(本机使用ws,远程使用wss)
/// bind: 连接地址,
/// auth:开启权限认证
/// tick_interval:运行节点代理tick的间隔（包括垃圾收集），默认为10秒
/// strict:严格模式
/// mode:连接模式（Memory表示内存File表示存到文件中，Tikv表示tikv集群地址）
/// path:存储到文件中的文件地址，使用Memory则无需设置
/// log:日志级别
/// query_timeout:设置查询超时时间
/// transaction_timeout: 事务超时时间
/// no_banner: 打印Banner
/// db_connection: 数据库连接行为
/// http_server: 服务器行为
/// capabilities: 能力
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SurrealismConfig {
    username: String,
    password: String,
    local:bool,
    auth: Option<bool>,
    bind: Option<String>,
    // --tick-interval：运行节点代理tick的间隔（包括垃圾收集），默认为10秒
    tick_interval: Option<String>,
    strict: Option<bool>,
    query_timeout: Option<String>,
    transaction_timeout: Option<String>,
    mode: Option<SurrealMode>,
    path: Option<PathBuf>,
    log: Option<LogLevel>,
    no_banner: Option<bool>,
    db_connection: Option<DBConnection>,
    http_server: Option<HttpServer>,
    capabilities: Option<Capabilities>,
}

impl From<&str> for SurrealismConfig {
    fn from(value: &str) -> Self {
        let obj: SurrealismConfig = serde_json::from_str(value).unwrap();
        obj
    }
}



impl SurrealismConfig {
    pub fn new() -> SurrealismConfig {
        SurrealismConfig::default()
    }
    // pub fn from_self(&mut self, data: SurrealismConfig) -> Self {}
    /// get ref SurrealismConfig
    pub fn get_config(&self) -> &SurrealismConfig {
        &self
    }
    /// get logger service
    /// @return:SurrealLogger
    pub fn get_logger(&self) -> LogLevel {
        match self.log {
            Some(ref logger) => LogLevel::from(logger),
            None => LogLevel::default()
        }
    }
    pub fn get_username(&self) -> &str {
        &self.username
    }
    pub fn get_password(&self) -> &str {
        &self.password
    }
    pub fn get_username_password(&self) -> (&str, &str) {
        (self.get_username(), self.get_password())
    }
    pub fn get_no_banner(&self)->bool{
        match self.no_banner {
            None => false,
            Some( banner ) => banner
        }
    }
    pub fn get_bind(&self) -> &Option<String> {
        &self.bind
    }
    pub fn get_local(&self)->bool{
        self.local
    }
}

impl Default for SurrealismConfig {
    fn default() -> Self {
        SurrealismConfig {
            username: String::from("root"),
            password: String::from("root"),
            local:true,
            auth: Some(true),
            bind: Some(String::from("0.0.0.0:8000")),
            tick_interval: None,
            strict: Some(true),
            query_timeout: None,
            transaction_timeout: None,
            mode: Some(SurrealMode::default()),
            path: None,
            log: Some(LogLevel::default()),
            no_banner: None,
            db_connection: None,
            http_server: None,
            capabilities: None,
        }
    }
}

impl Display for SurrealismConfig {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        //序列化
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
enum Capabilities {
    AllowAll,
    AllowScripting,
    AllowGuests,
    AllowFuncs,
    AllowNet,
    DenyAll,
    DenyScripting,
    DenyGuests,
    DenyFuncs,
    DenyNet,
}
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
struct DBConnection {
    kvs_ca: PathBuf,
    kvs_crt: PathBuf,
    kvs_key: PathBuf,
}
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
struct HttpServer {
    web_crt: PathBuf,
    web_key: PathBuf,
    client_ip: ClientIP,
}

/// --client-ip ： 指定用于检测客户端 IP 地址的方法。
/// 该选项可以帮助SurrealDB连接客户端的 IP 地址，进行一些操作或记录日志等。
///  这个选项可以设置不同的值来指定不同的客户端 IP 检测方法（该选项可能会随着SurrealDB的更新增加更多的远程连接支持）：
///    1. none ： 不使用客户端 IP，即不获取客户端的 IP 地址
///    2. socket ： 使用原始套接字获取客户端的 IP 地址
///    3. CF-Connecting-IP ：使用云服务提供商 Cloudflare 的连接 IP 方法获取客户端 IP
///    4. Fly-Client-IP ： 使用 Fly.io 平台的客户端 IP 方法获取客户端 IP
///    5. True-Client-IP ： 使用 Akamai、Cloudflare 等服务商的真实客户端 IP 方法获取客户端 IP
///    6. X-Real-IP ：使用 Nginx 的真实 IP 方法获取客户端 IP
///    7. X-Forwarded-For ：使用来自其他代理的行业标准头部获取客户端 IP
#[derive(Debug, Serialize, Deserialize, PartialEq,Clone)]
enum ClientIP {
    None,
    Socket,
    CFConnectingIP,
    FlyClientIP,
    TrueClientIP,
    XRealIP,
    XForwardedIP,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
enum SurrealMode {
    Memory,
    File,
    Tikv,
}

impl Default for SurrealMode {
    fn default() -> Self {
        SurrealMode::Memory
    }
}


#[cfg(test)]
mod tests {
    use super::*;

}