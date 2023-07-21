//! Default implementation for ConfigurationService
//! ```txt
//! @author:syf20020816@Outlook.com
//! @date:2023/7/20
//! @version:0.0.1
//! @description:
//! ```

use std::ops::{Deref, DerefMut};
use std::path::PathBuf;
use super::ConfigurationService;
use crate::{ConfigDirNotFoundError, SurrealismConfig, ErrorLevel};
use crate::core::constant::{CONFIG_PATH_COMMON, CONFIG_PATH_LEVEL1, CONFIG_PATH_LEVEL2};

/// - path : final target configuration path
/// - data : configuration
/// - tmp_dirs : Process directory for obtaining configuration
#[derive(Debug)]
pub struct DefaultConfigurationService {
    path: Option<PathBuf>,
    data: Option<SurrealismConfig>,
    tmp_dirs: Vec<PathBuf>,
}

impl ConfigurationService for DefaultConfigurationService {
    fn new() -> Self {
        DefaultConfigurationService {
            path: None,
            data: None,
            tmp_dirs: vec![],
        }
    }

    fn get_config_dir(&mut self) -> () {
        let mut config_dirs = vec![];
        config_dirs.push(PathBuf::from(CONFIG_PATH_COMMON));
        config_dirs.push(PathBuf::from(CONFIG_PATH_LEVEL1));
        config_dirs.push(PathBuf::from(CONFIG_PATH_LEVEL2));
        for config_dir in config_dirs {
            // exist paths
            if config_dir.try_exists().unwrap() {
                self.tmp_dirs.push(config_dir)
            }
        }
    }

    fn define_config_dir(&mut self, path: &str) -> Result<(), ConfigDirNotFoundError> {
        let target_dir = PathBuf::from(path);
        if target_dir.try_exists().unwrap() {
            self.tmp_dirs.push(target_dir);
            Ok(())
        } else {

            Err(ConfigDirNotFoundError::new(line!(),file!() ,ErrorLevel::Error)
                .set_msg(format!("Could not find the path : {}", path).as_str())
                .set_recommend(format!("You must make dir : {}", path).as_str())
                .print_description()
                .deref_mut()
            )
        }
    }
}