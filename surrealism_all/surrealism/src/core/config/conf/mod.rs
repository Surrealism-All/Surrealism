mod default;


use crate::{ConfigNotFoundError, ConfigParseError, ConfigError};

pub use default::DefaultConfigurationService;

/// Super Configuration Service
///
/// Other configuration service mut impl it
pub trait ConfigurationService {
    /// new configuration service
    fn new() -> Self;
    /// search configuration dir
    fn get_config_dir(&mut self) -> ();
    /// define a path to get the configuration
    fn define_config_dir(&mut self, path: &str) -> Result<(), ConfigNotFoundError>;
    /// get configuration file
    /// after get_config_dir()
    fn get_config_file(&mut self) -> Result<(), ConfigNotFoundError>;
    /// get configuration from config file
    ///
    /// Error : ConfigNotFoundError
    fn get_config_data(&mut self) -> Result<(), ConfigParseError>;
    /// init
    /// 1. get_config_dir()
    /// 2. get_config_file()
    /// 3. get_config_data()
    fn init(&mut self) -> Result<(), ConfigError>;
}