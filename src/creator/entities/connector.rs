use std::cell::RefCell;
use std::fmt::Debug;
use surrealdb::sql::{Thing};
use serde::{Deserialize, Serialize};
use super::{SurrealCore, Statements, RegionField, NS, DB, Wrapper};
use crate::config::SurrealConfig;


///
#[derive(Debug, Deserialize, Serialize)]
pub struct SurrealRecord {
    #[allow(dead_code)]
    id: Thing,
}

///SurrealDB存储配置和Surreal核心
pub struct SurrealDB {
    pub core: SurrealCore,
    pub config: SurrealConfig,
}

impl SurrealDB {
    /// 提交SurrealQL语句
    pub async fn commit(&self, mut wrapper: impl Wrapper) -> Result<surrealdb::Response, surrealdb::Error> {
        let sql = wrapper.commit();
        self.core.cn.query(sql).await
    }
    ///提交USE语句
    pub async fn use_commit(&self, mut wrapper: impl Wrapper) -> Result<(), surrealdb::Error> {
        match wrapper.get_keyword() {
            Statements::USE => {
                wrapper.commit();
                let sql_region = RefCell::new(wrapper.get_available());
                let region_field = sql_region.borrow_mut().get_region_field();
                match region_field {
                    RegionField::Single(_filed) => {
                        panic!("USE Statement use `RegionField::Multi` if you see this mistake please send email to developer:syf20020816@outlook.com");
                    }
                    RegionField::Multi(field_list) => {
                        let mut ns = "";
                        let mut db = "";
                        for field in field_list {
                            match field.get(NS) {
                                Some(res) => {
                                    ns = res;
                                }
                                None => ()
                            }
                            match field.get(DB) {
                                Some(res) => {
                                    db = res;
                                }
                                None => ()
                            }
                        }
                        self.core.cn.use_ns(ns).use_db(db).await
                    }
                }
            }
            _ => {
                panic!("{}", "非USE语句请使用commit方法")
            }
        }
    }
}

