//! # Init Service Example
//! use DefaultInitService
//! ```txt
//! @author:syf20020816@Outlook.com
//! @date:2023/8/3
//! @version:0.0.1
//! @description:
//! ```

use surrealism::{DefaultInitService, InitService, SurrealismConnector, SurrealismRes};

#[tokio::main]
async fn main() -> SurrealismRes<()> {
    let mut service = DefaultInitService::new().init();
    Ok(())
}