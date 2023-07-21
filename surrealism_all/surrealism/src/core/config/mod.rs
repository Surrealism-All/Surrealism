//! Surrealism configuration
//! ```txt
//! @author:syf20020816@Outlook.com
//! @date:2023/7/20
//! @version:0.2.0
//! @description:
//! ```


mod init;
mod conf;
mod configuration;
mod logger;

pub use configuration::SurrealismConfig;
pub use conf::DefaultConfigurationService;
pub use init::{DefaultInitService, InitService};
pub use logger::{LogLevel, SurrealLogger};




