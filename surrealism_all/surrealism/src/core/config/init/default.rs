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
use crate::core::config::conf::ConfigurationService;
use crate::core::config::{DefaultConfigurationService, SurrealLogger, LogLevel};
use super::InitService;
use log::{error, info};
use log::LevelFilter::{Warn, Debug, Info, Trace};
use crate::{INIT_LOGGER, INIT_CONFIG, err_panic};
use simple_logger::SimpleLogger;
use crate::core::{BANNER, SurrealismConnector};
use except_plugin::{EasyException, SuperBuilderImpl, Exception};

#[derive(Debug)]
pub struct DefaultInitService {
    config_service: DefaultConfigurationService,
    log_service: SurrealLogger,
}

impl InitService for DefaultInitService {
    fn new() -> Self {
        DefaultInitService {
            config_service: DefaultConfigurationService::new(),
            log_service: SurrealLogger::new(LogLevel::Info),
        }
    }

    fn init_banner(&self) -> () {
        println!("{}", BANNER);
        info!("{}", "Welcome to use Surrealism!");
    }

    fn init_log_service(&mut self) -> () {
        let config_service = self.config_service.clone();
        match config_service.get_logger() {
            Ok(logger) => {
                self.log_service = SurrealLogger::from(logger);
            }
            Err(e) => {
                let e_code = e.code() as i32;
                err_panic!(e.description(),e_code);
            }
        }
        // self.log_service.from()
        info!("{}",INIT_LOGGER);
    }

    fn init_config_service(&mut self) -> Result<(), EasyException> {
        self.config_service.init()
    }

    fn init(&mut self) -> SurrealismConnector {
        let _ = self.init_log().init().unwrap();
        self.init_banner();
        //init configuration
        match self.init_config_service() {
            Ok(_) => info!("{}",INIT_CONFIG),
            Err(e) => {
                err_panic!(e.description(),e.code() as i32);
            }
        }
        //init logger
        let _ = self.init_log_service();
        //init connection
        let config_data = self.config_service.get_surrealism_config().unwrap();
        let surrealism_connector = SurrealismConnector::from_config(&config_data);
        match surrealism_connector {
            Ok(res) => {
                let version = res.version();
                info!("Please focus following print to check!\n{:#?}",version);
                info!("Init Service : `Connection Service` Successfully!");
                res
            }
            Err(e) => panic!("{:?}", e)
        }
    }

    fn init_log(&self) -> SimpleLogger {
        let logger = SimpleLogger::new();
        match self.log_service.get_level() {
            LogLevel::Error => logger.with_level(log::LevelFilter::Error),
            LogLevel::Warn => logger.with_level(Warn),
            LogLevel::Debug => logger.with_level(Debug),
            LogLevel::Info => logger.with_level(Info),
            LogLevel::Trace => logger.with_level(Trace),
        }
    }
}