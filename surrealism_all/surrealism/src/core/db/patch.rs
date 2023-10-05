//! # Patch OP
//! ```txt
//! @author:syf20020816@Outlook.com
//! @date:2023/8/25
//! @version:0.0.1
//! @description:
//! ```



use serde::{Serialize, Deserialize};
use serde_json::json;
use crate::{ParamCombine, SurrealValue};

const ADD: &str = "add";
const REPLACE: &str = "replace";
const REMOVE: &str = "remove";
const TEST: &str = "test";
const MOVE: &str = "move";
const COPY: &str = "copy";

/// # Patch 语句是一种用于修改和更新数据的操作
///
/// 它遵循 JSON Patch 规范。Patch 语句由一个 JSON 数组组成，每个数组元素表示一个要应用的修改操作。
///
/// 一个 Patch 语句通常包含以下三个属性：
///
/// 1. "op"：指定要执行的操作类型。常见的操作类型包括：
/// 2. "path"：指定要操作的目标路径。路径是一个字符串，可以使用斜杠 / 分隔不同的层级。
/// 3. "value"：对于 "add"、"replace" 或 "test" 操作，指定要应用的新值。
///
/// ## example
/// ```json
/// [
///   { "op": "add", "path": "/name", "value": "John Doe" },
///   { "op": "replace", "path": "/age", "value": 30 },
///   { "op": "remove", "path": "/address" }
///   { "op": "copy", "from": "/name", "path": "/person/name_copy" }
///   { "op": "test", "path": "/name", "value": "John Doe" }
///   { "op": "move", "from": "/source/path", "path": "/destination/path" }
/// ]
/// ```
/// 在上述示例中：
/// - 第一个操作是 "add" 操作，将 "name": "John Doe" 添加到目标路径 /name。
/// - 第二个操作是 "replace" 操作，将目标路径 /age 的值替换为 30。
/// - 第三个操作是 "remove" 操作，删除目标路径 /address。
///
/// Patch 语句提供了一种通用的方式来表示数据的增删改操作，并且可以适用于各种数据存储系统
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Patch<'p> {
    op: PatchOp,
    path: &'p str,
    from: Option<&'p str>,
    value: Option<SurrealValue>,
}

impl<'p> Patch<'p> {
    pub fn add<T>(path: &'p str, value: T) -> Self
        where
            T: Serialize
    {
        Patch::new(PatchOp::Add, None, path, Some(SurrealValue::from(serde_json::to_value(value).unwrap())))
    }
    pub fn replace<T>(path: &'p str, value: T) -> Self
        where
            T: Serialize,
    {
        Patch::new(PatchOp::Replace, None, path, Some(SurrealValue::from(serde_json::to_value(value).unwrap())))
    }
    pub fn test<T>(path: &'p str, value: T) -> Self
        where
            T: Serialize,
    {
        Patch::new(PatchOp::Test, None, path, Some(SurrealValue::from(serde_json::to_value(value).unwrap())))
    }
    pub fn move_path(from: &'p str, path: &'p str) -> Self {
        Patch::new(PatchOp::Move, Some(from), path, None)
    }
    pub fn copy(from: &'p str, path: &'p str) -> Self {
        Patch::new(PatchOp::Move, Some(from), path, None)
    }
    pub fn remove(path: &'p str) -> Self {
        Patch::new(PatchOp::Remove, None, path, None)
    }
    fn new(op: PatchOp, from: Option<&'p str>, path: &'p str, value: Option<SurrealValue>) -> Self {
        Patch {
            op,
            path,
            from,
            value,
        }
    }
    pub fn build(&self) -> String {
        // Patch -> serde::Value -> json_str
        match self.op {
            PatchOp::Add | PatchOp::Replace | PatchOp::Test => {
                // no from
                json!({"op":self.op.to_str(),"path":self.path,"value":self.value.as_ref().unwrap().to_str()}).to_string()
            }
            PatchOp::Remove => {
                // no value and  no from
                json!({"op":self.op.to_str(),"path":self.path}).to_string()
            }
            PatchOp::Copy | PatchOp::Move => {
                // no value
                json!({"op":self.op.to_str(),"from":self.from.unwrap(),"path":self.path}).to_string()
            }
        }
    }
}

impl<'p> ParamCombine for Patch<'p> {
    fn combine(&self) -> String {
        self.build()
    }
}

/// # patch op for patch
///     - "add": 在指定路径上新增一个值。
///     - "remove": 从指定路径上删除一个值。
///     - "replace": 替换指定路径上的值。
///     - "copy": 将一个值从一个路径复制到另一个路径。
///     - "move": 将一个值从一个路径移动到另一个路径。
///     - "test": 检查指定路径上的值是否匹配给定的值。
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PatchOp {
    Add,
    Remove,
    Replace,
    Copy,
    Move,
    Test,
}

impl PatchOp {
    pub fn to_str(&self) -> &str {
        match self {
            PatchOp::Add => ADD,
            PatchOp::Remove => REMOVE,
            PatchOp::Replace => REPLACE,
            PatchOp::Copy => COPY,
            PatchOp::Move => MOVE,
            PatchOp::Test => TEST
        }
    }
}