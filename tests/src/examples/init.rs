//! # Init Service Example
//! use DefaultInitService
//!
//! ```txt
//! @author:syf20020816@Outlook.com
//! @date:2023/8/3
//! @version:0.0.1
//! @description:
//! ```
//    ▄▄▄▄                                                      ▄▄▄▄         ██
//  ▄█▀▀▀▀█                                                     ▀▀██         ▀▀
//  ██▄       ██    ██   ██▄████   ██▄████   ▄████▄    ▄█████▄    ██       ████     ▄▄█████▄  ████▄██▄
//   ▀████▄   ██    ██   ██▀       ██▀      ██▄▄▄▄██   ▀ ▄▄▄██    ██         ██     ██▄▄▄▄ ▀  ██ ██ ██
//       ▀██  ██    ██   ██        ██       ██▀▀▀▀▀▀  ▄██▀▀▀██    ██         ██      ▀▀▀▀██▄  ██ ██ ██
//  █▄▄▄▄▄█▀  ██▄▄▄███   ██        ██       ▀██▄▄▄▄█  ██▄▄▄███    ██▄▄▄   ▄▄▄██▄▄▄  █▄▄▄▄▄██  ██ ██ ██
//   ▀▀▀▀▀     ▀▀▀▀ ▀▀   ▀▀        ▀▀         ▀▀▀▀▀    ▀▀▀▀ ▀▀     ▀▀▀▀   ▀▀▀▀▀▀▀▀   ▀▀▀▀▀▀   ▀▀ ▀▀ ▀▀
//
// 2023-08-22T17:12:56.969Z INFO  [surrealism::core::config::init::default] Welcome to use Surrealism!
// 2023-08-22T17:12:56.970Z INFO  [surrealism::core::config::init::default] Init Service : `Config Service` Successfully!
// 2023-08-22T17:12:56.971Z INFO  [surrealism::core::config::init::default] Init Service : `Log Service` Successfully!
// 2023-08-22T17:12:57.001Z INFO  [surrealism::core::config::init::default] Please focus following print to check!
// Version {
//     router: Ok(
//         Router {
//             conn: PhantomData<surrealdb::api::engine::remote::ws::Client>,
//             sender: Sender,
//             last_id: 0,
//             features: {
//                 Auth,
//             },
//         },
//     ),
// }
// 2023-08-22T17:12:57.002Z INFO  [surrealism::core::config::init::default] Init Service : `Connection Service` Successfully!

use surrealism::{DefaultInitService, InitService, SurrealismConnector, SurrealismRes};

#[tokio::main]
async fn main() -> SurrealismRes<()> {
    let mut service = DefaultInitService::new().init();
    Ok(())
}