//! # DefaultInitService
//! 默认的初始化服务实现
//!
//! 所有初始化服务都需要实现InitService trait
//!
//! ```rust
//! use surrealism::{DefaultInitService,InitService};
//! let mut service = DefaultInitService::new().init();
//! ```
//! ```txt
//! @author:syf20020816@Outlook.com
//! @date:2023/7/20
//! @version:0.0.1
//! @description:
//! ```

use std::error::Error;
use std::process::exit;
use log::{error, info};
use log::LevelFilter::{Warn, Debug, Info, Trace};
use simple_logger::SimpleLogger;
use except_plugin::{EasyException, SuperBuilderImpl, Exception};
use futures::executor::block_on;
use crate::core::constant::{BANNER};
use crate::info::{INIT_CONFIG, INIT_LOGGER};
use crate::surreal::SurrealismConnector;
use crate::{err_panic};
use crate::surreal::config::{DefaultConfigurationService,ConfigurationService,LogLevel};
use super::InitService;


#[derive(Debug)]
pub struct DefaultInitService {
    config_service: DefaultConfigurationService,

}

impl InitService for DefaultInitService {
    fn new() -> Self {
        DefaultInitService {
            config_service: DefaultConfigurationService::new(),
        }
    }

    fn init_banner(&self) -> () {
        //判断no-banner是否开启
        if !self.config_service.get_no_banner().unwrap() {
            println!("{}", BANNER);
        }
        info!("{}", "Welcome to use Surrealism!");
    }

    // fn init_log_service(&mut self) -> () {
    //     let config_service = self.config_service.clone();
    //     match config_service.get_logger() {
    //         Ok(logger) => {
    //             self.log_service = SurrealLogger::from(logger);
    //         }
    //         Err(e) => {
    //             let e_code = e.code() as i32;
    //             err_panic!(e.description(),e_code);
    //         }
    //     }
    //     // self.log_service.from()
    //     info!("{}",INIT_LOGGER);
    // }

    fn init_config_service(&mut self) -> Result<(), EasyException> {
        self.config_service.init()
    }

    fn init(&mut self) -> SurrealismConnector {
        //init configuration
        match self.init_config_service() {
            Ok(_) => {
                let _ = self.init_log().init().unwrap();
                let _ = self.init_banner();
                info!("{}",INIT_CONFIG);
                //init logger
                // let _ = self.init_log_service();
                //init connection
                let config_data = self.config_service.get_surrealism_config().unwrap();
                let surrealism_connector = SurrealismConnector::from_config(&config_data);
                match surrealism_connector {
                    Ok(res) => {
                        let version = res.version();
                        info!("Please focus following print to check!\n{:#?}",version);
                        info!("Init Service : `Connection Service` Successfully!");
                        let data = config_data.get_username_password();
                        match block_on(res.try_connect(data.0, data.1)) {
                            Ok(_) => {}
                            Err(e) => {
                                panic!("{:?}", e)
                            }
                        }
                        res
                    }
                    Err(e) => panic!("{:?}", e)
                }
            },
            Err(e) => {
                err_panic!(e.description(),e.code() as i32);
            }
        }
    }

    fn init_log(&self) -> SimpleLogger {
        let logger = SimpleLogger::new();
        match self.config_service.get_logger().unwrap(){
            LogLevel::Error => logger.with_level(log::LevelFilter::Error),
            LogLevel::Warn => logger.with_level(Warn),
            LogLevel::Debug => logger.with_level(Debug),
            LogLevel::Info => logger.with_level(Info),
            LogLevel::Trace => logger.with_level(Trace),
            LogLevel::Full=>logger.with_level(Trace),
            LogLevel::None=>logger.with_level(log::LevelFilter::Off)
        }
    }
}