//! 获取配置信息的Trait
//! 默认实现为DefaultConfigService
//! ```txt
//! @author:syf20020816@outlook.com
//! @version:0.0.1
//! @date:20230609
//! ```

pub trait ConfigService {
    fn new() -> Self;
    ///获取配置文件地址，按照路径进行查找
    fn find_config_path(&self) -> Result<String, Box<&'static str>>;
    ///获取配置文件中的内容转化为SurrealConfig
    fn convert_config_data(&mut self);
    ///获取配置文件
    fn find_config_file(&mut self, path: &str) -> Result<String, Box<&'static str>>;
}