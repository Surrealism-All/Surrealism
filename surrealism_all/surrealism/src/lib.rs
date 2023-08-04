mod core;
mod error;
mod util;
mod info;

pub use crate::core::{
    SurrealismConfig, DefaultInitService, InitService, SurrealismConnector, SurrealismCommit, UseNSDB, RowSql,
    Table, IDNumber, SurrealID, ReturnType,
};
pub use crate::error::{ConfigNotFoundError, ErrorLevel, ConfigParseError, ConfigError};
pub use crate::info::*;

/// #Surreal返回值
/// ``` rust
/// use surrealism::{SurrealismRes};
/// #[tokio::main]
/// async fn main()->SurrealismRes<()>{
///     //....
///     Ok(())
/// }
/// ```
pub type SurrealismRes<T> = surrealdb::Result<T>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {}
}
