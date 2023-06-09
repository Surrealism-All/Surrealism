use serde::{Serialize, Deserialize};
use log::{LevelFilter::{Info, Warn, Debug, Error, Trace}};
use simple_logger::SimpleLogger;
use surrealdb::{Surreal};
use surrealdb::engine::remote::ws::{Ws, Client};
use surrealdb::opt::auth::{Root};


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

impl SurrealConfig {
    pub fn new() -> Self {
        SurrealConfig {
            surreal: SurrealType::Single,
            username: "root".to_string(),
            password: "surrealism".to_string(),
            url: "127.0.0.1".to_string(),
            port: "8080".to_string(),
            mode: ModeType::Memory,
            path: "".to_string(),
            logLevel: "info".to_string(),
        }
    }
    pub fn set_surreal(&mut self, surreal: SurrealType) {
        self.surreal = surreal
    }
    pub fn get_surreal(&self) -> &SurrealType {
        &self.surreal
    }
    pub fn set_username(&mut self, username: &str) {
        self.username = String::from(username);
    }
    pub fn get_username(&self) -> &str {
        &self.username
    }
    pub fn set_password(&mut self, password: &str) {
        self.password = String::from(password);
    }
    pub fn get_password(&self) -> &str {
        &self.password
    }
    pub fn set_url(&mut self, url: &str) {
        self.url = String::from(url);
    }
    pub fn get_url(&self) -> &str {
        &self.url
    }
    pub fn set_port(&mut self, port: &str) {
        self.port = String::from(port);
    }
    pub fn get_port(&self) -> &str {
        &self.port
    }
    pub fn set_mode(&mut self, mode: ModeType) {
        self.mode = mode;
    }
    pub fn get_mode(&self) -> &ModeType {
        &self.mode
    }
    pub fn set_path(&mut self, path: &str) {
        self.username = String::from(path);
    }
    pub fn get_path(&self) -> &str {
        &self.path
    }
    pub fn set_logLevel(&mut self, logLevel: &str) {
        self.logLevel = String::from(logLevel);
    }
    pub fn get_logLevel(&self) -> &str {
        &self.logLevel
    }
    pub fn build_log(&self) -> SimpleLogger {
        let logger = SimpleLogger::new();
        match self.get_logLevel() {
            "error" => logger.with_level(Error),
            "warn" => logger.with_level(Warn),
            "info" => logger.with_level(Info),
            "debug" => logger.with_level(Debug),
            "trace" => logger.with_level(Trace),
            _ => logger.with_level(Info),
        }
    }
    ///连接Surreal数据库
    /// surreal的静态数据库引擎和普通连接有以下不同之处：
    /// 1. 静态数据库引擎是一种基于文件的数据库引擎，而普通连接则是基于网络的连接方式。静态数据库引擎将数据存储在文件中，而普通连接则将数据存储在远程服务器上。
    /// 2. 静态数据库引擎不需要网络连接，可以在本地运行，而普通连接需要网络连接才能访问数据。
    /// 3. 静态数据库引擎可以提供更快的数据访问速度，因为它不需要通过网络传输数据。而普通连接则受制于网络速度和带宽的限制。
    /// 4. 静态数据库引擎通常用于单机应用程序或小型团队协作，而普通连接则用于大型分布式应用程序或多人协作。
    /// 5. 静态数据库引擎不支持多用户并发访问，而普通连接可以支持多用户并发访问。
    ///
    ///连接SurrealDB数据库
    /// connect SurrealDB database
    pub async fn connect(&self) -> Result<Surreal<Client>, surrealdb::Error> {
        //连接URI = URL+PORT
        let uri = format!("{}:{}", self.get_url(), self.get_port());
        //进行网络连接
        let db: Surreal<Client> = Surreal::new::<Ws>(uri.as_str()).await?;
        //设定用户名密码
        db.signin(Root {
            username: self.get_username(),
            password: self.get_password(),
        }).await?;
        Ok(db)
    }
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


///配置文件类型：toml和json
#[derive(Debug)]
pub enum ConfigFileType {
    TOML,
    JSON,
}

///配置文件路径
#[derive(Debug)]
pub enum ConfigFilePath {
    Common,
    Level1,
    Level2,
    Define,
}





