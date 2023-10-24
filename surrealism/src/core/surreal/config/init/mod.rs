mod default;

pub use default::DefaultInitService;
use crate::surreal::SurrealismConnector;
use simple_logger::SimpleLogger;
use except_plugin::{EasyException};

pub trait InitService {
    fn new() -> Self;
    fn init_banner(&self) -> ();
    /// Init config service
    fn init_config_service(&mut self) -> Result<(), EasyException>;
    /// Init all services including:
    /// - init log
    /// - init banner
    /// - init config service
    /// - init log service
    /// - init connection
    fn init(&mut self) -> SurrealismConnector;
    /// Init log service
    /// init logger
    fn init_log(&self) -> SimpleLogger;
}