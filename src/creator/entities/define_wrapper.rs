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
use crate::handle_str;
use super::{Wrapper, SQLField, SQLRegion, RegionField, TimeUnit, Criteria, COMMON_SEPARATOR, END_SEPARATOR, DEFINE, HOUR, DAY, MINUTE, SECOND, MILLISECOND};

const NAMESPACE: &str = "NAMESPACE";
const DATABASE: &str = "DATABASE";
const SCOPE: &str = "SCOPE";
const LOGIN: &str = "LOGIN";
const ON: &str = "ON";
const PASSWORD: &str = "PASSWORD";
const PASSHASH: &str = "PASSHASH";
const TOKEN: &str = "TOKEN";
const TYPE: &str = "TYPE";
const VALUE: &str = "VALUE";
const SIGNUP: &str = "SIGNUP";
const SIGNIN: &str = "SIGNIN";
const EVENT: &str = "EVENT";
const WHEN: &str = "WHEN";
const THEN: &str = "THEN";
const TABLE: &str = "TABLE";
const FUNCTION: &str = "FUNCTION";
const RETURN: &str = "RETURN";
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

#[derive(Debug, Clone)]
pub struct DefineWrapper {
    keyword: Statements,
    available: SQLRegion,
}

#[derive(Debug, Clone)]
enum AccountType {
    DATABASE,
    NAMESPACE,
    SCOPE,
    NONE,
}

#[derive(Debug, Clone)]
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


impl DefineWrapper {
    /// SurrealDB有一个多租户模型，它允许您将数据库的范围限定到一个名称空间。数据库的数量没有限制 可以在名称空间中，也没有对允许的名称空间的数量的限制。只有root用户有权 创建命名空间。
    /// - 您必须作为root用户进行身份验证，才能使用`DEFINE NAMESPACE`声明。
    pub fn define_namespace(&mut self, namespace: &str) -> DefineNamespace {
        let stmt = format!("{}{}{}{}{}{}", DEFINE, COMMON_SEPARATOR, NAMESPACE, COMMON_SEPARATOR, namespace, END_SEPARATOR);
        let tmp = SQLField::new(NAMESPACE, &stmt);
        self.available.push_set(&tmp);
        let mut tmp_define_namespace = DefineNamespace::new();
        tmp_define_namespace.available.set_region_single(&stmt);
        tmp_define_namespace
    }
    /// 该DEFINE DATABASE 语句使您可以实例化命名数据库，从而可以指定 安全和配置选项。
    /// - 必须以root用户或命名空间用户身份进行身份验证，然后才能使用DEFINE DATABASE 声明。
    /// - 必须选择命名空间 才能使用DEFINE DATABASE 声明。
    pub fn define_database(&mut self, database: &str) -> DefineDatabase {
        let stmt = format!("{}{}{}{}{}{}", DEFINE, COMMON_SEPARATOR, DATABASE, COMMON_SEPARATOR, database, END_SEPARATOR);
        let tmp = SQLField::new(DATABASE, &stmt);
        self.available.push_set(&tmp);
        let mut tmp_define_db = DefineDatabase::new();
        tmp_define_db.available.set_region_single(&stmt);
        tmp_define_db
    }

    ///使用DEFINE LOGIN 语句在SurrealDB上创建用户帐户
    ///- 必须以root或命名空间用户身份进行身份验证，才能使用DEFINE LOGIN 声明。
    ///- 必须以root、命名空间或数据库用户身份进行身份验证，才能使用DEFINE LOGIN 声明。
    ///- 必须选择命名空间和/或数据库 才能使用DEFINE LOGIN 声明。
    /// > 注意：您不能使用DEFINE LOGIN 语句创建根或SCOPE 用户。
    pub fn define_login_namespace(&mut self, username: &str, password: &str) -> DefineLogin {
        let stmt = format!("{} {} {} {} {} {} '{}'{}", DEFINE, LOGIN, username, ON, NAMESPACE, PASSWORD, password, END_SEPARATOR);
        let tmp = SQLField::new(DATABASE, &stmt);
        self.available.push_set(&tmp);
        let mut tmp_define_login = DefineLogin::new();
        tmp_define_login.available.set_region_single(&stmt);
        tmp_define_login
    }
    pub fn define_login_database(&mut self, username: &str, password: &str) -> DefineLogin {
        let stmt = format!("{} {} {} {} {} {} '{}'{}", DEFINE, LOGIN, username, ON, DATABASE, PASSWORD, password, END_SEPARATOR);
        let tmp = SQLField::new(DATABASE, &stmt);
        self.available.push_set(&tmp);
        let mut tmp_define_login = DefineLogin::new();
        tmp_define_login.available.set_region_single(&stmt);
        tmp_define_login
    }
    ///  SurrealDB可以与第三方OAuth提供商合作。假设您的提供者在您的服务通过身份验证后向其发布JWT。 通过使用DEFINE TOKEN 语句，您可以设置验证JWT真实性所需的公钥。
    ///
    /// 您可以指定什么TYPE 您的令牌使用的加密签名算法。支持以下算法：
    /// `EDDSA， ES256， ES384， ES512， HS256， HS384， HS512， PS256， PS384， PS512， RS256， RS384， RS512`
    /// - 到DEFINE TOKEN ... ON NAMESPACE ... 你必须有根或命名空间级别的访问权限。
    /// - 到DEFINE TOKEN ... ON DATABASE ... 必须具有根、命名空间或数据库级别的访问权限。
    /// - 到DEFINE TOKEN ... ON SCOPE ... 必须具有根、命名空间或数据库级别的访问权限。
    /// - 必须选择命名空间和/或数据库 才能使用DEFINE DATABASE 数据库或命名空间标记的语句。
    pub fn define_token_namespace(&mut self, token_name: &str, token_type: TokenType, value: &str) -> DefineToken {
        self.build_token(token_name, token_type, value, NAMESPACE, "")
    }
    pub fn define_token_database(&mut self, token_name: &str, token_type: TokenType, value: &str) -> DefineToken {
        self.build_token(token_name, token_type, value, DATABASE, "")
    }
    pub fn define_token_scope(&mut self, token_name: &str, token_type: TokenType, scope_name: &str, value: &str) -> DefineToken {
        self.build_token(token_name, token_type, value, SCOPE, scope_name)
    }
    fn build_token(&mut self, token_name: &str, token_type: TokenType, value: &str, scope_type: &str, scope_name: &str) -> DefineToken {
        let mut algorithm = "";
        match token_type {
            TokenType::EDDSA => algorithm = EDDSA,
            TokenType::ES256 => algorithm = ES256,
            TokenType::ES384 => algorithm = ES384,
            TokenType::ES512 => algorithm = ES512,
            TokenType::HS256 => algorithm = HS256,
            TokenType::HS384 => algorithm = HS384,
            TokenType::HS512 => algorithm = HS512,
            TokenType::PS256 => algorithm = PS256,
            TokenType::PS384 => algorithm = PS384,
            TokenType::PS512 => algorithm = PS512,
            TokenType::RS256 => algorithm = RS256,
            TokenType::RS384 => algorithm = RS384,
            TokenType::RS512 => algorithm = RS512,
        }
        let mut stmt = String::new();
        if scope_name.is_empty() {
            stmt = format!("{} {} {} {} {} {} {} {} '{}'{}", DEFINE, TOKEN, token_name, ON, scope_type, TYPE, algorithm, VALUE, value, END_SEPARATOR);
        } else {
            stmt = format!("{} {} {} {} {} {} {} {} {} '{}'{}", DEFINE, TOKEN, token_name, ON, scope_type, scope_name, TYPE, algorithm, VALUE, value, END_SEPARATOR);
        }
        let tmp = SQLField::new(TOKEN, &stmt);
        self.available.push_set(&tmp);
        let mut tmp_define_token = DefineToken::new();
        tmp_define_token.available.set_region_single(&stmt);
        tmp_define_token
    }
    /// DEFINE SCOPE @name SESSION @duration SIGNUP @expression SIGNIN @expression
    /// 设置范围访问允许SurrealDB作为Web数据库操作。使用作用域，您可以设置身份验证和访问规则，从而实现对表和字段的细粒度访问。
    /// - 使用DEFINE SCOPE 必须具有根、命名空间或数据库级别的访问权限。
    /// - 必须选择命名空间和数据库 才能使用DEFINE SCOPE 声明。
    pub fn define_scope(&mut self, scope_name: &str, session: usize, unit: TimeUnit, sign_up: &str, sign_in: &str) -> DefineScope {
        self.build_scope(scope_name, session, unit, sign_up, sign_in)
    }
    /// 采用相对固定的语句构建
    /// example:
    /// ```txt
    /// DEFINE SCOPE account SESSION 24h
    /// 	SIGNUP ( CREATE user SET email = $email, pass = crypto::argon2::generate($pass) )
    /// 	SIGNIN ( SELECT * FROM user WHERE email = $email AND crypto::argon2::compare(pass, $pass) )
    /// ;
    /// ```
    pub fn define_scope_bind(&mut self, scope_name: &str, session: usize, unit: TimeUnit, email: &str, password: &str) -> DefineScope {
        let sign_up = format!("CREATE user SET email = {}, pass = crypto::argon2::generate({})", email, password);
        let sign_in = format!("SELECT * FROM user WHERE email = {} AND crypto::argon2::compare(pass, {})", email, password);
        self.build_scope(scope_name, session, unit, &sign_up, &sign_in)
    }
    fn build_scope(&mut self, scope_name: &str, session: usize, unit: TimeUnit, sign_up: &str, sign_in: &str) -> DefineScope {
        let mut time_value = format!("{}", session);
        match unit {
            TimeUnit::MILLISECOND => time_value.push_str(MILLISECOND),
            TimeUnit::SECOND => time_value.push_str(SECOND),
            TimeUnit::MINUTE => time_value.push_str(MINUTE),
            TimeUnit::HOUR => time_value.push_str(HOUR),
            TimeUnit::DAY => time_value.push_str(DAY),
        }
        let stmt = format!("{} {} {} {} {} ( {} ) {} ( {} ){}", DEFINE, SCOPE, scope_name, &time_value, SIGNUP, sign_up, SIGNIN, sign_in, END_SEPARATOR);
        let mut define_scope = DefineScope::new_args(scope_name, &time_value, sign_up, sign_in);
        define_scope.available.set_region_single(&stmt);
        define_scope
    }
    /// 事件可以在对记录中的数据进行任何更改或修改之后触发。每个触发器都能看到 The$before 和/或$after 值，从而为每个触发器启用高级自定义逻辑。
    /// - 必须作为根用户、命名空间用户或数据库用户进行身份验证，才能使用DEFINE EVENT 声明。
    /// - 必须选择命名空间和数据库 才能使用DEFINE EVENT 声明。
    pub fn define_event(&mut self, event_name: &str, table_name: &str, when: &Criteria, then: &str) -> DefineEvent {
        let when_stmt = when.combine();
        let stmt = format!("{} {} {} {} {} {} {} {} {} ( {} ){}", DEFINE, EVENT, event_name, ON, TABLE, table_name, WHEN, &when_stmt, THEN, then, END_SEPARATOR);
        let mut define_event = DefineEvent::new_args(event_name, table_name, &when_stmt, then);
        define_event.available.set_region_single(&stmt);
        define_event
    }
    /// 该DEFINE FUNCTION 语句允许您定义可在整个数据库中重用的自定义函数。
    /// - 必须作为根用户、命名空间用户或数据库用户进行身份验证，才能使用DEFINE FUNCTION 声明。
    /// - 必须选择命名空间和数据库 才能使用DEFINE FUNCTION 声明。
    pub fn define_function(&mut self) -> DefineFunction {
        DefineFunction::new()
    }
}

impl Wrapper for DefineWrapper {
    fn new() -> Self {
        DefineWrapper {
            keyword: Statements::DEFINE,
            available: SQLRegion::new(RegionField::Multi(Vec::new()), DEFINE),
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

/// 构建DEFINE NAMESPACE @name
#[derive(Debug, Clone)]
pub struct DefineNamespace {
    keyword: Statements,
    available: SQLRegion,
}

impl Wrapper for DefineNamespace {
    fn new() -> Self {
        DefineNamespace {
            keyword: Statements::DEFINE,
            available: SQLRegion::new(RegionField::Single(String::new()), DEFINE),
        }
    }

    fn commit(&mut self) -> &str {
        let stmt = self.available.clone();
        self.available.set_region_statement(stmt.get_region_single());
        self.available.get_region_statement()
    }

    fn get_keyword(&self) -> &Statements {
        &self.keyword
    }

    fn get_available(&self) -> &SQLRegion {
        &self.available
    }
}

#[derive(Debug, Clone)]
pub struct DefineDatabase {
    keyword: Statements,
    available: SQLRegion,
}

impl Wrapper for DefineDatabase {
    fn new() -> Self {
        DefineDatabase {
            keyword: Statements::DEFINE,
            available: SQLRegion::new(RegionField::Single(String::new()), DEFINE),
        }
    }

    fn commit(&mut self) -> &str {
        let stmt = self.available.clone();
        self.available.set_region_statement(stmt.get_region_single());
        self.available.get_region_statement()
    }

    fn get_keyword(&self) -> &Statements {
        &self.keyword
    }

    fn get_available(&self) -> &SQLRegion {
        &self.available
    }
}

#[derive(Debug, Clone)]
pub struct DefineLogin {
    keyword: Statements,
    available: SQLRegion,
}

impl Wrapper for DefineLogin {
    fn new() -> Self {
        DefineLogin {
            keyword: Statements::DEFINE,
            available: SQLRegion::new(RegionField::Single(String::new()), DEFINE),
        }
    }

    fn commit(&mut self) -> &str {
        let stmt = self.available.clone();
        self.available.set_region_statement(stmt.get_region_single());
        self.available.get_region_statement()
    }

    fn get_keyword(&self) -> &Statements {
        &self.keyword
    }

    fn get_available(&self) -> &SQLRegion {
        &self.available
    }
}

#[derive(Debug, Clone)]
pub struct DefineToken {
    keyword: Statements,
    available: SQLRegion,
}

impl Wrapper for DefineToken {
    fn new() -> Self {
        DefineToken {
            keyword: Statements::DEFINE,
            available: SQLRegion::new(RegionField::Single(String::new()), DEFINE),
        }
    }

    fn commit(&mut self) -> &str {
        let stmt = self.available.clone();
        self.available.set_region_statement(stmt.get_region_single());
        self.available.get_region_statement()
    }

    fn get_keyword(&self) -> &Statements {
        &self.keyword
    }

    fn get_available(&self) -> &SQLRegion {
        &self.available
    }
}

///其中Scope的signup，signin不是我们应该考虑的
/// 调用者应该自己写
#[derive(Debug, Clone)]
pub struct DefineScope {
    keyword: Statements,
    available: SQLRegion,
    name: String,
    session: String,
    sign_up: String,
    sign_in: String,

}

impl DefineScope {
    fn new_args(name: &str, session: &str, sign_up: &str, sign_in: &str) -> Self {
        DefineScope {
            keyword: Statements::DEFINE,
            available: SQLRegion::new(RegionField::Single(String::new()), DEFINE),
            name: name.to_string(),
            session: session.to_string(),
            sign_up: sign_up.to_string(),
            sign_in: sign_in.to_string(),
        }
    }
}

impl Wrapper for DefineScope {
    fn new() -> Self {
        DefineScope {
            keyword: Statements::DEFINE,
            available: SQLRegion::new(RegionField::Single(String::new()), DEFINE),
            name: "".to_string(),
            session: "".to_string(),
            sign_up: "".to_string(),
            sign_in: "".to_string(),
        }
    }

    fn commit(&mut self) -> &str {
        let stmt = self.available.clone();
        self.available.set_region_statement(stmt.get_region_single());
        self.available.get_region_statement()
    }

    fn get_keyword(&self) -> &Statements {
        &self.keyword
    }

    fn get_available(&self) -> &SQLRegion {
        &self.available
    }
}

#[derive(Debug, Clone)]
pub struct DefineTable {}

///DEFINE EVENT @name ON [ TABLE ] @table WHEN @expression THEN @expression
#[derive(Debug, Clone)]
pub struct DefineEvent {
    keyword: Statements,
    available: SQLRegion,
    name: String,
    table: String,
    when: String,
    then: String,
}

impl DefineEvent {
    fn new_args(name: &str, table: &str, when: &str, then: &str) -> Self {
        DefineEvent {
            keyword: Statements::DEFINE,
            available: SQLRegion::new(RegionField::Single(String::new()), DEFINE),
            name: name.to_string(),
            table: table.to_string(),
            when: when.to_string(),
            then: then.to_string(),
        }
    }
}

impl Wrapper for DefineEvent {
    fn new() -> Self {
        DefineEvent {
            keyword: Statements::DEFINE,
            available: SQLRegion::new(RegionField::Single(String::new()), DEFINE),
            name: "".to_string(),
            table: "".to_string(),
            when: "".to_string(),
            then: "".to_string(),
        }
    }

    fn commit(&mut self) -> &str {
        let stmt = self.available.clone();
        self.available.set_region_statement(stmt.get_region_single());
        self.available.get_region_statement()
    }

    fn get_keyword(&self) -> &Statements {
        &self.keyword
    }

    fn get_available(&self) -> &SQLRegion {
        &self.available
    }
}

#[derive(Debug, Clone)]
pub struct DefineFunction {
    keyword: Statements,
    available: SQLRegion,
    fn_name: String,
    fn_params: Vec<String>,
    fn_content: String,
}

impl DefineFunction {
    pub fn add_name(&mut self, name: &str) -> &mut Self {
        self.fn_name = format!("{}::{}", "fn", name);
        self
    }
    pub fn add_params(&mut self, param_name: &str, param_type: &str) -> &mut Self {
        let param_str = format!("${}:{}", param_name, param_type);
        self.fn_params.push(param_str);
        self
    }
    pub fn add_content(&mut self, content: &str) -> &mut Self {
        self.fn_content = String::from(handle_str(content));
        self
    }
    pub fn get_name(&self) -> &str {
        &self.fn_name
    }
    pub fn get_params(&self) -> &Vec<String> {
        &self.fn_params
    }
    pub fn get_content(&self) -> &str {
        &self.fn_content
    }
}

impl Wrapper for DefineFunction {
    fn new() -> Self {
        DefineFunction {
            keyword: Statements::DEFINE,
            available: SQLRegion::new(RegionField::Single(String::new()), DEFINE),
            fn_name: "".to_string(),
            fn_params: vec![],
            fn_content: "".to_string(),
        }
    }

    fn commit(&mut self) -> &str {
        let tmp = self.available.clone();
        let mut param_str = String::new();
        let params = self.get_params();
        for i in 0..params.len() {
            param_str.push_str(&params[i]);
            if i != params.len() - 1 {
                param_str.push_str(",");
            }
        }
        let stmt = format!("{} {} {} ( {} ) {} {} {}{}", DEFINE, FUNCTION, self.get_name(), param_str, "{", self.get_content(), "}", END_SEPARATOR);
        self.available.set_region_statement(&stmt);
        self.available.get_region_statement()
    }

    fn get_keyword(&self) -> &Statements {
        &self.keyword
    }

    fn get_available(&self) -> &SQLRegion {
        &self.available
    }
}

#[derive(Debug, Clone)]
struct DefineField {}

#[derive(Debug, Clone)]
struct DefineIndex {}

#[derive(Debug, Clone)]
struct DefineParam {}