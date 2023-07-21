mod default;


use crate::ConfigDirNotFoundError;

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
    fn define_config_dir(&mut self, path: &str) -> Result<(), ConfigDirNotFoundError>;
}