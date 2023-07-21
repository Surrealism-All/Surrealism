//!
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
use log::{warn, error, info};
use log::LevelFilter::{Warn, Debug, Info, Trace};
use crate::{INIT_LOGGER, INIT_CONFIG, ConfigError, err_panic};
use simple_logger::SimpleLogger;

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

    fn init_config(&mut self) -> Result<(), ConfigError> {
        self.config_service.init()
    }

    fn init(&mut self) -> () {
        //init logger
        let _ = self.init_log().init().unwrap();
        info!("{}",INIT_LOGGER);
        //init configuration
        match self.init_config() {
            Ok(_) => info!("{}",INIT_CONFIG),
            Err(e) => {
                err_panic!(e.description(),(ConfigError::ERROR_TYPE_ID as i32));
            }
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