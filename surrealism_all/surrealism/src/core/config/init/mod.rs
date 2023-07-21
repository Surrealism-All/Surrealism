mod default;

pub use default::DefaultInitService;
use crate::{ConfigError};
use simple_logger::SimpleLogger;

pub trait InitService {
    fn new() -> Self;
    /// Init config service
    fn init_config(&mut self) -> Result<(), ConfigError>;
    /// Init all services including:
    /// - init configuration
    /// - init banner
    /// - init log
    /// - init connection
    fn init(&mut self) -> ();
    /// init logger
    fn init_log(&self) -> SimpleLogger;
}