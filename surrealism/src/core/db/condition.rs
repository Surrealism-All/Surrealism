//! # where condition
//! ```txt
//! @author:syf20020816@Outlook.com
//! @date:2023/8/23
//! @version:0.0.1
//! @description:
//! ```
use std::fmt::{Display, Formatter};
use serde::{Serialize, Deserialize};
use crate::db::{SurrealValue, ParamCombine};
use crate::db::constants::{ADD, ADD_OP, ALLEQ, BOTH, DIVIDE, EITHER, GT_EQ, HAS, INLIKE, IS, ISNOT, LIKE, LT_EQ, MATCHES, MINUS, NOTLIKE, PLUS, POW, EQ, LT, GT, GTE, LTE, LINK, NEQ, WHERE, AND, OR, CONTAINS, CONTAINSALL, CONTAINSNOT, CONTAINSANY, CONTAINSNONE, INSIDE, NOTINSIDE, ALLINSIDE, ANYINSIDE, NONEINSIDE, OUTSIDE, INTERSECTS, SELECT_ALL, STMT_END};

/// where condition for statment
/// ## example
/// ```rust
/// use surrealism::db::{ConditionSign,Condition,Criteria,ParamCombine,SurrealValue,CriteriaSign};
///     // WHERE username = 'Mat' AND age != 16
///     let condition = Condition::new()
///         .push(Criteria::new("username","Mat", CriteriaSign::Eq), ConditionSign::And)
///         .push(Criteria::new("age", 16, CriteriaSign::Neq), ConditionSign::None)
///         .deref_mut();
///     dbg!(condition.combine());
///     // WHERE -> knows -> person -> (knows WHERE influencer = true)
///     // use cheat to build complex statements
///     let link = Condition::new()
///         .push(Criteria::new("knows", "person", CriteriaSign::Link), ConditionSign::Link)
///         .push(Criteria::cheat("knows","influencer = true","WHERE"),ConditionSign::None)
///         .deref_mut();
///     dbg!(link.combine());
/// ```
#[derive(Debug, Clone, PartialEq)]
pub struct Condition(Vec<(Criteria, ConditionSign)>);

impl Condition {
    /// new condition
    pub fn new() -> Self {
        Condition(Vec::new())
    }
    /// push a criteria with sign to Condition
    pub fn push(&mut self, criteria: Criteria, sign: ConditionSign) -> &mut Self {
        self.0.push((criteria, sign));
        self
    }
    pub fn deref_mut(&mut self) -> Self {
        Condition(self.0.clone())
    }
    pub fn build(&self) -> String {
        self.to_string()
    }
}

impl Display for Condition {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut res = String::new();
        // pre for ConditionSign
        // use if ConditionSign::Links
        let mut pre_pointer = ConditionSign::None;
        for (criteria, sign) in &self.0 {
            match sign {
                ConditionSign::None => {
                    let mut tmp = criteria.combine();
                    if pre_pointer.is_link() {
                        tmp = format!("({})", &tmp);
                    }
                    res.push_str(&tmp);
                }
                _ => {
                    res.push_str(format!("{} {} ", &criteria.combine(), sign.to_str()).as_str());
                }
            };
            pre_pointer = sign.clone();
        }
        write!(f, "{}", res)
    }
}

impl ParamCombine for Condition {
    fn combine(&self) -> String {
        format!("{} {}", WHERE, self.build())
    }
}

/// #  condition sign
/// 作用于 Condition
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ConditionSign {
    And,
    Or,
    Link,
    None,
}

impl Default for ConditionSign {
    fn default() -> Self {
        ConditionSign::None
    }
}

impl Display for ConditionSign {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_str())
    }
}

impl ConditionSign {
    pub fn to_str(&self) -> &str {
        match self {
            ConditionSign::And => AND,
            ConditionSign::Or => OR,
            ConditionSign::Link => LINK,
            ConditionSign::None => "",
        }
    }
    pub fn is_and(&self) -> bool {
        match self {
            ConditionSign::And => true,
            _ => false
        }
    }
    pub fn is_or(&self) -> bool {
        match self {
            ConditionSign::Or => true,
            _ => false
        }
    }
    pub fn is_link(&self) -> bool {
        match self {
            ConditionSign::Link => true,
            _ => false
        }
    }
    pub fn is_none(&self) -> bool {
        match self {
            ConditionSign::None => true,
            _ => false
        }
    }
}

impl From<&str> for ConditionSign {
    fn from(value: &str) -> Self {
        match value {
            AND => ConditionSign::And,
            OR => ConditionSign::Or,
            LINK => ConditionSign::Link,
            _ => ConditionSign::None
        }
    }
}

impl From<String> for ConditionSign {
    fn from(value: String) -> Self {
        ConditionSign::from(value.as_str())
    }
}

/// # criteria
/// left sign right
/// as :
/// - name = 'Mat;
/// - age != 10
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Criteria {
    left: String,
    right: SurrealValue,
    sign: CriteriaSign,
}

impl Criteria {
    /// new a easy criteria : left = right
    pub fn new<T>(left: &str, right: T, sign: CriteriaSign) -> Self where T: Serialize, {
        Criteria {
            left: String::from(left),
            right: SurrealValue::from(serde_json::to_value(right).unwrap()),
            sign,
        }
    }
    /// use when define field
    pub fn new_field<T>(right: T, sign: CriteriaSign) -> Self where T: Serialize, {
        Criteria {
            left: String::from("$value"),
            right: SurrealValue::from(serde_json::to_value(right).unwrap()),
            sign,
        }
    }
    pub fn new_event<T>(right: T, sign: CriteriaSign) -> Self where T: Serialize, {
        Criteria {
            left: String::from("$event"),
            right: SurrealValue::from(serde_json::to_value(right).unwrap()),
            sign,
        }
    }
    /// This is a simple but unreasonable method to new Criteria
    pub fn new_easy<T>(left: T, right: T, sign: CriteriaSign) -> Self where T: Serialize, {
        Criteria {
            left: SurrealValue::from(serde_json::to_value(left).unwrap()).inner_str().unwrap(),
            right: SurrealValue::from(serde_json::to_value(right).unwrap()),
            sign,
        }
    }
    /// # Cheat Condition Builder
    /// When encountering difficulties in directly constructing statements with conditional constructors
    ///
    /// like:  knows WHERE influencer = true
    pub fn cheat(left: &str, right: &str, sign: &str) -> Self {
        Criteria {
            left: String::from(left),
            right: SurrealValue::from(right),
            sign: CriteriaSign::Cheat(String::from(sign)),
        }
    }
    pub fn left(&mut self, left: &str) -> &mut Self {
        self.left = String::from(left);
        self
    }
    pub fn right<T>(&mut self, right: T) -> &mut Self where T: Serialize, {
        self.right = SurrealValue::from(serde_json::to_value(right).unwrap());
        self
    }
    pub fn sign(&mut self, sign: CriteriaSign) -> &mut Self {
        self.sign = sign;
        self
    }
    pub fn sign_from_str(&mut self, sign: &str) -> &mut Self {
        self.sign = CriteriaSign::from(sign);
        self
    }
    pub fn build(&self) -> String {
        self.to_string()
    }
    /// consume self to SurrealValue
    pub fn to_value(self) -> SurrealValue {
        SurrealValue::from(self.build())
    }
    /// build easy : select * from ... ;
    pub fn build_easy(&self) -> String {
        format!("{} {}{}", SELECT_ALL, &self.to_string(), STMT_END)
    }
}

impl Display for Criteria {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self.sign {
            CriteriaSign::Link => {
                match self.right {
                    SurrealValue::String(ref s) => write!(f, "{} {} {} {}", self.sign.to_str(), &self.left, LINK, s),
                    _ => panic!("{}", "Link Multiple Tables need use SurrealValue::Str for right!")
                }
            }
            // for cheat
            CriteriaSign::Cheat(ref value) => {
                match self.right {
                    SurrealValue::String(ref s) => write!(f, "{} {} {}", &self.left, value, s),
                    _ => panic!("{}", "This Panic may not exist , if you see this panic , please connect to author or commit issue on github!")
                }
            }
            _ => write!(f, "{} {} {}", &self.left, self.sign.to_str(), &self.right.to_string())
        }
    }
}

impl ParamCombine for Criteria {
    fn combine(&self) -> String {
        self.build()
    }
}


/// # criteria sign
/// 作用于Criteria是SurrealDB的操作符
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum CriteriaSign {
    /// &&
    And,
    /// ||
    Or,
    /// ??
    Either,
    /// ?:
    Both,
    /// = 会进行隐式转换
    Is,
    /// !=
    IsNot,
    /// ?= 检查数组中的任何值是否等于另一个值
    Has,
    /// *= 检查数组中的所有值是否都等于另一个值
    AllEq,
    /// ~  使用模糊匹配比较两个值是否相等
    Like,
    /// !~  使用模糊匹配比较两个值的不等式
    NotLike,
    /// ?~ 检查集合中的任何值是否等于使用模糊匹配的值
    InLike,
    /// <
    Lt,
    /// <=
    Leq,
    /// >
    Gt,
    /// >=
    Geq,
    Add,
    Minus,
    /// * 或 ×
    Plus,
    /// / 或÷
    Divide,
    /// == 检查两个值是否精确。该运算符还检查每个值是否具有相同的类型
    Eq,
    /// 幂运算 **
    Pow,
    /// CONTAINS 或 ∋ 检查一个值是否包含其他值
    Contains,
    /// CONTAINSNOT 或 ∌ 检查一个值是否不包含其他值
    ContainsNot,
    /// CONTAINSALL 或 ⊇ 检查一个值是否包含所有多个值
    ContainsAll,
    /// CONTAINSANY 或 ⊃ 检查一个值是否包含多个值中的任何一个
    ContainsAny,
    /// CONTAINSNONE 或 ⊅ 检查一个值是否不包含任何多个值。
    ContainsNone,
    /// INSIDE 或 ∈ 或 IN 检查一个值是否包含在另一个值中
    Inside,
    /// NOTINSIDE 或 ∉ 或NOT IN 检查一个值是否不包含在另一个值中。
    NotInside,
    /// ALLINSIDE 或 ⊆ 检查所有多个值是否包含在另一个值中
    AllInside,
    /// ANYINSIDE 或 ⊂ 检查多个值中的任何一个是否包含在另一个值中
    AnyInside,
    /// NONEINSIDE 或 ⊄ 检查多个值中是否没有一个包含在另一个值中
    NoneInside,
    /// OUTSIDE 检查一个几何体值是否在另一个几何体值之外
    Outside,
    /// INTERSECTS 检查一个几何值是否与另一个几何值相交
    Intersects,
    /// MATCHES 检查是否在全文索引字段中找到术语
    Matches,
    Link,
    Cheat(String),
}

impl Default for CriteriaSign {
    fn default() -> Self {
        CriteriaSign::Eq
    }
}

impl ParamCombine for CriteriaSign {
    fn combine(&self) -> String {
        String::from(self.to_str())
    }
}

impl Display for CriteriaSign {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_str())
    }
}

impl CriteriaSign {
    /// # quick build and select
    ///
    /// ## example
    /// ```rust
    /// use surrealism::builder::SQLBuilderFactory;
    /// use surrealism::db::{Criteria, CriteriaSign};
    /// use surrealism::DefaultRes;
    ///
    /// // [tests\src\main.rs:13] operator1 = "SELECT * FROM 10 AND 20;"
    /// // [tests\src\main.rs:14] operator2 = "SELECT * FROM 0 OR false;"
    /// // [tests\src\main.rs:15] operator3 = "SELECT * FROM NULL ?? 0;"
    /// // [tests\src\main.rs:16] operator4 = "SELECT * FROM true ?: 1;"
    /// // [tests\src\main.rs:17] operator5 = "SELECT * FROM 'test text' ~ 'Test';"
    /// #[tokio::main]
    /// async fn main() -> DefaultRes<()> {
    ///     let operator1 = CriteriaSign::And.build(10.into(), 20.into());
    ///     let operator2 = CriteriaSign::Or.build(0.into(), false.into());
    ///     let operator3 = CriteriaSign::Either.build("NULL".into(), 0.into());
    ///     let operator4 = CriteriaSign::Both.build(true.into(), 1.into());
    ///     let operator5 = CriteriaSign::Like.build("test text".into(), "Test".into());
    ///     Ok(())
    /// }
    /// ```
    pub fn build(self, left: SurrealValue, right: SurrealValue) -> String {
        format!("{} {} {} {}{}", SELECT_ALL, &left.to_string(), self.to_str(), &right.to_string(),STMT_END)
    }
    pub fn to_str(&self) -> &str {
        match self {
            CriteriaSign::Eq => EQ,
            CriteriaSign::Lt => LT,
            CriteriaSign::Gt => GT,
            CriteriaSign::Link => LINK,
            CriteriaSign::Cheat(value) => value.as_str(),
            CriteriaSign::And => AND,
            CriteriaSign::Or => OR,
            CriteriaSign::Either => EITHER,
            CriteriaSign::Both => BOTH,
            CriteriaSign::Is => IS,
            CriteriaSign::IsNot => ISNOT,
            CriteriaSign::Has => HAS,
            CriteriaSign::AllEq => ALLEQ,
            CriteriaSign::Like => LIKE,
            CriteriaSign::NotLike => NOTLIKE,
            CriteriaSign::InLike => INLIKE,
            CriteriaSign::Leq => LT_EQ,
            CriteriaSign::Geq => GT_EQ,
            CriteriaSign::Add => ADD,
            CriteriaSign::Minus => MINUS,
            CriteriaSign::Plus => PLUS,
            CriteriaSign::Divide => DIVIDE,
            CriteriaSign::Pow => POW,
            CriteriaSign::Contains => CONTAINS,
            CriteriaSign::ContainsNot => CONTAINSNOT,
            CriteriaSign::ContainsAll => CONTAINSALL,
            CriteriaSign::ContainsAny => CONTAINSANY,
            CriteriaSign::ContainsNone => CONTAINSNONE,
            CriteriaSign::Inside => INSIDE,
            CriteriaSign::NotInside => NOTINSIDE,
            CriteriaSign::AllInside => ALLINSIDE,
            CriteriaSign::AnyInside => ANYINSIDE,
            CriteriaSign::NoneInside => NONEINSIDE,
            CriteriaSign::Outside => OUTSIDE,
            CriteriaSign::Intersects => INTERSECTS,
            CriteriaSign::Matches => MATCHES
        }
    }
}

impl From<&str> for CriteriaSign {
    fn from(value: &str) -> Self {
        match value {
            MATCHES => CriteriaSign::Matches,
            INTERSECTS => CriteriaSign::Intersects,
            OUTSIDE => CriteriaSign::Outside,
            NONEINSIDE => CriteriaSign::NoneInside,
            ANYINSIDE => CriteriaSign::AnyInside,
            ALLINSIDE => CriteriaSign::AllInside,
            NOTINSIDE => CriteriaSign::NotInside,
            INSIDE => CriteriaSign::Inside,
            CONTAINSNONE => CriteriaSign::ContainsNone,
            CONTAINSANY => CriteriaSign::ContainsAny,
            CONTAINSALL => CriteriaSign::ContainsAll,
            CONTAINSNOT => CriteriaSign::ContainsNot,
            CONTAINS => CriteriaSign::Contains,
            POW => CriteriaSign::Pow,
            DIVIDE => CriteriaSign::Divide,
            PLUS => CriteriaSign::Plus,
            MINUS => CriteriaSign::Minus,
            ADD => CriteriaSign::Add,
            GT_EQ => CriteriaSign::Geq,
            LT_EQ => CriteriaSign::Leq,
            INLIKE => CriteriaSign::InLike,
            NOTLIKE => CriteriaSign::NotLike,
            LIKE => CriteriaSign::Like,
            ALLEQ => CriteriaSign::AllEq,
            HAS => CriteriaSign::Has,
            ISNOT => CriteriaSign::IsNot,
            IS => CriteriaSign::Is,
            BOTH => CriteriaSign::Both,
            EITHER => CriteriaSign::Either,
            OR => CriteriaSign::Or,
            AND => CriteriaSign::And,
            LT => CriteriaSign::Lt,
            GT => CriteriaSign::Gt,
            LINK => CriteriaSign::Link,
            _ => CriteriaSign::Cheat(String::from(value))
        }
    }
}

impl From<String> for CriteriaSign {
    fn from(value: String) -> Self {
        CriteriaSign::from(value.as_str())
    }
}