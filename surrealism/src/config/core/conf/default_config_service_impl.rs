//! 对ConfigService的默认实现
//! ```txt
//! @author:syf20020816@outlook.com
//! @version:0.0.1
//! @date:20230609
//! ```


use figment::{Figment, providers::{Toml, Format, Json}};
use crate::config::constants::{CONFIG_PATH_COMMON, CONFIG_PATH_LEVEL1, CONFIG_PATH_LEVEL2, CONFIG_NAME, CONFIG_FILE_TYPE_TOML, CONFIG_FILE_TYPE_JSON};
use std::path::Path;
use crate::utils::is_file_exist;
use crate::config::core::traits::ConfigService;
use crate::config::SurrealConfig;


pub struct DefaultConfigServiceImpl {
    config_path: String,
    config_data: SurrealConfig,
}

impl DefaultConfigServiceImpl {
    pub fn set_config_path(&mut self, config_path: &str) {
        self.config_path = String::from(config_path)
    }
    pub fn get_config_path(&self) -> &str {
        &self.config_path
    }
    pub fn set_config_data(&mut self, config_data: SurrealConfig) {
        self.config_data = config_data
    }
    pub fn get_config_data(&self) -> &SurrealConfig {
        &self.config_data
    }
}


impl ConfigService for DefaultConfigServiceImpl {
    fn new() -> Self {
        DefaultConfigServiceImpl {
            config_path: "".to_string(),
            config_data: SurrealConfig::new(),
        }
    }
    fn find_config_path(&self) -> Result<String, Box<&'static str>> {
        //地址列表
        let mut config_path_vec = Vec::new();
        config_path_vec.push(Path::new(CONFIG_PATH_COMMON));
        config_path_vec.push(Path::new(CONFIG_PATH_LEVEL1));
        config_path_vec.push(Path::new(CONFIG_PATH_LEVEL2));
        let mut target_path = String::new();
        //匹配地址列表
        for x in config_path_vec {
            if x.exists() {
                target_path = x.to_str().unwrap().to_string();
            }
        }

        if target_path.is_empty() {
            Err(Box::new("Couldn't find dir"))
        } else {
            Ok(target_path)
        }
    }
    fn find_config_file(&mut self, path: &str) -> Result<String, Box<&'static str>> {
        let mut files = Vec::new();
        files.push(format!("{}{}.{}", path, CONFIG_NAME, CONFIG_FILE_TYPE_TOML));
        files.push(format!("{}{}.{}", path, CONFIG_NAME, CONFIG_FILE_TYPE_JSON));

        for file in files {
            //存在目录寻找文件
            if is_file_exist(&file.as_str()).unwrap_or_default() {
                self.set_config_path(&file);
                return Ok(file);
            }
        }
        Err(Box::new("Invalid file"))
    }
    fn convert_config_data(&mut self) {
        let config_path = self.find_config_path();
        match config_path {
            Ok(res) => {
                match self.find_config_file(&res) {
                    Ok(res_file) => {
                        let parser = Figment::new();
                        //去除前缀
                        let real_file_path: String = res_file[2..].to_string();
                        //判断后缀
                        if real_file_path.ends_with(".toml") {
                            //toml解析
                            self.set_config_data(parser.merge(Toml::file(real_file_path).nested()).extract::<SurrealConfig>().unwrap());
                        } else {
                            //json解析
                            self.set_config_data(parser.merge(Json::file(real_file_path)).extract::<SurrealConfig>().unwrap_or_else(|_|{panic!("config parse error : please check the Surrealism Config(Surrealism.toml||Surrealism.json)")}));
                        }
                    }
                    Err(e) => { panic!("{}", e) }
                }
            }
            Err(e) => { panic!("{}", e) }
        };
    }
}
