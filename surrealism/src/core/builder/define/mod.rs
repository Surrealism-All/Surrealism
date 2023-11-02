//! # Define Wrapper
//! ```rust
//! use surrealism::DefaultRes;
//! use surrealism::builder::{BaseWrapperImpl, SQLBuilderFactory, TableImpl};
//! use surrealism::builder::define::{FieldColumn, OnType, PwdType, TokenType, UniqueSearch};
//! use surrealism::builder::select::SelectWrapperImpl;
//! use surrealism::db::{Condition, ConditionSign, Criteria, CriteriaSign, Roles, TimeUnit, ValueConstructor, ValueType};
//!
//! // [tests\src\main.rs:98] define_ns = "DEFINE NAMESPACE test;"
//! // [tests\src\main.rs:99] define_db = "DEFINE NAMESPACE test;"
//! // [tests\src\main.rs:100] define_user = "DEFINE USER username ON ROOT PASSWORD '123456' ROLES OWNER;"
//! // [tests\src\main.rs:101] define_token.rs = "DEFINE TOKEN test_tk ON NAMESPACE TYPE HS512 VALUE 'sNSYneezcr8kqphfOC6NwwraUHJCVAt0XjsRSNmssBaBRh3WyMa9TRfq8ST7fsU2H2kGiOpU4GbAF1bCiXmM1b3JGgleBzz7rsrz6VvYEM4q3CLkcO8CMBIlhwhzWmy8';"
//! // [tests\src\main.rs:102] define_scope = "DEFINE SCOPE account TIMEOUT 24h SIGNUP ( CREATE user SET email = $email, pass = crypto::argon2::generate($pass) ) SIGNIN ( SELECT * FROM user WHERE email = $email AND crypto::argon2::compare(pass, $pass) );"
//! // [tests\src\main.rs:103] define_table1 = "DEFINE TABLE reading;"
//! // [tests\src\main.rs:104] define_table2 = "DEFINE TABLE reading DROP;"
//! // [tests\src\main.rs:105] define_table3 = "DEFINE TABLE reading CHANGEFEED 1d;"
//! // [tests\src\main.rs:106] define_table4 = "DEFINE TABLE  SCHEMAFULL;"
//! // [tests\src\main.rs:107] define_table5 = "DEFINE TABLE temperatures_by_month AS SELECT count() AS total FROM reading GROUP BY city;"
//! // [tests\src\main.rs:108] define_event = "DEFINE EVENT  ON TABLE TABLE user WHEN 1@mail.com != '2@mail.com' THEN CREATE event SET user = $value.id, time = time::now(), value = $after.email, action = 'email_changed';"
//! // [tests\src\main.rs:109] define_func = "DEFINE FUNCTION fn::great($name:string) {  'Hello, '+$name + '!' };"
//! // [tests\src\main.rs:110] define_field1 = "DEFINE FIELD email ON TABLE TABLE user TYPE string;"
//! // [tests\src\main.rs:111] define_field2 = "DEFINE FIELD locked ON TABLE TABLE user FLEXIBLE TYPE bool DEFAULT true VALUE true;"
//! // [tests\src\main.rs:112] define_analyzer1 = "DEFINE ANALYZER example_camel TOKENIZERS camel;"
//! // [tests\src\main.rs:113] define_analyzer2 = "DEFINE ANALYZER code TOKENIZERS class, camel FILTERS lowercase, ascii;"
//! // [tests\src\main.rs:114] define_index1 = "DEFINE INDEX userEmailIndex ON TABLE user COLUMNS email UNIQUE;"
//! // [tests\src\main.rs:115] define_index2 = "DEFINE INDEX userEmailIndex ON TABLE user COLUMNS name SEARCH ANALYZER ascii BM25  HIGHLIGHTS;"
//! // [tests\src\main.rs:116] define_param = "DEFINE PARAM $endpointBase VALUE 'https://dummyjson.com';"
//! #[tokio::main]
//! async fn main() -> DefaultRes<()> {
//!     let define_ns = SQLBuilderFactory::define().ns().name("test").to_string();
//!     let define_db = SQLBuilderFactory::define().db().name("test").to_string();
//!     let define_user = SQLBuilderFactory::define()
//!         .user()
//!         .name("username")
//!         .on(OnType::ROOT)
//!         .pwd(PwdType::Pwd("123456"))
//!         .role(Roles::OWNER)
//!         .build();
//!     let define_token = SQLBuilderFactory::define().token()
//!         .name("test_tk")
//!         .on(OnType::NS)
//!         .token_type(TokenType::HS512)
//!         .value("sNSYneezcr8kqphfOC6NwwraUHJCVAt0XjsRSNmssBaBRh3WyMa9TRfq8ST7fsU2H2kGiOpU4GbAF1bCiXmM1b3JGgleBzz7rsrz6VvYEM4q3CLkcO8CMBIlhwhzWmy8")
//!         .build();
//!     let define_scope = SQLBuilderFactory::define().scope()
//!         .name("account")
//!         .session(24, TimeUnit::HOUR)
//!         .sign_up("CREATE user SET email = $email, pass = crypto::argon2::generate($pass)")
//!         .sign_in("SELECT * FROM user WHERE email = $email AND crypto::argon2::compare(pass, $pass)")
//!         .build();
//!     let define_table1 = SQLBuilderFactory::define().table()
//!         .name("reading")
//!         .build();
//!     let define_table2 = SQLBuilderFactory::define().table()
//!         .name("reading")
//!         .drop()
//!         .build();
//!     let define_table3 = SQLBuilderFactory::define().table()
//!         .name("reading")
//!         .changefeed(1, TimeUnit::DAY)
//!         .build();
//!     let define_table4 = SQLBuilderFactory::define().table()
//!         .schemafull()
//!         .build();
//!     let as_select = SQLBuilderFactory::select().table("reading").column("count()", Some("total")).group_by(vec!["city"]).to_string();
//!     let define_table5 = SQLBuilderFactory::define().table()
//!         .name("temperatures_by_month")
//!         .as_expression(&as_select)
//!         .build();
//!     let define_event = SQLBuilderFactory::define().event()
//!         .on("user")
//!         .when(Condition::new().push(Criteria::new("1@mail.com", "2@mail.com", CriteriaSign::Neq), ConditionSign::None).deref_mut())
//!         .then("CREATE event SET user = $value.id, time = time::now(), value = $after.email, action = 'email_changed'")
//!         .build();
//!     let define_func = SQLBuilderFactory::define().function()
//!         .name("great")
//!         .args(vec!["$name:string"])
//!         .returned("'Hello, '+$name + '!'")
//!         .build();
//!     let define_field1 = SQLBuilderFactory::define().field()
//!         .name("email")
//!         .on("user")
//!         .value(ValueConstructor::new(ValueType::String, None, None, None, false))
//!         .build();
//!
//!     let define_field2 = SQLBuilderFactory::define().field()
//!         .name("locked")
//!         .on("user")
//!         .value(ValueConstructor::new(ValueType::Bool, Some(true.into()), Some(true.into()), None, true))
//!         .build();
//!     let define_analyzer1 = SQLBuilderFactory::define().analyzer()
//!         .name("example_camel")
//!         .tokenizer("camel")
//!         .build();
//!     let define_analyzer2 = SQLBuilderFactory::define().analyzer()
//!         .name("code")
//!         .tokenizer("class")
//!         .tokenizer("camel")
//!         .filter("lowercase")
//!         .filter("ascii")
//!         .build();
//!     let define_index1 = SQLBuilderFactory::define().index()
//!         .name("userEmailIndex")
//!         .on("user")
//!         .field_column(FieldColumn::COLUMNS(vec!["email"]))
//!         .unique_search(UniqueSearch::Unique)
//!         .build();
//!     let define_index2 = SQLBuilderFactory::define().index()
//!         .name("userEmailIndex")
//!         .on("user")
//!         .field_column(FieldColumn::COLUMNS(vec!["name"]))
//!         .unique_search(UniqueSearch::search("ascii").push("", "").highlight().clone())
//!         .build();
//!     let define_param = SQLBuilderFactory::define().param()
//!         .name("endpointBase")
//!         .value("https://dummyjson.com".into())
//!         .build();
//!     dbg!(define_ns);
//!     dbg!(define_db);
//!     dbg!(define_user);
//!     dbg!(define_token);
//!     dbg!(define_scope);
//!     dbg!(define_table1);
//!     dbg!(define_table2);
//!     dbg!(define_table3);
//!     dbg!(define_table4);
//!     dbg!(define_table5);
//!     dbg!(define_event);
//!     dbg!(define_func);
//!     dbg!(define_field1);
//!     dbg!(define_field2);
//!     dbg!(define_analyzer1);
//!     dbg!(define_analyzer2);
//!     dbg!(define_index1);
//!     dbg!(define_index2);
//!     dbg!(define_param);
//!     Ok(())
//! }
//! ```
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
pub use index::{DefineIndex,UniqueSearch};



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
    pub fn index<'w>(self) -> DefineIndex<'w> {
        DefineIndex::default()
    }
    /// 该DEFINE PARAM 语句允许您定义可用于每个客户端的全局（数据库范围）参数。
    /// - 必须作为根用户、命名空间用户或数据库用户进行身份验证，才能使用DEFINE PARAM 声明。
    /// - 必须选择命名空间和数据库 才能使用DEFINE PARAM 声明。
    pub fn param<'w>(self) -> DefineParam<'w> {
        DefineParam::default()
    }
    /// 在数据库的上下文中，分析器在文本处理和搜索中起着至关重要的作用。 它由其名称、一组标记器和一组过滤器定义。
    /// 要求
    /// - 在使用DEFINE ANALYZER 声明
    /// - 必须选择命名空间和数据库 然后才能使用DEFINE ANALYZER 声明
    pub fn analyzer<'w>(self) -> DefineAnalyzer<'w>{
        DefineAnalyzer::default()
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
            PwdType::Pwd(pwd) => format!("{} '{}'", PASSWORD, pwd),
            PwdType::Hash(hash) => format!("{} '{}'", PASSHASH, hash),
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