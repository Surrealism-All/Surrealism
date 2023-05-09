use std::ops::Add;
use surrealdb::{ Surreal};
use surrealdb::engine::remote::ws::{Ws, Client};
use surrealdb::opt::auth::{Root};
use crate::config::{SurrealConfig};



///连接Surreal数据库
/// surreal的静态数据库引擎和普通连接有以下不同之处：
/// 1. 静态数据库引擎是一种基于文件的数据库引擎，而普通连接则是基于网络的连接方式。静态数据库引擎将数据存储在文件中，而普通连接则将数据存储在远程服务器上。
/// 2. 静态数据库引擎不需要网络连接，可以在本地运行，而普通连接需要网络连接才能访问数据。
/// 3. 静态数据库引擎可以提供更快的数据访问速度，因为它不需要通过网络传输数据。而普通连接则受制于网络速度和带宽的限制。
/// 4. 静态数据库引擎通常用于单机应用程序或小型团队协作，而普通连接则用于大型分布式应用程序或多人协作。
/// 5. 静态数据库引擎不支持多用户并发访问，而普通连接可以支持多用户并发访问。

///连接SurrealDB数据库
pub async fn connect(config: SurrealConfig) -> Result<Surreal<Client>, surrealdb::Error> {
    //连接URI = URL+PORT
    let uri = config.url.add(":").add(config.port.as_str());
    //进行网络连接
    let db: Surreal<Client> = Surreal::new::<Ws>(uri.as_str()).await?;
    //设定用户名密码
    db.signin(Root {
        username: config.username.as_str(),
        password: config.password.as_str(),
    }).await?;

    Ok(db)

}


