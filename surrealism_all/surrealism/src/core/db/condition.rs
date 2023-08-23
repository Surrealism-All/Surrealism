//! # where condition
//! ```txt
//! @author:syf20020816@Outlook.com
//! @date:2023/8/23
//! @version:0.0.1
//! @description:
//! ```
use crate::SurrealValue;
use super::{ParamCombine};
use super::constants::{EQ, LT, GT, GTE, LTE, LINK, NEQ, WHERE, BLANK, AND, OR};

/// where condition for statment
/// ## example
/// ```rust
/// use surrealism::{ConditionSign,Condition,Criteria,ParamCombine,SurrealValue,CriteriaSign};
///     // WHERE username = 'Mat' AND age != 16
///     let condition = Condition::new()
///         .push(Criteria::new("username", SurrealValue::Str(String::from("Mat")), CriteriaSign::Eq), ConditionSign::And)
///         .push(Criteria::new("age", SurrealValue::Int(16), CriteriaSign::Neq), ConditionSign::None)
///         .deref_mut();
///     dbg!(condition.combine());
///     // WHERE -> knows -> person -> (knows WHERE influencer = true)
///     // use cheat to build complex statements
///     let link = Condition::new()
///         .push(Criteria::new("knows", SurrealValue::from("person"), CriteriaSign::Link), ConditionSign::Link)
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
        res
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
#[derive(Debug, Clone, PartialEq)]
pub struct Criteria {
    left: String,
    right: SurrealValue,
    sign: CriteriaSign,
}

impl Criteria {
    /// new a easy criteria : left = right
    pub fn new(left: &str, right: SurrealValue, sign: CriteriaSign) -> Self {
        Criteria {
            left: String::from(left),
            right,
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
    pub fn right(&mut self, right: SurrealValue) -> &mut Self {
        self.right = right;
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
        match self.sign {
            CriteriaSign::Link => {
                match self.right {
                    SurrealValue::Str(ref s) => format!("{} {} {} {}", self.sign.to_str(), &self.left, LINK, s),
                    _ => panic!("{}", "Link Multiple Tables need use SurrealValue::Str for right!")
                }
            }
            // for cheat
            CriteriaSign::Cheat(ref value) => {
                match self.right {
                    SurrealValue::Str(ref s) => format!("{} {} {}", &self.left, value, s),
                    _ => panic!("{}", "This Panic may not exist , if you see this panic , please connect to author or commit issue on github!")
                }
            }
            _ => format!("{} {} {}", &self.left, self.sign.to_str(), &self.right.to_str())
        }
    }
}

impl ParamCombine for Criteria {
    fn combine(&self) -> String {
        self.build()
    }
}


/// # criteria sign
/// 作用于Criteria
/// - Eq:等于
/// - Lt:小于
/// - Gt:大于
/// - Neq:不等于
/// - Lte:小于等于
/// - Gte:大于等于
/// - Link:连接`->`
#[derive(Debug, Clone, PartialEq)]
pub enum CriteriaSign {
    Eq,
    Lt,
    Gt,
    Neq,
    Lte,
    Gte,
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

impl CriteriaSign {
    pub fn to_str(&self) -> &str {
        match self {
            CriteriaSign::Eq => EQ,
            CriteriaSign::Lt => LT,
            CriteriaSign::Gt => GT,
            CriteriaSign::Neq => NEQ,
            CriteriaSign::Lte => LTE,
            CriteriaSign::Gte => GTE,
            CriteriaSign::Link => LINK,
            CriteriaSign::Cheat(value) => value.as_str()
        }
    }
}

impl From<&str> for CriteriaSign {
    fn from(value: &str) -> Self {
        match value {
            EQ => CriteriaSign::Eq,
            LT => CriteriaSign::Lt,
            GT => CriteriaSign::Gt,
            LTE => CriteriaSign::Lte,
            GTE => CriteriaSign::Gte,
            NEQ => CriteriaSign::Neq,
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