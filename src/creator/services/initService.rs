use crate::config::SurrealConfig;
use crate::utils::{is_file_exist, read_file};
use super::default_log;
use crate::config::find_config;
use log::{warn, error, info};
use crate::creator::{BANNER};
use super::{connect};
use futures::executor::block_on;
use surrealdb::engine::remote::ws::Client;
use surrealdb::{Error, Surreal};
use crate::creator::entities::{SurrealDB,SurrealCore};



trait InitService {
    ///初始化Banner
    fn initBanner(&self);
    fn initLog(&self, config: SurrealConfig);
    fn initConnection(&self, config: SurrealConfig) -> Result<Surreal<Client>, surrealdb::Error>;
}


pub struct InitServiceImpl {}

impl InitService for InitServiceImpl {
    fn initBanner(&self) {
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

    fn initLog(&self, config: SurrealConfig) {
        let logger = default_log(config.logLevel.as_str());
        logger.init().unwrap();
    }

    fn initConnection(&self, config: SurrealConfig) -> Result<Surreal<Client>, surrealdb::Error> {
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
        &self.initBanner();
        //获取配置信息
        let config = find_config().unwrap();
        //logger
        &self.initLog(config.clone());
        info!("{}","Configuration Initialization over(配置初始化完成)");
        //connection
        info!("{}","Connection Initialization start(初始化连接检测开始)");

        let cn = &self.initConnection(config.clone()).unwrap();
        info!("{}","Connection Initialization over , Pay attention to checking the connection detection information above(初始化连接检测结束,注意查看上方连接检测信息)");

        Ok(SurrealDB {
            core: SurrealCore::new(cn.to_owned().to_owned()),
            config: config.clone(),
        })
    }
}