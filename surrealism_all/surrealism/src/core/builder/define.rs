//! # Define Wrapper
//! ```txt
//! @author:syf20020816@Outlook.com
//! @date:2023/8/29
//! @version:0.0.1
//! @description:
//! ```
//!

use std::fmt::{Display, Formatter};
use crate::db::constants::{ROLES, DEFINE_USER, NAMESPACE, DATABASE, PASSHASH, PASSWORD, ROOT, TABLE, DEFINE_DB, DEFINE_NS, DEFINE_LOGIN, DEFINE_SCOPE, STMT_END, ON, TYPE, SCOPE, PS256, PS384, PS512, EDDSA, ES256, ES384, ES512, HS256, HS384, HS512, RS256, RS384, RS512, VALUE, DEFINE_TOKEN, SCHEMA_FULL, SCHEMA_LESS, SIGN_IN, SIGN_UP, DROP, DEFINE_TABLE, BLANK, NONE, FULL, FOR, DEFINE_EVENT, ON_TABLE, WHEN, THEN, DEFINE_FUNCTION, RETURN, DEFINE_FIELD, FIELDS, COLUMNS, DEFINE_INDEX, UNIQUE, DEFINE_PARAM};
use crate::core::db::{Condition, ParamCombine, SurrealValue, TimeOut, ValueConstructor, Role};


/// # DefineWrapper
/// The DEFINE statement can be used to specify authentication access and behaviour, global parameters, table configurations, table events, schema definitions, and indexes.
/// ## example
/// ```rust
/// use surrealism::db::{Condition, ConditionSign, Criteria, CriteriaSign,  SurrealValue, TimeOut, TimeUnit, ValueConstructor, ValueType};
/// use surrealism::builder::*;
/// use surrealism::surreal::SurrealismRes;
/// use surrealism::builder::define::{FieldColumn, OnType, Permissions, PwdType, Schema, TokenType};
///
/// // [tests\src\main.rs:51] define1 = "DEFINE NAMESPACE abcum;"
/// // [tests\src\main.rs:52] define2 = "DEFINE DATABASE app_vitalsense;"
/// // [tests\src\main.rs:53] define3 = "DEFINE LOGIN username ON DATABASE PASSWORD 123456;"
/// // [tests\src\main.rs:54] define4 = "DEFINE TOKEN token_name ON DATABASE TYPE HS512 VALUE sNSYneezcr8kqphfOC6NwwraUHJCVAt0XjsRSNmssBaBRh3WyMa9TRfq8ST7fsU2H2kGiOpU4GbAF1bCiXmM1b3JGgleBzz7rsrz6VvYEM4q3CLkcO8CMBIlhwhzWmy8;"
/// // [tests\src\main.rs:55] define5 = "DEFINE TOKEN token_name ON SCOPE account TYPE HS512 VALUE sNSYneezcr8kqphfOC6NwwraUHJCVAt0XjsRSNmssBaBRh3WyMa9TRfq8ST7fsU2H2kGiOpU4GbAF1bCiXmM1b3JGgleBzz7rsrz6VvYEM4q3CLkcO8CMBIlhwhzWmy8;"
/// // [tests\src\main.rs:56] define6 = "DEFINE SCOPE account TIMEOUT 24h SIGNUP ( CREATE user SET email = $email, pass = crypto::argon2::generate($pass) ) SIGNIN ( SELECT * FROM user WHERE email = $email AND crypto::argon2::compare(pass,
/// // $pass) );"
/// // [tests\src\main.rs:57] define7 = "DEFINE TABLE post SCHEMALESS FOR WHERE published = true OR user = '$auth.id' FOR WHERE user = '$auth.id' FOR WHERE user = '$auth.id' FOR WHERE user = '$auth.id' OR $auth.admin = true;"
/// // [tests\src\main.rs:58] define8 = "DEFINE TABLE temperatures_by_month SELECT count() AS total,time::month(recorded_at) AS month,math::mean(temperature) AS average_temp FROM reading GROUP BY city;"
/// // [tests\src\main.rs:59] define9 = "DEFINE EVENT publish_post ON TABLE publish_post WHEN $event = 'CREATE' THEN UPDATE post SET status = \"PUBLISHED\" WHERE id = $after.post_id;"
/// // [tests\src\main.rs:60] define10 = "DEFINE FUNCTION fn::greet($name:string) {  RETURN \"Hello, \" + $name + \"!\"; };"
/// // [tests\src\main.rs:61] define11 = "DEFINE FIELD email ON TABLE user TYPE string;"
/// // [tests\src\main.rs:62] define12 = "DEFINE FIELD countrycode ON TABLE user TYPE string VALUE $value OR 'GBR' ASSERT $value != 'NONE' AND $value = '/[A-Z]{3}/';"
/// // [tests\src\main.rs:63] define13 = "DEFINE INDEX userEmailIndex ON TABLE user COLUMNS email UNIQUE;"
/// // [tests\src\main.rs:64] define14 = "DEFINE PARAM $endpointBase VALUE 'https://dummyjson.com';"
/// #[tokio::main]
/// async fn main() -> SurrealismRes<()> {
///     let define1 = SQLBuilderFactory::define().namespace("abcum").build();
///     let define2 = SQLBuilderFactory::define().db("app_vitalsense").build();
///     let define3 = SQLBuilderFactory::define()
///         .login("username", OnType::DB, PwdType::Pwd("123456")).build();
///     let define4 = SQLBuilderFactory::define()
///         .token("token_name", OnType::DB, TokenType::HS512, "sNSYneezcr8kqphfOC6NwwraUHJCVAt0XjsRSNmssBaBRh3WyMa9TRfq8ST7fsU2H2kGiOpU4GbAF1bCiXmM1b3JGgleBzz7rsrz6VvYEM4q3CLkcO8CMBIlhwhzWmy8").build();
///     let define5 = SQLBuilderFactory::define()
///         .token("token_name", OnType::SCOPE("account"), TokenType::HS512, "sNSYneezcr8kqphfOC6NwwraUHJCVAt0XjsRSNmssBaBRh3WyMa9TRfq8ST7fsU2H2kGiOpU4GbAF1bCiXmM1b3JGgleBzz7rsrz6VvYEM4q3CLkcO8CMBIlhwhzWmy8").build();
///     let define6 = SQLBuilderFactory::define().scope("account", TimeOut::new(24, TimeUnit::HOUR), "CREATE user SET email = $email, pass = crypto::argon2::generate($pass)", "SELECT * FROM user WHERE email = $email AND crypto::argon2::compare(pass, $pass)").build();
///     let define7 = SQLBuilderFactory::define()
///         .table("post", false, Some(Schema::SCHEMALESS), None, Some(
///             Permissions::FOR {
///                 select: Condition::new().push(Criteria::new("published", true, CriteriaSign::Eq), ConditionSign::Or).push(Criteria::new("user", "$auth.id", CriteriaSign::Eq), ConditionSign::None).deref_mut(),
///                 create: Condition::new().push(Criteria::new("user", "$auth.id", CriteriaSign::Eq), ConditionSign::None).deref_mut(),
///                 update: Condition::new().push(Criteria::new("user", "$auth.id", CriteriaSign::Eq), ConditionSign::None).deref_mut(),
///                 delete: Condition::new().push(Criteria::new("user", "$auth.id", CriteriaSign::Eq), ConditionSign::Or).push(Criteria::new("$auth.admin", true, CriteriaSign::Eq), ConditionSign::None).deref_mut(),
///             }
///         )).build();
///     let define8 = SQLBuilderFactory::define()
///         .table("temperatures_by_month", false, None, Some("SELECT count() AS total,time::month(recorded_at) AS month,math::mean(temperature) AS average_temp FROM reading GROUP BY city"), None).build();
///
///
///     let define9 = SQLBuilderFactory::define()
///         .event("publish_post", "publish_post", Condition::new().push(Criteria::new_event("CREATE", CriteriaSign::Eq), ConditionSign::None).deref_mut(), r#"UPDATE post SET status = "PUBLISHED" WHERE id = $after.post_id"#).build();
///
///     let define10 = SQLBuilderFactory::define()
///         .function("greet", vec!["$name:string"], "", r#""Hello, " + $name + "!""#).build();
///     let define13 = SQLBuilderFactory::define().index("userEmailIndex", "user", FieldColumn::COLUMNS(vec!["email"]), true).build();
///     let define14 = SQLBuilderFactory::define().param("endpointBase", SurrealValue::from("https://dummyjson.com")).build();
///
///     Ok(())
/// }
/// ```
#[derive(Debug, Clone)]
pub enum DefineWrapper<'w> {
    NONE,
    NAMESPACE(&'w str),
    DATABASE(&'w str),
    /// DEFINE LOGIN @name ON [ NAMESPACE | DATABASE ] [ PASSWORD @pass | PASSHASH @hash ]
    LOGIN {
        name: &'w str,
        on: OnType<'w>,
        pwd: PwdType<'w>,
    },
    /// DEFINE TOKEN @name ON [ NAMESPACE | DATABASE | SCOPE @scope ] TYPE @type VALUE @value
    TOKEN {
        name: &'w str,
        on: OnType<'w>,
        token_type: TokenType,
        value: &'w str,
    },
    /// DEFINE SCOPE @name SESSION @duration SIGNUP @expression SIGNIN @expression
    SCOPE {
        name: &'w str,
        session: TimeOut,
        sign_up: &'w str,
        sign_in: &'w str,
    },
    TABLE {
        name: &'w str,
        drop: bool,
        schema: Option<Schema>,
        as_expression: Option<&'w str>,
        permissions: Option<Permissions>,
    },
    /// DEFINE EVENT @name ON [ TABLE ] @table WHEN @expression THEN @expression
    EVENT {
        name: &'w str,
        on: &'w str,
        when: Condition,
        then: &'w str,
    },
    FUNCTION {
        name: &'w str,
        args: Vec<&'w str>,
        query: &'w str,
        returned: &'w str,
    },
    FIELD {
        name: &'w str,
        on: &'w str,
        value: ValueConstructor,
        permissions: Option<Permissions>,
    },
    /// DEFINE INDEX @name ON [ TABLE ] @table [ FIELDS | COLUMNS ] @fields [ UNIQUE ]
    INDEX {
        name: &'w str,
        on: &'w str,
        field_column: FieldColumn<'w>,
        unique: bool,
    },
    /// DEFINE PARAM $@name VALUE @value;
    PARAM {
        name: &'w str,
        value: SurrealValue,
    },
    USER {
        username: &'w str,
        on: OnType<'w>,
        password: PwdType<'w>,
        role: Role,
    },
}

impl<'w> DefineWrapper<'w> {
    pub fn new() -> Self {
        DefineWrapper::NONE
    }
    pub fn user(&self, username: &'w str, password: PwdType<'w>, on: OnType<'w>, role: Role) -> Self {
        if on.is_scope() { panic!("OnType::SCOPE can not be used in Define User") }
        if on.is_table() { panic!("OnType::TABLE can not be used in Define User") }
        DefineWrapper::USER {
            username,
            on,
            password,
            role,
        }
    }
    /// SurrealDB有一个多租户模型，它允许您将数据库的范围限定到一个名称空间。数据库的数量没有限制 可以在名称空间中，也没有对允许的名称空间的数量的限制。只有root用户有权 创建命名空间。
    /// - 您必须作为root用户进行身份验证，才能使用`DEFINE NAMESPACE`声明。
    pub fn namespace(&self, ns: &'w str) -> Self {
        DefineWrapper::NAMESPACE(ns)
    }
    /// 该DEFINE DATABASE 语句使您可以实例化命名数据库，从而可以指定 安全和配置选项。
    /// - 必须以root用户或命名空间用户身份进行身份验证，然后才能使用DEFINE DATABASE 声明。
    /// - 必须选择命名空间 才能使用DEFINE DATABASE 声明。
    pub fn db(&self, db: &'w str) -> Self {
        DefineWrapper::DATABASE(db)
    }
    ///使用DEFINE LOGIN 语句在SurrealDB上创建用户帐户
    ///- 必须以root或命名空间用户身份进行身份验证，才能使用DEFINE LOGIN 声明。
    ///- 必须以root、命名空间或数据库用户身份进行身份验证，才能使用DEFINE LOGIN 声明。
    ///- 必须选择命名空间和/或数据库 才能使用DEFINE LOGIN 声明。
    /// > 注意：您不能使用DEFINE LOGIN 语句创建根或SCOPE 用户。
    pub fn login(&self, name: &'w str, on: OnType<'w>, pwd: PwdType<'w>) -> Self {
        return if !on.is_scope() {
            DefineWrapper::LOGIN {
                name,
                on,
                pwd,
            }
        } else {
            panic!("OnType::SCOPE use in Define Token")
        };
    }
    ///  SurrealDB可以与第三方OAuth提供商合作。假设您的提供者在您的服务通过身份验证后向其发布JWT。 通过使用DEFINE TOKEN 语句，您可以设置验证JWT真实性所需的公钥。
    ///
    /// 您可以指定什么TYPE 您的令牌使用的加密签名算法。支持以下算法：
    /// `EDDSA， ES256， ES384， ES512， HS256， HS384， HS512， PS256， PS384， PS512， RS256， RS384， RS512`
    /// - 到DEFINE TOKEN ... ON NAMESPACE ... 你必须有根或命名空间级别的访问权限。
    /// - 到DEFINE TOKEN ... ON DATABASE ... 必须具有根、命名空间或数据库级别的访问权限。
    /// - 到DEFINE TOKEN ... ON SCOPE ... 必须具有根、命名空间或数据库级别的访问权限。
    /// - 必须选择命名空间和/或数据库 才能使用DEFINE DATABASE 数据库或命名空间标记的语句。
    pub fn token(&self, name: &'w str, on: OnType<'w>, token_type: TokenType, value: &'w str) -> Self {
        DefineWrapper::TOKEN {
            name,
            on,
            token_type,
            value,
        }
    }
    pub fn scope(&self, name: &'w str, session: TimeOut, sign_up: &'w str, sign_in: &'w str) -> Self {
        DefineWrapper::SCOPE {
            name,
            session,
            sign_up,
            sign_in,
        }
    }
    /// 该DEFINE TABLE 语句允许您按名称声明表，从而可以应用严格的 控件添加到表的架构中，方法是将SCHEMAFULL，创建外部表视图，并设置权限 指定可以在字段上执行什么操作。
    /// - 必须作为根用户、命名空间用户或数据库用户进行身份验证，才能使用DEFINE TABLE 声明。
    /// - 必须选择命名空间和数据库 才能使用DEFINE TABLE 声明。
    pub fn table(&self, name: &'w str, drop: bool, schema: Option<Schema>, as_expression: Option<&'w str>, permissions: Option<Permissions>) -> Self {
        DefineWrapper::TABLE {
            name,
            drop,
            schema,
            as_expression,
            permissions,
        }
    }
    /// 事件可以在对记录中的数据进行任何更改或修改之后触发。每个触发器都能看到 The$before 和/或$after 值，从而为每个触发器启用高级自定义逻辑。
    /// - 必须作为根用户、命名空间用户或数据库用户进行身份验证，才能使用DEFINE EVENT 声明。
    /// - 必须选择命名空间和数据库 才能使用DEFINE EVENT 声明。
    pub fn event(&self, name: &'w str, on: &'w str, when: Condition, then: &'w str) -> Self {
        DefineWrapper::EVENT {
            name,
            on,
            when,
            then,
        }
    }
    /// 该DEFINE FUNCTION 语句允许您定义可在整个数据库中重用的自定义函数。
    /// - 必须作为根用户、命名空间用户或数据库用户进行身份验证，才能使用DEFINE FUNCTION 声明。
    /// - 必须选择命名空间和数据库 才能使用DEFINE FUNCTION 声明。
    pub fn function(&self, name: &'w str, args: Vec<&'w str>, query: &'w str, returned: &'w str) -> Self {
        DefineWrapper::FUNCTION {
            name,
            args,
            query,
            returned,
        }
    }
    /// 该DEFINE FIELD 语句允许您实例化表中的命名字段，使您能够设置 字段的数据类型、设置默认值、应用断言以保护数据一致性以及设置权限 指定可以在字段上执行什么操作。
    /// - 必须作为根用户、命名空间用户或数据库用户进行身份验证，才能使用DEFINE FIELD 声明。
    /// - 必须选择命名空间和数据库 才能使用DEFINE FIELD 声明。
    pub fn field(&self, name: &'w str, on: &'w str, value: ValueConstructor, permissions: Option<Permissions>) -> Self {
        DefineWrapper::FIELD {
            name,
            on,
            value,
            permissions,
        }
    }
    /// 就像在其他数据库中一样，SurrealDB使用索引来帮助优化查询性能。索引可以包括 表中的一个或多个字段，并且可以强制唯一性约束。如果您不希望索引具有 唯一性约束，则为索引选择的字段应具有高度的基数， 这意味着在索引表记录中的数据之间存在大量的多样性。
    /// - 必须作为根用户、命名空间用户或数据库用户进行身份验证，才能使用DEFINE INDEX 声明。
    /// - 必须选择命名空间和数据库 才能使用DEFINE INDEX 声明。
    pub fn index(&self, name: &'w str, on: &'w str, field_column: FieldColumn<'w>, unique: bool) -> Self {
        DefineWrapper::INDEX {
            name,
            on,
            field_column,
            unique,
        }
    }
    /// 该DEFINE PARAM 语句允许您定义可用于每个客户端的全局（数据库范围）参数。
    /// - 必须作为根用户、命名空间用户或数据库用户进行身份验证，才能使用DEFINE PARAM 声明。
    /// - 必须选择命名空间和数据库 才能使用DEFINE PARAM 声明。
    pub fn param(&self, name: &'w str, value: SurrealValue) -> Self {
        DefineWrapper::PARAM {
            name,
            value,
        }
    }
    pub fn build(&self) -> String {
        match self {
            DefineWrapper::NONE => panic!("DEFINE NONE is not allowed!"),
            DefineWrapper::NAMESPACE(ns) => format!("{} {}{}", DEFINE_NS, ns, STMT_END),
            DefineWrapper::DATABASE(db) => format!("{} {}{}", DEFINE_DB, db, STMT_END),
            DefineWrapper::USER { username, on, password, role }
            => format!("{} {} {} {} {} {} {}{}", DEFINE_USER, username, ON, on.to_string(), password.to_string(), ROLES, role.to_string(), STMT_END),
            DefineWrapper::LOGIN {
                name, on, pwd
            } => format!("{} {} {} {} {}{}", DEFINE_LOGIN, name, ON, on.to_string(), pwd.to_string(), STMT_END),
            DefineWrapper::TOKEN { name, on, token_type, value } =>
                format!("{} {} {} {} {} {} {}{}", DEFINE_TOKEN, name, ON, on.to_string(), token_type.to_string(), VALUE, value, STMT_END),
            DefineWrapper::SCOPE { name, session, sign_up, sign_in } =>
                format!("{} {} {} {} ( {} ) {} ( {} ){}", DEFINE_SCOPE, name, session.combine(), SIGN_UP, sign_up, SIGN_IN, sign_in, STMT_END),
            DefineWrapper::TABLE { name, drop, schema, as_expression, permissions } => {
                let mut res = format!("{} {}", DEFINE_TABLE, name);
                if *drop {
                    res.push_str(BLANK);
                    res.push_str(DROP);
                }
                if schema.is_some() {
                    res.push_str(BLANK);
                    res.push_str(schema.as_ref().unwrap().to_string().as_str())
                }
                if as_expression.is_some() {
                    res.push_str(" AS ");
                    res.push_str(as_expression.as_ref().unwrap())
                }
                if permissions.is_some() {
                    res.push_str(BLANK);
                    res.push_str(&permissions.as_ref().unwrap().to_string())
                }
                res.push_str(STMT_END);
                res
            }
            DefineWrapper::EVENT { name, on, when, then } =>
                format!("{} {} {} {} {} {} {} {}{}", DEFINE_EVENT, name, ON_TABLE, on, WHEN, when.build(), THEN, then, STMT_END),
            DefineWrapper::FUNCTION { name, args, query, returned } =>
                format!("{} fn::{}({}){} {} {} {}{} {}{}", DEFINE_FUNCTION, name, args.join(" , "), "{", query, RETURN, returned, STMT_END, "}", STMT_END),
            DefineWrapper::FIELD { name, on, value, permissions } => {
                let mut res = format!("{} {} {} {} {}", DEFINE_FIELD, name, ON_TABLE, on, value.build());
                if permissions.is_some() {
                    res.push_str(BLANK);
                    res.push_str(&permissions.as_ref().unwrap().to_string())
                }
                res.push_str(STMT_END);
                res
            }
            DefineWrapper::INDEX { name, on, field_column, unique } => {
                let mut res = format!("{} {} {} {} {}", DEFINE_INDEX, name, ON_TABLE, on, field_column.to_string());
                if *unique {
                    res.push_str(BLANK);
                    res.push_str(UNIQUE);
                }
                res.push_str(STMT_END);
                res
            }
            DefineWrapper::PARAM { name, value } => format!("{} ${} {} {}{}", DEFINE_PARAM, name, VALUE, value.to_string(), STMT_END)
        }
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

impl Display for TokenType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        // let mut res = format!("{} ", TYPE);
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

impl<'f> Display for FieldColumn<'f> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let res = match self {
            FieldColumn::FIELDS(f) => format!("{} {}", FIELDS, f.join(" , ")),
            FieldColumn::COLUMNS(c) => format!("{} {}", COLUMNS, c.join(" , ")),
        };
        write!(f, "{}", res)
    }
}