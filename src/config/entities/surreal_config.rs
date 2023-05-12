use serde::{Serialize, Deserialize};

///配置结构体
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


#[derive(Debug, Serialize, Deserialize, PartialEq,Clone)]
pub enum SurrealType {
    Single,
    Multi,
}

#[derive(Debug, Serialize, Deserialize, PartialEq,Clone)]
pub enum ModeType {
    Memory,
    File,
}

