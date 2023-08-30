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
use crate::core::SurrealismConfig;

#[async_trait]
pub trait UseNSDB {
    async fn use_commit(&self, ns: &str, db: &str) -> Result<(), Error>;
}

#[async_trait]
pub trait SurrealismCommit {
    async fn commit_sql(&self, sql: &str) -> Result<surrealdb::Response, surrealdb::Error>;
}

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
    pub async fn new(url: &str, port: u16) -> Result<Surreal<Client>, surrealdb::Error> {
        let uri = format!("{}:{}", url, port);
        let db = Surreal::new::<Ws>(uri.as_str()).await?;
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
        let cn = block_on(SurrealismConnector::new(config.get_url(), config.get_port()));
        match cn {
            Ok(client) => {
                Ok(SurrealismConnector {
                    client
                })
            }
            Err(e) => Err(e)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_connect() {}
}