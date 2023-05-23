use serde::{Serialize, Deserialize};

///配置结构体
/// surreal:单机本地连接Single还是分布式连接Multi
/// username:用户名
/// password:密码
/// url:连接地址
/// port:连接端口
/// mode:连接模式（Memory表示内存File表示存到文件中）
/// path:存储到文件中的文件地址，使用Memory设置为""即可
/// logLevel:日志级别
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct SurrealConfig {
    pub surreal: SurrealType,
    pub username: String,
    pub password: String,
    pub url: String,
    pub port: String,
    pub mode: ModeType,
    pub path: String,
    pub logLevel: String,
}


impl Default for SurrealConfig {
    fn default() -> Self {
        SurrealConfig {
            surreal: SurrealType::Single,
            username: String::default(),
            password: String::default(),
            url: String::default(),
            port: String::default(),
            mode: ModeType::Memory,
            path: String::default(),
            logLevel: String::default(),
        }
    }
}


#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub enum SurrealType {
    Single,
    Multi,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub enum ModeType {
    Memory,
    File,
}

