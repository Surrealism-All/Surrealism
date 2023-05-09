use std::fs;
use std::fs::{File};
use std::io::Read;
/*=================================================
 * @params:
 * 1. file_path:文件路径
 * @return: Result<bool, &'static str>
 * @date:2023/4/12
 * @description:判断文件是否存在
*=================================================*/
pub fn is_file_exist(file_path: &str) -> Result<bool, &'static str> {
    return if let Ok(meta_data) = fs::metadata(file_path) {
        if meta_data.is_file() {
            Ok(true)
        } else {
            Ok(false)
        }
    } else {
        Ok(false)
    };
}

/*=================================================
 * @params:
 * @return:
 * @date:2023/4/14
 * @description:读取文件信息
*=================================================*/
pub fn read_file(file: &str) -> String {
    let mut target:File = File::open(file).expect("can not open file");
    let mut content = String::new();
    target.read_to_string(&mut content).expect("can not read file");
    return content;
}