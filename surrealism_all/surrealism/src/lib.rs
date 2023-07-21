mod core;
mod error;
mod util;
mod info;

pub use crate::core::{SurrealismConfig, DefaultInitService, InitService};
pub use crate::error::{ConfigDirNotFoundError, ErrorLevel};
pub use crate::info::*;


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {}
}
