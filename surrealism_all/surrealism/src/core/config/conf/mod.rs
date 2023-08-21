mod default;

pub use default::DefaultConfigurationService;
use crate::core::config::SurrealLogger;
use except_plugin::{EasyException};

/// Super Configuration Service
///
/// Other configuration service mut impl it
pub trait ConfigurationService {
    /// new configuration service
    fn new() -> Self;
    /// search configuration dir
    fn get_config_dir(&mut self) -> ();
    /// get configuration file
    /// after get_config_dir()
    fn get_config_file(&mut self) -> Result<(), EasyException>;
    /// get configuration from config file
    ///
    /// Error : ConfigNotFoundError
    fn get_config_data(&mut self) -> Result<(), EasyException>;
    /// init
    /// 1. get_config_dir()
    /// 2. get_config_file()
    /// 3. get_config_data()
    fn init(&mut self) -> Result<(), EasyException>;
    fn get_logger(self) -> Result<SurrealLogger, EasyException>;
}

pub trait DefineConfiguration {
    /// define a path to get the configuration
    fn define_config_dir(&mut self, path: &str) -> Result<(), EasyException>;
}