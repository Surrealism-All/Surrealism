mod core;
mod error;
mod util;
mod info;

pub use crate::core::{
    SurrealismConfig, DefaultInitService, InitService, SurrealismConnector, SurrealismCommit, UseNSDB, RowSql,
    Table, SurrealID, ReturnType, SQLBuilder, TimeUnit, Range, Object, Array, SurrealValue, TimeOut, ParamCombine,
    ContentSet,
};
pub use crate::error::ErrorLevel;
pub use crate::info::*;
pub use util::{handle_str, remove_format_half};

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
