//! # Define Wrapper
//! ```txt
//! @author:syf20020816@Outlook.com
//! @date:2023/8/29
//! @version:0.0.1
//! @description:
//! ```
//!
mod ns;
mod db;
mod table;
mod scope;
mod field;
mod user;
mod token;
mod event;
mod index;
mod param;
mod function;
mod analyzer;

pub use ns::DefineNS;
pub use db::DefineDB;
pub use table::DefineTable;
pub use event::DefineEvent;
pub use scope::DefineScope;
pub use user::DefineUser;
pub use token::DefineToken;
pub use field::DefineField;
pub use analyzer::DefineAnalyzer;
pub use function::DefineFunction;
pub use param::DefineParam;
pub use index::DefineIndex;



use std::fmt::{Display, Formatter};
use crate::db::{Condition, ParamCombine};
use crate::db::constants::{ROLES, DEFINE_USER, NAMESPACE, DATABASE, PASSHASH, PASSWORD, ROOT, TABLE, DEFINE_DB, DEFINE_NS, DEFINE_LOGIN, DEFINE_SCOPE, STMT_END, ON, TYPE, SCOPE, PS256, PS384, PS512, EDDSA, ES256, ES384, ES512, HS256, HS384, HS512, RS256, RS384, RS512, VALUE, DEFINE_TOKEN, SCHEMA_FULL, SCHEMA_LESS, SIGN_IN, SIGN_UP, DROP, DEFINE_TABLE, BLANK, NONE, FULL, FOR, DEFINE_EVENT, ON_TABLE, WHEN, THEN, DEFINE_FUNCTION, RETURN, DEFINE_FIELD, FIELDS, COLUMNS, DEFINE_INDEX, UNIQUE, DEFINE_PARAM};



/// # DefineWrapper
/// The DEFINE statement can be used to specify authentication access and behaviour, global parameters, table configurations, table events, schema definitions, and indexes.
/// ## example
/// ```rust
#[derive(Debug, Clone)]
pub struct DefineWrapper;

impl DefineWrapper {
    pub fn new() -> Self {
        DefineWrapper
    }
    ///使用DEFINE USER语句在SurrealDB上创建用户帐户
    ///- 必须以root或命名空间用户身份进行身份验证，才能使用DEFINE USER 声明。
    ///- 必须以root、命名空间或数据库用户身份进行身份验证，才能使用DEFINE USER 声明。
    ///- 必须选择命名空间和/或数据库 才能使用DEFINE USER 声明。
    /// > 注意：您不能使用DEFINE USER 语句创建根或SCOPE 用户。
    pub fn user<'w>(self) -> DefineUser<'w> {
        DefineUser::default()
    }
    /// SurrealDB有一个多租户模型，它允许您将数据库的范围限定到一个名称空间。数据库的数量没有限制 可以在名称空间中，也没有对允许的名称空间的数量的限制。只有root用户有权 创建命名空间。
    /// - 您必须作为root用户进行身份验证，才能使用`DEFINE NAMESPACE`声明。
    pub fn ns<'w>(self) -> DefineNS<'w> {
        DefineNS::default()
    }
    /// 该DEFINE DATABASE 语句使您可以实例化命名数据库，从而可以指定 安全和配置选项。
    /// - 必须以root用户或命名空间用户身份进行身份验证，然后才能使用DEFINE DATABASE 声明。
    /// - 必须选择命名空间 才能使用DEFINE DATABASE 声明。
    pub fn db<'w>(self) -> DefineDB<'w> {
        DefineDB::default()
    }

    ///  SurrealDB可以与第三方OAuth提供商合作。假设您的提供者在您的服务通过身份验证后向其发布JWT。 通过使用DEFINE TOKEN 语句，您可以设置验证JWT真实性所需的公钥。
    ///
    /// 您可以指定什么TYPE 您的令牌使用的加密签名算法。支持以下算法：
    /// `EDDSA， ES256， ES384， ES512， HS256， HS384， HS512， PS256， PS384， PS512， RS256， RS384， RS512`
    /// - 到DEFINE TOKEN ... ON NAMESPACE ... 你必须有根或命名空间级别的访问权限。
    /// - 到DEFINE TOKEN ... ON DATABASE ... 必须具有根、命名空间或数据库级别的访问权限。
    /// - 到DEFINE TOKEN ... ON SCOPE ... 必须具有根、命名空间或数据库级别的访问权限。
    /// - 必须选择命名空间和/或数据库 才能使用DEFINE DATABASE 数据库或命名空间标记的语句。
    pub fn token<'w>(self) -> DefineToken<'w> {
        DefineToken::default()
    }
    pub fn scope<'w>(self) -> DefineScope<'w> {
        DefineScope::default()
    }
    /// 该DEFINE TABLE 语句允许您按名称声明表，从而可以应用严格的 控件添加到表的架构中，方法是将SCHEMAFULL，创建外部表视图，并设置权限 指定可以在字段上执行什么操作。
    /// - 必须作为根用户、命名空间用户或数据库用户进行身份验证，才能使用DEFINE TABLE 声明。
    /// - 必须选择命名空间和数据库 才能使用DEFINE TABLE 声明。
    pub fn table<'w>(self) -> DefineTable<'w> {
        DefineTable::default()
    }
    /// 事件可以在对记录中的数据进行任何更改或修改之后触发。每个触发器都能看到 The$before 和/或$after 值，从而为每个触发器启用高级自定义逻辑。
    /// - 必须作为根用户、命名空间用户或数据库用户进行身份验证，才能使用DEFINE EVENT 声明。
    /// - 必须选择命名空间和数据库 才能使用DEFINE EVENT 声明。
    pub fn event<'w>(self) -> DefineEvent<'w> {
        DefineEvent::default()
    }
    /// 该DEFINE FUNCTION 语句允许您定义可在整个数据库中重用的自定义函数。
    /// - 必须作为根用户、命名空间用户或数据库用户进行身份验证，才能使用DEFINE FUNCTION 声明。
    /// - 必须选择命名空间和数据库 才能使用DEFINE FUNCTION 声明。
    pub fn function<'w>(self) -> DefineFunction<'w> {
        DefineFunction::default()
    }
    /// 该DEFINE FIELD 语句允许您实例化表中的命名字段，使您能够设置 字段的数据类型、设置默认值、应用断言以保护数据一致性以及设置权限 指定可以在字段上执行什么操作。
    /// - 必须作为根用户、命名空间用户或数据库用户进行身份验证，才能使用DEFINE FIELD 声明。
    /// - 必须选择命名空间和数据库 才能使用DEFINE FIELD 声明。
    pub fn field<'w>(self) -> DefineField<'w> {
        DefineField::default()
    }
    /// 就像在其他数据库中一样，SurrealDB使用索引来帮助优化查询性能。索引可以包括 表中的一个或多个字段，并且可以强制唯一性约束。如果您不希望索引具有 唯一性约束，则为索引选择的字段应具有高度的基数， 这意味着在索引表记录中的数据之间存在大量的多样性。
    /// - 必须作为根用户、命名空间用户或数据库用户进行身份验证，才能使用DEFINE INDEX 声明。
    /// - 必须选择命名空间和数据库 才能使用DEFINE INDEX 声明。
    pub fn index<'w>() -> DefineIndex<'w> {
        DefineIndex::default()
    }
    /// 该DEFINE PARAM 语句允许您定义可用于每个客户端的全局（数据库范围）参数。
    /// - 必须作为根用户、命名空间用户或数据库用户进行身份验证，才能使用DEFINE PARAM 声明。
    /// - 必须选择命名空间和数据库 才能使用DEFINE PARAM 声明。
    pub fn param<'w>(self) -> DefineParam<'w> {
        DefineParam::default()
    }
}

#[derive(Debug, Clone)]
pub enum OnType<'o> {
    DB,
    NS,
    ROOT,
    TABLE(&'o str),
    SCOPE(&'o str),
}

impl<'o> Default for OnType<'o> {
    fn default() -> Self {
        OnType::ROOT
    }
}

impl<'o> Display for OnType<'o> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let res = match self {
            OnType::DB => DATABASE.to_string(),
            OnType::NS => NAMESPACE.to_string(),
            OnType::ROOT => ROOT.to_string(),
            OnType::TABLE(table) => format!("{} {}", TABLE, table),
            OnType::SCOPE(scope) => format!("{} {}", SCOPE, scope)
        };
        write!(f, "{}", res)
    }
}

impl<'o> OnType<'o> {
    pub fn is_scope(&self) -> bool {
        match self {
            OnType::SCOPE(_) => true,
            _ => false
        }
    }
    pub fn is_ns(&self) -> bool {
        match self {
            OnType::NS => true,
            _ => false
        }
    }
    pub fn is_db(&self) -> bool {
        match self {
            OnType::DB => true,
            _ => false
        }
    }
    pub fn is_root(&self) -> bool {
        match self {
            OnType::ROOT => true,
            _ => false
        }
    }
    pub fn is_table(&self) -> bool {
        match self {
            OnType::TABLE(_) => true,
            _ => false
        }
    }
    pub fn on_user(&self) -> bool {
        match self {
            OnType::ROOT | OnType::NS | OnType::DB => true,
            _ => false
        }
    }
    ///judge when in token
    pub fn on_token(&self) -> bool {
        match self {
            OnType::DB | OnType::NS | OnType::SCOPE(_) => true,
            _ => false
        }
    }
}

#[derive(Debug, Clone)]
pub enum PwdType<'p> {
    Pwd(&'p str),
    Hash(&'p str),
}

impl<'p> Display for PwdType<'p> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let res = match self {
            PwdType::Pwd(pwd) => format!("{} {}", PASSWORD, pwd),
            PwdType::Hash(hash) => format!("{} {}", PASSHASH, hash),
        };
        write!(f, "{}", res)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum TokenType {
    EDDSA,
    ES256,
    ES384,
    ES512,
    HS256,
    HS384,
    HS512,
    PS256,
    PS384,
    PS512,
    RS256,
    RS384,
    RS512,
}

impl Default for TokenType {
    fn default() -> Self {
        TokenType::HS512
    }
}

impl Display for TokenType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let res = match self {
            TokenType::EDDSA => EDDSA,
            TokenType::ES256 => ES256,
            TokenType::ES384 => ES384,
            TokenType::ES512 => ES512,
            TokenType::HS256 => HS256,
            TokenType::HS384 => HS384,
            TokenType::HS512 => HS512,
            TokenType::PS256 => PS256,
            TokenType::PS384 => PS384,
            TokenType::PS512 => PS512,
            TokenType::RS256 => RS256,
            TokenType::RS384 => RS384,
            TokenType::RS512 => RS512,
        };
        write!(f, "{} {}", TYPE, res)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Schema {
    SCHEMAFULL,
    SCHEMALESS,
}

impl Display for Schema {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let res = match self {
            Schema::SCHEMAFULL => SCHEMA_FULL,
            Schema::SCHEMALESS => SCHEMA_LESS,
        };
        write!(f, "{}", res)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Permissions {
    NONE,
    FULL,
    FOR {
        select: Condition,
        create: Condition,
        update: Condition,
        delete: Condition,
    },
}

impl Display for Permissions {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let res = match self {
            Permissions::NONE => String::from(NONE),
            Permissions::FULL => String::from(FULL),
            Permissions::FOR { select, create, update, delete } =>
                format!("{} {} {} {} {} {} {} {}", FOR, select.combine(), FOR, create.combine(), FOR, update.combine(), FOR, delete.combine())
        };
        write!(f, "{}", res)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum FieldColumn<'f> {
    FIELDS(Vec<&'f str>),
    COLUMNS(Vec<&'f str>),
}

impl<'f> Default for FieldColumn<'f> {
    fn default() -> Self {
        FieldColumn::COLUMNS(Vec::new())
    }
}

impl<'f> From<Vec<&'f str>> for FieldColumn<'f> {
    fn from(value: Vec<&'f str>) -> Self {
        FieldColumn::COLUMNS(value)
    }
}

impl<'f> Display for FieldColumn<'f> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            FieldColumn::FIELDS(field) => write!(f, "{} {}", FIELDS, field.join(" , ")),
            FieldColumn::COLUMNS(c) => write!(f, "{} {}", COLUMNS, c.join(" , ")),
        }
    }
}