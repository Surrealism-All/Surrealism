mod default;

pub use default::DefaultInitService;
use simple_logger::SimpleLogger;

pub trait InitService {
    fn new() -> Self;
    /// Init all services including:
    /// - init configuration
    /// - init banner
    /// - init log
    /// - init connection
    fn init(&mut self) -> ();
    fn init_log(&self) -> SimpleLogger;
}