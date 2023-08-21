//! # Default implementation for ConfigurationService
//!
//! # impl DefineConfiguration
//!
//! be like:
//! ``` code
//!  fn define_config_dir(&mut self, path: &str) -> Result<(), ConfigNotFoundError> {
//!          let target_dir = PathBuf::from(path);
//!          if target_dir.try_exists().unwrap() {
//!              self.tmp_dirs.push(target_dir);
//!              Ok(())
//!          } else {
//!              Err(ConfigNotFoundError::new(line!(), file!(), ErrorLevel::Error)
//!                  .set_msg(format!("Could not find the path : {}", path).as_str())
//!                  .set_recommend(format!("You must make dir : {}", path).as_str())
//!                  .print_description()
//!                  .deref_mut()
//!              )
//!          }
//!      }
//! ```
//!
//! ```txt
//! @author:syf20020816@Outlook.com
//! @date:2023/7/20
//! @version:0.0.1
//! @description:
//! ```

use figment::{Figment, providers::{Toml, Json}};
use std::env::current_dir;
use std::path::PathBuf;
use figment::providers::Format;
use super::ConfigurationService;
use crate::{SurrealismConfig};
use crate::core::config::SurrealLogger;
use crate::core::constant::{CONFIG_PATH_COMMON, CONFIG_PATH_LEVEL1, CONFIG_PATH_LEVEL2, CONFIG_NAME, CONFIG_FILE_TYPE_TOML, CONFIG_FILE_TYPE_JSON};
use except_plugin::{EasyException, easy_e, EasyExceptionBuilder, SuperBuilderImpl, ExceptionFactory, ExceptionLevel, CommonParamImpl};
use crate::error::{CONFIG_NOT_FOUND_ERROR, ErrorTypeCode};

/// - path : final target configuration path
/// - data : configuration
/// - tmp_dirs : Process directory for obtaining configuration
#[derive(Debug, Clone)]
pub struct DefaultConfigurationService {
    path: PathBuf,
    data: Option<SurrealismConfig>,
    tmp_dirs: Vec<PathBuf>,
}

impl DefaultConfigurationService {
    pub fn get_surrealism_config(&self) -> Option<SurrealismConfig> {
        self.data.clone()
    }
}

impl ConfigurationService for DefaultConfigurationService {
    fn new() -> Self {
        DefaultConfigurationService {
            path: PathBuf::new(),
            data: None,
            tmp_dirs: vec![],
        }
    }

    fn get_config_dir(&mut self) -> () {
        let current_dir = current_dir().unwrap();
        let mut config_dirs = vec![];
        config_dirs.push(PathBuf::from(format!("{}/{}", current_dir.to_str().unwrap(), CONFIG_PATH_COMMON)));
        config_dirs.push(PathBuf::from(format!("{}/{}", current_dir.to_str().unwrap(), CONFIG_PATH_LEVEL1)));
        config_dirs.push(PathBuf::from(format!("{}/{}", current_dir.to_str().unwrap(), CONFIG_PATH_LEVEL2)));
        for config_dir in config_dirs {
            // exist paths
            if config_dir.try_exists().unwrap() {
                self.tmp_dirs.push(config_dir)
            }
        }
    }

    fn get_config_file(&mut self) -> Result<(), EasyException> {
        /// 计算配置文件等级（初始等级1000）
        /// 等级最高的配置文件将会被启用
        /// configs = -10
        /// templates = -100
        /// toml = 0
        /// json = -1
        fn count_level(files: &Vec<PathBuf>) -> Vec<u16> {
            let mut scores: Vec<u16> = vec![];
            for file in files {
                let mut score = 1000_u16;
                let file_str = file.to_str().unwrap();
                // judge path
                if file_str.contains(CONFIG_PATH_LEVEL1) {
                    score -= 10
                } else if file_str.contains(CONFIG_PATH_LEVEL2) {
                    score -= 100
                } else {}
                //judge file
                if file_str.contains(CONFIG_FILE_TYPE_JSON) {
                    score -= 1
                }
                scores.push(score);
            }
            scores
        }
        fn get_max_index(scores: Vec<u16>) -> usize {
            scores.iter().enumerate().max_by_key(|(_, &value)| value).map(|(index, _)| index).unwrap()
        }

        let mut target_files = vec![];
        for tmp_dir in &self.tmp_dirs {
            target_files.push(PathBuf::from(format!("{}/{}.{}", tmp_dir.to_str().unwrap(), CONFIG_NAME, CONFIG_FILE_TYPE_JSON)));
            target_files.push(PathBuf::from(format!("{}/{}.{}", tmp_dir.to_str().unwrap(), CONFIG_NAME, CONFIG_FILE_TYPE_TOML)));
        }
        // 确定包含了哪些目标文件
        let mut final_files = vec![];
        for target_file in target_files {
            if target_file.try_exists().unwrap() {
                final_files.push(target_file);
            }
        }
        // Can not find Config File
        if final_files.is_empty() {
            Err(
                easy_e!(ErrorTypeCode::CONFIG_NOT_FOUND_ERROR,CONFIG_NOT_FOUND_ERROR,ExceptionLevel::Error,line!(),PathBuf::from(file!()))
            )
        } else {
            let scores = count_level(&final_files);
            // 获取权重最高的索引值
            let target_index = get_max_index(scores);
            // 得到最终确定的配置文件地址
            self.path.push(final_files.remove(target_index));
            Ok(())
        }
    }

    fn get_config_data(&mut self) -> Result<(), EasyException> {
        let parser = Figment::new();
        let mut data = SurrealismConfig::new();
        //get file type (toml or json)
        let path = self.path.to_str().unwrap();
        if path.ends_with(".toml") {
            data = data.from_self(parser.merge(Toml::file(&self.path).nested()).extract::<SurrealismConfig>().unwrap());
        } else {
            data = data.from_self(parser.merge(Json::file(&self.path).nested()).extract::<SurrealismConfig>().unwrap());
        }
        self.data.replace(data);
        Ok(())
    }

    fn init(&mut self) -> Result<(), EasyException> {
        let _ = self.get_config_dir();
        match self.get_config_file() {
            Ok(_) => {
                let _ = self.get_config_data();
                Ok(())
            }
            Err(e) => Err(e)
        }
    }

    fn get_logger(self) -> Result<SurrealLogger, EasyException> {
        match self.data {
            None => {
                Err(easy_e!(ErrorTypeCode::CONFIG_NOT_FOUND_ERROR,CONFIG_NOT_FOUND_ERROR,ExceptionLevel::Error,line!(),PathBuf::from(file!())))
            }
            Some(config_data) => Ok(config_data.get_logger())
        }
    }
}