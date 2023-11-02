use std::fmt::{Display, Formatter};
use crate::db::constants::{BLANK, WITH_INDEX};
use super::constants::{WITH_NOINDEX};

/// # With INDEX | NOINDEX
/// 查询规划器可以根据查询的结构和需求，用一个或多个索引迭代器替换标准的表迭代器。
/// 然而，可能存在期望或需要对这些潜在优化进行手动控制的情况。
///
/// 例如，索引的基数可以很高，甚至可能等于表中的记录数。 通过几个索引迭代的记录的总和可能最终大于通过迭代表获得的记录数。 在这种情况下，如果有不同的索引可能性，最可能的最佳选择将是使用具有最低基数的索引。
///
/// - WITH NOINDEX 强制查询规划器使用表迭代器。
/// - WITH INDEX @indexes ... 将查询规划器限制为仅使用指定的索引。
#[derive(Debug, Clone)]
pub enum With<'w> {
    NOINDEX,
    INDEX(Vec<&'w str>),
}

impl<'w> Default for With<'w> {
    fn default() -> Self {
        With::NOINDEX
    }
}

impl<'w> With<'w> {
    pub fn new() -> Self {
        With::NOINDEX
    }
    pub fn no_index() -> Self {
        With::NOINDEX
    }
    pub fn index() -> Self {
        With::INDEX(Vec::new())
    }
    pub fn push(&mut self, field_name: &'w str) -> &mut Self {
        match self {
            With::NOINDEX => panic!("NOINDEX cannot use push function!"),
            With::INDEX(ref mut fields) => fields.push(field_name)
        };
        self
    }
    pub fn build(&self) -> String {
        self.to_string()
    }
}

impl<'w> From<&'w str> for With<'w> {
    fn from(value: &'w str) -> Self {
        With::INDEX(vec![value])
    }
}

impl<'w> From<Vec<&'w str>> for With<'w> {
    fn from(value: Vec<&'w str>) -> Self {
        With::INDEX(value)
    }
}

impl<'w> Display for With<'w> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            With::NOINDEX => f.write_str(WITH_NOINDEX),
            With::INDEX(index) => {
                let _ = f.write_str(WITH_INDEX);
                let _ = f.write_str(BLANK);
                f.write_str(&index.join(", "))
            }
        }
    }
}
