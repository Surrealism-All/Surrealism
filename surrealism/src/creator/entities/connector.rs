use std::cell::RefCell;
use std::fmt::Debug;
use surrealdb::sql::{Thing};
use serde::{Deserialize, Serialize};
use super::{SurrealCore, Statements, RegionField, NS, DB, Wrapper, Transaction, DefineFunction, RETURN};
use crate::config::SurrealConfig;
use crate::handle_str;


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
    pub fn new(core: SurrealCore, config: SurrealConfig) -> Self {
        SurrealDB {
            core,
            config,
        }
    }
    /// 提交SurrealQL语句
    pub async fn commit(&self, wrapper: &mut impl Wrapper) -> Result<surrealdb::Response, surrealdb::Error> {
        let sql = wrapper.commit();
        self.core.cn.query(sql).await
    }
    ///提交USE语句
    pub async fn use_commit(&self, wrapper: &mut impl Wrapper) -> Result<(), surrealdb::Error> {
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
    /// 提交事务
    pub async fn commit_transaction(&self, transaction: &Transaction) -> Result<surrealdb::Response, surrealdb::Error> {
        let sql = transaction.get_stmt();
        self.core.cn.query(sql).await
    }
    /// 运行某个函数，前提是需要先使用Define Function语句进行定义
    pub async fn run_fn(&self, func: &DefineFunction, params: &Vec<&str>) -> Result<surrealdb::Response, surrealdb::Error> {
        let params_list = func.get_params();
        if params.len() != params_list.len() {
            panic!("参数数量错误请检查(Parameter quantity error, please check)");
        }
        let mut params_str = String::new();
        for i in 0..params.len() {
            params_str.push_str(handle_str(serde_json::to_string(params[i]).unwrap().as_str()).as_str());
            if i != params.len() - 1 {
                params_str.push_str(",");
            }
        }
        let return_stmt = format!("{} {}({})", RETURN, func.get_name(), params_str);

        self.core.cn.query(&return_stmt).await
    }
    /// 返回某个Param，前提是SurrealDB中有，你需要使用Define Param语句先进行定义
    /// 具体怎么返回取决于传入的语句
    pub async fn return_param(&self, return_stmt: &str) -> Result<surrealdb::Response, surrealdb::Error> {
        let stmt = format!("RETURN {};", return_stmt);
        self.core.cn.query(&stmt).await
    }
}

