//! # Surrealism Core Configuration For Start
//! Main: SurrealismConfig
//! ```txt
//! @author:syf20020816@Outlook.com
//! @date:2023/7/20
//! @version:0.2.0
//! @description:
//! ```

use std::path::PathBuf;
use serde_json;
use serde::{Deserialize, Serialize};
use super::logger::SurrealLogger;

/// Surrealism configuration
/// surreal:单机本地连接Single还是分布式连接Multi
/// username:用户名
/// password:密码
/// url:连接地址
/// port:连接端口
/// mode:连接模式（Memory表示内存File表示存到文件中）
/// path:存储到文件中的文件地址，使用Memory设置为""即可
/// logLevel:日志级别
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SurrealismConfig {
    surreal: SurrealType,
    username: String,
    password: String,
    auth: Option<Auth>,
    url: String,
    port: u16,
    mode: Option<SurrealMode>,
    path: Option<PathBuf>,
    log: Option<SurrealLogger>,
    ns: Option<String>,
    db: Option<String>,
}

impl SurrealismConfig {
    pub fn new() -> SurrealismConfig {
        SurrealismConfig::default()
    }
    /// parse str to SurrealismConfig
    pub fn from(s: &str) -> SurrealismConfig {
        let obj: SurrealismConfig = serde_json::from_str(s).unwrap();
        obj
    }
    pub fn from_self(&mut self, data: SurrealismConfig) -> Self {
        Self {
            surreal: data.surreal,
            username: data.username,
            password: data.password,
            auth: data.auth,
            url: data.url,
            port: data.port,
            mode: data.mode,
            path: data.path,
            log: data.log,
            ns: data.ns,
            db: data.db,
        }
    }
    /// get ref SurrealismConfig
    pub fn get_config(&self) -> &SurrealismConfig {
        &self
    }
    /// get logger service
    /// @return:SurrealLogger
    pub fn get_logger(self) -> SurrealLogger {
        match self.log {
            None => {
                SurrealLogger::default()
            }
            Some(logger) => logger
        }
    }
    pub fn get_username(&self) -> &str {
        &self.username
    }
    pub fn get_password(&self) -> &str {
        &self.password
    }
    pub fn get_url(&self) -> &str { &self.url }
    pub fn get_port(&self) -> u16 { self.port }
}

impl Default for SurrealismConfig {
    fn default() -> Self {
        SurrealismConfig {
            surreal: SurrealType::Single,
            username: "".to_string(),
            password: "".to_string(),
            auth: Some(Auth::Default),
            url: "".to_string(),
            port: 9999,
            mode: Some(SurrealMode::Default),
            path: None,
            log: Some(SurrealLogger::default()),
            ns: None,
            db: None,
        }
    }
}

/// connection type
/// - Single : single connection
/// - Multi : multi connection
/// - Default : default connection == single connection
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
enum SurrealType {
    Single,
    Multi,
    Default,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
enum SurrealMode {
    Memory,
    File,
    Default,
}


#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
enum Auth {
    Root,
    NS,
    DB,
    Default,
}

#[cfg(test)]
mod tests {
    use super::*;

    // #[test]
    // fn config_parse() {
    //     let conf_str = r#"SurrealismConfig { surreal: Single, username: "", password: "", auth: Some(Default), url: "", port: 9999, mode: Some(Default), path: None, log: Some(SurrealLog { level: Warn, print: false, path: "E:\\Rust\\try\\Surrealism\\surrealism_all\\tests" }), ns: None, db: None }"#;
    //     assert_eq!(serde_json::to_string(&SurrealismConfig::default()).unwrap(), String::from(conf_str));
    // }
}