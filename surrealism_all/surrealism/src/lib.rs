mod core;
pub mod error;
mod util;
mod info;

pub use crate::core::{
    builder, db::*,
    RowSql, DefaultInitService, InitService, UseNSDB, SurrealismCommit, SurrealismConnector,
};

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
