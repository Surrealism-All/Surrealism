use std::error::Error;
use surrealdb::sql::Thing;
use serde::{Deserialize, Serialize};
use super::SurrealCore;
use crate::config::SurrealConfig;
use crate::Wrapper;
use log::error;

///
#[derive(Debug, Deserialize, Serialize)]
pub struct SurrealRecord {
    #[allow(dead_code)]
    id: Thing,
}

pub struct SurrealDB {
    pub core: SurrealCore,
    pub config: SurrealConfig,
}

impl SurrealDB {
    /// 提交SurralQL语句
    pub async fn commit(&self, wrapper: impl Wrapper) -> Result<surrealdb::Response, surrealdb::Error> {
        let sql = wrapper.commit();
        self.core.cn.query(sql).await
    }
    ///提交USE语句
    pub async fn use_commit(&self, wrapper: impl Wrapper) -> Result<(), Box<&'static str>> {
        match wrapper.get_keyword() {
            "USE" => {
                let attrs = wrapper.get_available().lock().unwrap().clone();

                if attrs.is_empty() {
                    Err(Box::new("命名空间，数据库构建异常"))
                } else if attrs.len() == 2 {
                    let mut use_attrs: Vec<&String> = Vec::new();
                    match attrs.get("NS") {
                        Some(res) => use_attrs.push(res),
                        None => (),
                    };
                    match attrs.get("DB") {
                        Some(res) => use_attrs.push(res),
                        None => (),
                    };
                    self.core.cn.use_ns(use_attrs[0]).use_db(use_attrs[1]).await;
                    Ok(())
                } else {
                    let e = "USE语句参数长度异常";
                    // error!("{}",e);
                    Err(Box::new(e))
                }
            }
            _ => {
                let e = "非USE语句请使用commit方法";
                // error!("{}",e);
                Err(Box::new(e))
            }
        }
    }
}