//! # Surrealism Connector
//! 1. Init connection
//! 2. commit sql
//! 3. commit transaction
//! ```txt
//! @author:syf20020816@Outlook.com
//! @date:2023/8/3
//! @version:0.0.1
//! @description:
//! ```

use surrealdb::engine::remote::ws::{Client, Ws};
use surrealdb::{Error, Response, Surreal};
use surrealdb::opt::auth::Root;
use futures::executor::block_on;
use surrealdb::method::{Version};
use async_trait::async_trait;
use crate::surreal::SurrealismConfig;
use crate::db::functions::{CryptoFunc, GenerateCompare};

#[async_trait]
pub trait UseNSDB {
    async fn use_commit(&self, ns: &str, db: &str) -> Result<(), Error>;
}

#[async_trait]
pub trait SurrealismCommit {
    async fn commit_sql(&self, sql: &str) -> Result<surrealdb::Response, surrealdb::Error>;
}
#[derive(Debug)]
pub struct SurrealismConnector {
    client: Surreal<Client>,
}

#[async_trait]
impl UseNSDB for SurrealismConnector {
    async fn use_commit(&self, ns: &str, db: &str) -> Result<(), Error> {
        self.client.use_ns(ns).use_db(db).await
    }
}

#[async_trait]
impl SurrealismCommit for SurrealismConnector {
    async fn commit_sql(&self, sql: &str) -> Result<Response, Error> {
        self.client.query(sql).await
    }
}

impl SurrealismConnector {
    pub async fn new(url:&str) -> Result<Surreal<Client>, surrealdb::Error> {
        let db = Surreal::new::<Ws>(url).await?;
        Ok(db)
    }
    /// from Surreal<Client> return SurrealismConnector
    pub fn from(client: Surreal<Client>) -> SurrealismConnector {
        SurrealismConnector { client }
    }
    /// try to connect surrealDB server
    ///
    /// use Root{username,password}
    pub async fn try_connect(&self, username: &str, password: &str) -> Result<(), Error> {
        let _ = self.client.signin(Root { username, password }).await;
        Ok(())
    }
    /// get version from surrealDB
    ///
    /// @return :  Version<Client>  (from :`use surrealdb::method::Version;`)
    /// print be like:
    ///
    /// ``` json
    ///  Version {
    ///     router: Ok(
    ///         Router {
    ///             conn: PhantomData<surrealdb::api::engine::remote::ws::Client>,
    ///             sender: Sender,
    ///             last_id: 2,
    ///             features: {
    ///                 Auth,
    ///             },
    ///         },
    ///     ),
    /// }
    ///```
    pub fn version(&self) -> Version<Client> {
        self.client.version()
    }
    /// from SurrealismConfig return Result<SurrealismConnector, surrealdb::Error>
    pub fn from_config(config: &SurrealismConfig) -> Result<SurrealismConnector, surrealdb::Error> {
        let cn = block_on(SurrealismConnector::new(config.get_bind().as_ref().unwrap()));
        match cn {
            Ok(client) => {
                Ok(SurrealismConnector {
                    client
                })
            }
            Err(e) => Err(e)
        }
    }
    // /// # crypto
    // /// These functions can be used when hashing data, encrypting data, and for securely authenticating users into the database.
    // /// ## exmaple
    // /// ```rust
    // /// use surrealism::surreal::{ SurrealismRes,UseNSDB, DefaultInitService, InitService, SurrealValue, parse_response};
    // /// use surrealism::db::functions::{GenerateCompare, CryptoFunc};
    // ///
    // /// #[tokio::main]
    // /// async fn main() -> SurrealismRes<()> {
    // ///     // init service
    // ///     let mut service = DefaultInitService::new().init();
    // ///     // use ns:test and db:test
    // ///     let _ = service.use_commit("test", "test").await?;
    // ///
    // ///     // compare
    // ///     let compare = service
    // ///         .crypto(CryptoFunc::bcrypt().compare("$2b$12$kRCndYbUi.N1XHjWcDfNue95k/EvJgAnmHRiVG6oPp9Cg05YOtyDq", SurrealValue::from("surreal")).up())
    // ///         .await?;
    // ///     let res: bool = parse_response(compare);
    // ///     dbg!(res);
    // ///
    // ///     Ok(())
    // /// }
    // /// ```
    // pub async fn crypto<'a>(&self, crypto: CryptoFunc<'a>) -> Result<Response, Error> {
    //     self.client.query(crypto.return_crypto()).await
    // }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_connect() {}
}