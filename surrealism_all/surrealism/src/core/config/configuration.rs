//! # Surrealism Core Configuration For Start
//! Main: SurrealismConfig
//! ```txt
//! @author:syf20020816@Outlook.com
//! @date:2023/7/20
//! @version:0.2.0
//! @description:
//! ```

use std::path::PathBuf;
use std::env::current_dir;
use serde_json;
use serde::{Deserialize, Serialize};

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
    log: Option<SurrealLog>,
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
    /// get ref SurrealismConfig
    pub fn get_config(&self) -> &SurrealismConfig {
        &self
    }
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
            log: Some(SurrealLog::default()),
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

/// log struct for configuration
/// - level : log level (Error,Warn,Debug,Info,Trace)
/// - print : true/false (open log or not)
/// - path : the path for logging
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
struct SurrealLog {
    level: LogLevel,
    print: bool,
    path: PathBuf,
}

impl Default for SurrealLog {
    fn default() -> Self {
        SurrealLog {
            level: LogLevel::Warn,
            print: false,
            path: current_dir().unwrap(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
enum LogLevel {
    Error,
    Warn,
    Debug,
    Info,
    Trace,
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

    #[test]
    fn config_parse() {
        let conf_str = r#"SurrealismConfig { surreal: Single, username: "", password: "", auth: Some(Default), url: "", port: 9999, mode: Some(Default), path: None, log: Some(SurrealLog { level: Warn, print: false, path: "E:\\Rust\\try\\Surrealism\\surrealism_all\\tests" }), ns: None, db: None }"#;
        assert_eq!(serde_json::to_string(&SurrealismConfig::default()).unwrap(), String::from(conf_str));
    }
}