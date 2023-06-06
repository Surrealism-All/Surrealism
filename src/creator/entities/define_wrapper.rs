//! DEFINE语句可用于指定身份验证访问和行为、全局参数、表配置、表事件、模式定义和索引。
//! DEFINE [
//! 	NAMESPACE @name
//! 	| DATABASE @name
//! 	| LOGIN @name ON [ NAMESPACE | DATABASE ] [ PASSWORD @pass | PASSHASH @hash ]
//! 	| TOKEN @name ON [ NAMESPACE | DATABASE | SCOPE @scope ] TYPE @type VALUE @value
//! 	| SCOPE @name
//! 		[ SESSION @duration ]
//! 		[ SIGNUP @expression ]
//! 		[ SIGNIN @expression ]
//! 	| TABLE @name
//! 		[ DROP ]
//! 		[ SCHEMAFULL | SCHEMALESS ]
//! 		[ AS SELECT @projections
//! 			FROM @tables
//! 			[ WHERE @condition ]
//! 			[ GROUP [ BY ] @groups ]
//! 		]
//! 		[ PERMISSIONS [ NONE | FULL
//! 			| FOR select @expression
//! 			| FOR create @expression
//! 			| FOR update @expression
//! 			| FOR delete @expression
//! 		] ]
//! 	| EVENT @name ON [ TABLE ] @table WHEN @expression THEN @expression
//! 	| FIELD @name ON [ TABLE ] @table
//! 		[ TYPE @type ]
//! 		[ VALUE @expression ]
//! 		[ ASSERT @expression ]
//! 		[ PERMISSIONS [ NONE | FULL
//! 			| FOR select @expression
//! 			| FOR create @expression
//! 			| FOR update @expression
//! 			| FOR delete @expression
//! 		] ]
//! 	| INDEX @name ON [ TABLE ] @table [ FIELDS | COLUMNS ] @fields [ UNIQUE ]
//! ]

use crate::creator::entities::Statements;
use super::{Wrapper, SQLField, SQLRegion, RegionField, TimeUnit, COMMON_SEPARATOR, END_SEPARATOR, DEFINE, HOUR, DAY, MINUTE, SECOND, MILLISECOND};

const NAMESPACE: &str = "NAMESPACE";
const DATABASE: &str = "DATABASE";
const SCOPE: &str = "SCOPE";
const LOGIN: &str = "LOGIN";
const ON: &str = "ON";
const PASSWORD: &str = "PASSWORD";
const PASSHASH: &str = "PASSHASH";
const EDDSA: &str = "EDDSA";
const ES256: &str = "ES256";
const ES384: &str = "ES384";
const ES512: &str = "ES512";
const HS256: &str = "HS256";
const HS384: &str = "HS384";
const HS512: &str = "HS512";
const PS256: &str = "PS256";
const PS384: &str = "PS384";
const PS512: &str = "PS512";
const RS256: &str = "RS256";
const RS384: &str = "RS384";
const RS512: &str = "RS512";

pub struct DefineWrapper {
    keyword: Statements,
    available: SQLRegion,
    namespace: String,
    database: String,
    account: Account,
    token: TokenAccount,
    scope: ScopeAccount,
}

struct Account {
    keyword: AccountType,
    username: String,
    password: String,
}

impl Account {
    pub fn new(account_type: AccountType) -> Self {
        Account {
            keyword: account_type,
            username: "".to_string(),
            password: "".to_string(),
        }
    }
    pub fn set_username(&mut self, username: &str) {
        self.username = String::from(username);
    }
    pub fn set_password(&mut self, password: &str) {
        self.password = String::from(password)
    }
    pub fn set_keyword(&mut self, keyword: AccountType) {
        self.keyword = keyword
    }
}

enum AccountType {
    DATABASE,
    NAMESPACE,
    SCOPE,
    NONE,
}

struct TokenAccount {
    token_name: String,
    account_type: AccountType,
    token_type: TokenType,
    value: String,
}

impl TokenAccount {
    pub fn new(account_type: AccountType, token_type: TokenType) -> Self {
        TokenAccount {
            token_name: "".to_string(),
            account_type,
            token_type,
            value: "".to_string(),
        }
    }
    pub fn set_token_name(&mut self, token_name: &str) {
        self.token_name = String::from(token_name);
    }
    pub fn set_account_type(&mut self, account_type: AccountType) {
        self.account_type = account_type
    }
    pub fn set_token_type(&mut self, token_type: TokenType) {
        self.token_type = token_type
    }
    pub fn set_value(&mut self, value: &str) {
        self.value = String::from(value);
    }
}

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
    NONE,
}

struct ScopeAccount {
    scope_name: String,
    session: String,
    sign_up: String,
    sign_in: String,
}

impl ScopeAccount {
    pub fn new() -> Self {
        ScopeAccount {
            scope_name: "".to_string(),
            session: "".to_string(),
            sign_up: "".to_string(),
            sign_in: "".to_string(),
        }
    }
    pub fn set_scope_name(&mut self, name: &str) {
        self.scope_name = String::from(name);
    }
    pub fn set_session(&mut self, session: &str) {
        self.session = String::from(session);
    }
    pub fn set_sign_up(&mut self, sign_up: &str) {
        self.sign_up = String::from(sign_up);
    }
    pub fn set_sign_in(&mut self, sign_in: &str) {
        self.sign_in = String::from(sign_in);
    }
}


impl DefineWrapper {
    /// SurrealDB有一个多租户模型，它允许您将数据库的范围限定到一个名称空间。数据库的数量没有限制 可以在名称空间中，也没有对允许的名称空间的数量的限制。只有root用户有权 创建命名空间。
    /// - 您必须作为root用户进行身份验证，才能使用`DEFINE NAMESPACE`声明。
    pub fn define_namespace(&mut self, namespace: &str) -> &mut Self {
        self.namespace = String::from(namespace);
        self
    }
    /// 该DEFINE DATABASE 语句使您可以实例化命名数据库，从而可以指定 安全和配置选项。
    /// - 必须以root用户或命名空间用户身份进行身份验证，然后才能使用DEFINE DATABASE 声明。
    /// - 必须选择命名空间 才能使用DEFINE DATABASE 声明。
    pub fn define_database(&mut self, database: &str) -> &mut Self {
        self.namespace = String::from(database);
        self
    }

    ///使用DEFINE LOGIN 语句在SurrealDB上创建用户帐户
    ///- 必须以root或命名空间用户身份进行身份验证，才能使用DEFINE LOGIN 声明。
    ///- 必须以root、命名空间或数据库用户身份进行身份验证，才能使用DEFINE LOGIN 声明。
    ///- 必须选择命名空间和/或数据库 才能使用DEFINE LOGIN 声明。
    /// > 注意：您不能使用DEFINE LOGIN 语句创建根或SCOPE 用户。
    pub fn define_login_namespace(&mut self, username: &str, password: &str) -> &mut Self {
        self.account.set_username(username);
        self.account.set_password(password);
        self.account.set_keyword(AccountType::NAMESPACE);
        self
    }
    pub fn define_login_database(&mut self, username: &str, password: &str) -> &mut Self {
        self.account.set_username(username);
        self.account.set_password(password);
        self.account.set_keyword(AccountType::DATABASE);
        self
    }
    ///  SurrealDB可以与第三方OAuth提供商合作。假设您的提供者在您的服务通过身份验证后向其发布JWT。 通过使用DEFINE TOKEN 语句，您可以设置验证JWT真实性所需的公钥。
    ///
    /// 您可以指定什么TYPE 您的令牌使用的加密签名算法。支持以下算法：
    /// `EDDSA， ES256， ES384， ES512， HS256， HS384， HS512， PS256， PS384， PS512， RS256， RS384， RS512`
    /// - 到DEFINE TOKEN ... ON NAMESPACE ... 你必须有根或命名空间级别的访问权限。
    /// - 到DEFINE TOKEN ... ON DATABASE ... 必须具有根、命名空间或数据库级别的访问权限。
    /// - 到DEFINE TOKEN ... ON SCOPE ... 必须具有根、命名空间或数据库级别的访问权限。
    /// - 必须选择命名空间和/或数据库 才能使用DEFINE DATABASE 数据库或命名空间标记的语句。
    pub fn define_token_namespace(&mut self, token_name: &str, token_type: TokenType, value: &str) -> &mut Self {
        self.token.set_token_name(token_name);
        self.token.set_token_type(token_type);
        self.token.set_account_type(AccountType::NAMESPACE);
        self.token.set_value(value);
        self
    }
    pub fn define_token_database(&mut self, token_name: &str, token_type: TokenType, value: &str) -> &mut Self {
        self.token.set_token_name(token_name);
        self.token.set_token_type(token_type);
        self.token.set_account_type(AccountType::DATABASE);
        self.token.set_value(value);
        self
    }
    /// DEFINE SCOPE @name SESSION @duration SIGNUP @expression SIGNIN @expression
    /// 设置范围访问允许SurrealDB作为Web数据库操作。使用作用域，您可以设置身份验证和访问规则，从而实现对表和字段的细粒度访问。
    /// - 使用DEFINE SCOPE 必须具有根、命名空间或数据库级别的访问权限。
    /// - 必须选择命名空间和数据库 才能使用DEFINE SCOPE 声明。
    pub fn define_scope(&mut self, scope_name: &str, session: usize, unit: TimeUnit, sign_up: &str, sign_in: &str) -> &mut Self {
        let mut time_value = format!("{}", session);
        match unit {
            TimeUnit::MILLISECOND => time_value.push_str(MILLISECOND),
            TimeUnit::SECOND => time_value.push_str(SECOND),
            TimeUnit::MINUTE => time_value.push_str(MINUTE),
            TimeUnit::HOUR => time_value.push_str(HOUR),
            TimeUnit::DAY => time_value.push_str(DAY),
        }
        self.scope.set_scope_name(scope_name);
        self.scope.set_session(&time_value);
        self.scope.set_sign_in(sign_in);
        self.scope.set_sign_up(sign_up);
        self
    }
}

impl Wrapper for DefineWrapper {
    fn new() -> Self {
        DefineWrapper {
            keyword: Statements::DEFINE,
            available: SQLRegion::new(RegionField::Multi(Vec::new()), DEFINE),
            namespace: "".to_string(),
            database: "".to_string(),
            account: Account::new(AccountType::NONE),
            token: TokenAccount::new(AccountType::NONE, TokenType::NONE),
            scope: ScopeAccount::new(),
        }
    }

    fn commit(&mut self) -> &str {
        todo!()
    }

    fn get_keyword(&self) -> &Statements {
        &self.keyword
    }

    fn get_available(&self) -> &SQLRegion {
        &self.available
    }
}