//! 对InitService的默认实现
//! ```txt
//! @author:syf20020816@outlook.com
//! @version:0.0.1
//! @date:20230609
//! ```

use crate::config::SurrealConfig;
use crate::utils::{is_file_exist, read_file};
use log::{error, info};
use futures::executor::block_on;
use surrealdb::engine::remote::ws::Client;
use surrealdb::{Surreal};
use crate::config::core::traits::{ConfigService, InitService};
use crate::SurrealDB;
use crate::config::constants::{BANNER};
use crate::config::core::DefaultConfigServiceImpl;
use crate::creator::SurrealCore;


///默认初始化服务的实现
pub struct DefaultInitServiceImpl {}

///默认初始化服务的具体实现
impl InitService for DefaultInitServiceImpl {
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

    fn init_log(&self, config: &SurrealConfig) {
        let logger = config.build_log();
        logger.init().unwrap();
    }

    fn init_connection(&self, config: &SurrealConfig) -> Result<Surreal<Client>, surrealdb::Error> {
        // wait for connection
        let cn = block_on(config.connect());
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

impl DefaultInitServiceImpl {
    pub fn new() -> Self {
        DefaultInitServiceImpl {}
    }
    pub fn init(&self) -> Result<SurrealDB, surrealdb::Error> {
        let mut config_service = DefaultConfigServiceImpl::new();
        config_service.convert_config_data();
        //banner
        &self.init_banner();
        //获取配置信息
        let config = config_service.get_config_data();
        //logger
        &self.init_log(config);
        info!("{}","Configuration Initialization over(配置初始化完成)");
        //connection
        info!("{}","Connection Initialization start(初始化连接检测开始)");
        let cn = &self.init_connection(config).unwrap();
        info!("{}","Connection Initialization over , Pay attention to checking the connection detection information above(初始化连接检测结束,注意查看上方连接检测信息)");
        Ok(SurrealDB::new(SurrealCore::new(cn.to_owned().to_owned()), config.clone()))
    }
}