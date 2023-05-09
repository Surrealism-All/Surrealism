use figment::{Figment, providers::{Toml, Format, Json}};
use crate::config::bindings::*;
use crate::config::entities::*;
use std::path::Path;
use crate::utils::is_file_exist;


trait ConfigService {
    ///获取配置文件地址，按照路径进行查找
    fn get_config_path() -> Result<String, Box<&'static str>>;
    ///获取配置文件中的内容转化为SurrealConfig
    fn get_config_data(file_path: String) -> SurrealConfig;
    ///获取配置文件
    fn get_config_file(path: String) -> Result<String, Box<&'static str>>;
}

struct ConfigServiceImpl {}

impl ConfigService for ConfigServiceImpl {
    fn get_config_path() -> Result<String, Box<&'static str>> {
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
    fn get_config_file(path: String) -> Result<String, Box<&'static str>> {
        let mut files = Vec::new();
        files.push(path.clone() + CONFIG_NAME + "." + CONFIG_FILE_TYPE_TOML);
        files.push(path.clone() + CONFIG_NAME + "." + CONFIG_FILE_TYPE_JSON);

        for file in files {
            //存在目录寻找文件
            if is_file_exist(&file.as_str()).unwrap_or_default() {
                return Ok(file);
            }
        }
        Err(Box::new("Invalid file"))
    }
    fn get_config_data(file_path: String) -> SurrealConfig {
        let parser = Figment::new();
        //去除前缀
        let real_file_path: String = file_path[2..].to_string();

        //判断后缀
        if real_file_path.ends_with(".toml") {

            //toml解析
            return parser.merge(Toml::file(real_file_path).nested()).extract::<SurrealConfig>().unwrap();
        } else {
            //json解析
            return parser.merge(Json::file(real_file_path)).extract::<SurrealConfig>().unwrap();
        }
    }
}

///获取配置文件
pub fn find_config() -> Result<SurrealConfig, Box<&'static str>> {
    let config_path = ConfigServiceImpl::get_config_path();
    match config_path {
        Ok(res) => {
            match ConfigServiceImpl::get_config_file(res) {
                Ok(res_file) => {
                    let res = ConfigServiceImpl::get_config_data(res_file);
                    return Ok(res);
                }
                Err(e) => { panic!("{}",e) }
            }
        }
        Err(e) => { panic!("{}", e) }
    };
}