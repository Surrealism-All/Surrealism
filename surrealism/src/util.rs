use std::any::{Any, TypeId};
use regex::Regex;
use crate::core::db::{SurrealIDType, Range, Array, Object};

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

///将字符串中的所有\"去除
pub fn remove_format_half(origin_str: String) -> String {
    let re = Regex::new(r#"\\(.)|""#).unwrap();
    re.replace_all(&origin_str, "").to_string()
}

///将字符串中的所有\去除
pub fn remove_half(origin_str: String) -> String {
    let re = Regex::new(r#"\\(.)"#).unwrap();
    re.replace_all(&origin_str, "").to_string()
}

pub fn match_id_type<T: Any>(_value: &T) -> SurrealIDType {
    match TypeId::of::<T>() {
        id if id == TypeId::of::<i32>() => SurrealIDType::Int,
        id if id == TypeId::of::<f32>() => SurrealIDType::Float,
        id if id == TypeId::of::<f64>() => SurrealIDType::Decimal,
        id if id == TypeId::of::<&str>() => SurrealIDType::Str,
        id if id == TypeId::of::<String>() => SurrealIDType::Str,
        id if id == TypeId::of::<Array>() => SurrealIDType::Array,
        id if id == TypeId::of::<Object>() => SurrealIDType::Object,
        id if id == TypeId::of::<Range>() => SurrealIDType::Range,
        _ => SurrealIDType::Default
    }
}