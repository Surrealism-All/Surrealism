mod core;
mod error;
mod util;

pub use crate::core::SurrealismConfig;
pub use crate::error::{ConfigDirNotFoundError,ErrorLevel};



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {

    }
}
