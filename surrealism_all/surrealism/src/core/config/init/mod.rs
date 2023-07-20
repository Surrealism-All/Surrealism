mod default;

pub trait InitService{
    /// Init all services including:
    /// - init configuration
    /// - init banner
    /// - init log
    /// - init connection
    fn init(&self)->();
}