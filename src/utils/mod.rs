use std::fs;
use std::fs::{File};
use std::io::Read;
use regex::Regex;
use crate::creator::AvailData;
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
    let mut target: File = File::open(file).expect("can not open file");
    let mut content = String::new();
    target.read_to_string(&mut content).expect("can not read file");
    return content;
}

///将一段字符串中被\"包含是字符串处理为用'包含
pub fn handle_str(origin_str: &str) -> String {
    let re = Regex::new(r#"\\(.)|""#).unwrap();

    let result = re.replace_all(&origin_str, |caps: &regex::Captures| {
        if let Some(matched) = caps.get(1) {
            format!("'{}'", matched.as_str())
        } else {
            "'".to_owned()
        }
    });
    // 用于匹配 {key:'value'} 类型的字符串片段。其中，'([^']*)' 匹配以单引号开头和结尾的字符串，([^']*) 表示匹配中间的任意字符
    // $1: 表示替换为匹配中间的字符串后加上冒号
    let re2 = Regex::new(r"'([^']*)':").unwrap();
    let tmp = result.to_string();
    let result_stmt = re2.replace_all(&tmp, "$1:");


    result_stmt.to_string()
}

///将一段字符串中"包含是字符串处理为用'包含
fn handle_str2(origin_str: &str) -> String {
    let re = Regex::new(r#"""#).unwrap();
    re.replace_all(origin_str, "'").to_string()
}

///检查顺序
pub fn check_available_order(availables: &Vec<AvailData>) -> bool {
    let mut order_or_not: Option<usize> = None;
    for available in availables {
        if let Some(order) = order_or_not {
            if available.order() != order + 1 {
                return false;
            }
        } else {
            if available.order() != 0 {
                return false;
            }
        }
        order_or_not = Some(available.order())
    }
    true
}