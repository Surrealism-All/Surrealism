use crate::config::SurrealConfig;
use crate::utils::{is_file_exist, read_file};
use super::default_log;
use crate::config::find_config;
use log::{warn, error, info};
use crate::creator::{BANNER};
use super::{connect};
use futures::executor::block_on;
use surrealdb::engine::remote::ws::Client;
use surrealdb::{Surreal};
use crate::creator::entities::{SurrealDB, SurrealCore};


///初始化服务
trait InitService {
    ///初始化Banner
    /// 若使用者的项目中包含一个banner.txt既可以自定义打印banner
    fn init_banner(&self);
    ///初始化日志
    /// 使用者在Surrealism.
    fn init_log(&self, config: SurrealConfig);
    ///初始化连接
    fn init_connection(&self, config: SurrealConfig) -> Result<Surreal<Client>, surrealdb::Error>;
}

///默认初始化服务的实现
pub struct InitServiceImpl {}

///默认初始化服务的具体实现
impl InitService for InitServiceImpl {
    fn init_banner(&self) {
        //查找banner.txt文件进行打印
        match is_file_exist("./banner.txt") {
            Ok(file_exist) => {
                if file_exist {
                    let banner_data = read_file("./banner.txt");
                    println!("{}", banner_data);
                    println!("{}", BANNER);
                } else {
                    println!("{}", BANNER);
                    println!("{}", "Welcome to use Surrealism!");
                }
            }
            Err(e) => error!("{}", e)
        };
    }

    fn init_log(&self, config: SurrealConfig) {
        let logger = default_log(config.logLevel.as_str());
        logger.init().unwrap();
    }

    fn init_connection(&self, config: SurrealConfig) -> Result<Surreal<Client>, surrealdb::Error> {
        let cn = block_on(connect(config));
        match cn {
            Ok(instance) => {
                info!("{:#?}",instance.version());
                return Ok(instance);
            }
            Err(e) =>
                {
                    error!("{:?}", e);
                    panic!("{:?}", e)
                }
        };
    }
}

impl InitServiceImpl {
    pub fn new() -> Self {
        InitServiceImpl {}
    }
    pub fn init(&self) -> Result<SurrealDB, surrealdb::Error> {
        //banner
        &self.init_banner();
        //获取配置信息
        let config = find_config().unwrap();
        //logger
        &self.init_log(config.clone());
        info!("{}","Configuration Initialization over(配置初始化完成)");
        //connection
        info!("{}","Connection Initialization start(初始化连接检测开始)");

        let cn = &self.init_connection(config.clone()).unwrap();
        info!("{}","Connection Initialization over , Pay attention to checking the connection detection information above(初始化连接检测结束,注意查看上方连接检测信息)");

        Ok(SurrealDB {
            core: SurrealCore::new(cn.to_owned().to_owned()),
            config: config.clone(),
        })
    }
}