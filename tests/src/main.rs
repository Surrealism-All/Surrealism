use surrealism::DefaultRes;
use surrealism::builder::{BaseWrapperImpl, SQLBuilderFactory, TableImpl};
use surrealism::builder::define::{FieldColumn, OnType, PwdType, TokenType, UniqueSearch};
use surrealism::builder::select::SelectWrapperImpl;
use surrealism::db::{Condition, ConditionSign, Criteria, CriteriaSign, Roles, TimeUnit, ValueConstructor, ValueType};

// [tests\src\main.rs:98] define_ns = "DEFINE NAMESPACE test;"
// [tests\src\main.rs:99] define_db = "DEFINE NAMESPACE test;"
// [tests\src\main.rs:100] define_user = "DEFINE USER username ON ROOT PASSWORD '123456' ROLES OWNER;"
// [tests\src\main.rs:101] define_token.rs = "DEFINE TOKEN test_tk ON NAMESPACE TYPE HS512 VALUE 'sNSYneezcr8kqphfOC6NwwraUHJCVAt0XjsRSNmssBaBRh3WyMa9TRfq8ST7fsU2H2kGiOpU4GbAF1bCiXmM1b3JGgleBzz7rsrz6VvYEM4q3CLkcO8CMBIlhwhzWmy8';"
// [tests\src\main.rs:102] define_scope = "DEFINE SCOPE account TIMEOUT 24h SIGNUP ( CREATE user SET email = $email, pass = crypto::argon2::generate($pass) ) SIGNIN ( SELECT * FROM user WHERE email = $email AND crypto::argon2::compare(pass, $pass) );"
// [tests\src\main.rs:103] define_table1 = "DEFINE TABLE reading;"
// [tests\src\main.rs:104] define_table2 = "DEFINE TABLE reading DROP;"
// [tests\src\main.rs:105] define_table3 = "DEFINE TABLE reading CHANGEFEED 1d;"
// [tests\src\main.rs:106] define_table4 = "DEFINE TABLE  SCHEMAFULL;"
// [tests\src\main.rs:107] define_table5 = "DEFINE TABLE temperatures_by_month AS SELECT count() AS total FROM reading GROUP BY city;"
// [tests\src\main.rs:108] define_event = "DEFINE EVENT  ON TABLE TABLE user WHEN 1@mail.com != '2@mail.com' THEN CREATE event SET user = $value.id, time = time::now(), value = $after.email, action = 'email_changed';"
// [tests\src\main.rs:109] define_func = "DEFINE FUNCTION fn::great($name:string) {  'Hello, '+$name + '!' };"
// [tests\src\main.rs:110] define_field1 = "DEFINE FIELD email ON TABLE TABLE user TYPE string;"
// [tests\src\main.rs:111] define_field2 = "DEFINE FIELD locked ON TABLE TABLE user FLEXIBLE TYPE bool DEFAULT true VALUE true;"
// [tests\src\main.rs:112] define_analyzer1 = "DEFINE ANALYZER example_camel TOKENIZERS camel;"
// [tests\src\main.rs:113] define_analyzer2 = "DEFINE ANALYZER code TOKENIZERS class, camel FILTERS lowercase, ascii;"
// [tests\src\main.rs:114] define_index1 = "DEFINE INDEX userEmailIndex ON TABLE user COLUMNS email UNIQUE;"
// [tests\src\main.rs:115] define_index2 = "DEFINE INDEX userEmailIndex ON TABLE user COLUMNS name SEARCH ANALYZER ascii BM25  HIGHLIGHTS;"
// [tests\src\main.rs:116] define_param = "DEFINE PARAM $endpointBase VALUE 'https://dummyjson.com';"
#[tokio::main]
async fn main() -> DefaultRes<()> {
    let define_ns = SQLBuilderFactory::define().ns().name("test").to_string();
    let define_db = SQLBuilderFactory::define().db().name("test").to_string();
    let define_user = SQLBuilderFactory::define()
        .user()
        .name("username")
        .on(OnType::ROOT)
        .pwd(PwdType::Pwd("123456"))
        .role(Roles::OWNER)
        .build();
    let define_token = SQLBuilderFactory::define().token()
        .name("test_tk")
        .on(OnType::NS)
        .token_type(TokenType::HS512)
        .value("sNSYneezcr8kqphfOC6NwwraUHJCVAt0XjsRSNmssBaBRh3WyMa9TRfq8ST7fsU2H2kGiOpU4GbAF1bCiXmM1b3JGgleBzz7rsrz6VvYEM4q3CLkcO8CMBIlhwhzWmy8")
        .build();
    let define_scope = SQLBuilderFactory::define().scope()
        .name("account")
        .session(24, TimeUnit::HOUR)
        .sign_up("CREATE user SET email = $email, pass = crypto::argon2::generate($pass)")
        .sign_in("SELECT * FROM user WHERE email = $email AND crypto::argon2::compare(pass, $pass)")
        .build();
    let define_table1 = SQLBuilderFactory::define().table()
        .name("reading")
        .build();
    let define_table2 = SQLBuilderFactory::define().table()
        .name("reading")
        .drop()
        .build();
    let define_table3 = SQLBuilderFactory::define().table()
        .name("reading")
        .changefeed(1, TimeUnit::DAY)
        .build();
    let define_table4 = SQLBuilderFactory::define().table()
        .schemafull()
        .build();
    let as_select = SQLBuilderFactory::select().table("reading").column("count()", Some("total")).group_by(vec!["city"]).to_string();
    let define_table5 = SQLBuilderFactory::define().table()
        .name("temperatures_by_month")
        .as_expression(&as_select)
        .build();
    let define_event = SQLBuilderFactory::define().event()
        .on("user")
        .when(Condition::new().push(Criteria::new("1@mail.com", "2@mail.com", CriteriaSign::Neq), ConditionSign::None).deref_mut())
        .then("CREATE event SET user = $value.id, time = time::now(), value = $after.email, action = 'email_changed'")
        .build();
    let define_func = SQLBuilderFactory::define().function()
        .name("great")
        .args(vec!["$name:string"])
        .returned("'Hello, '+$name + '!'")
        .build();
    let define_field1 = SQLBuilderFactory::define().field()
        .name("email")
        .on("user")
        .value(ValueConstructor::new(ValueType::String, None, None, None, false))
        .build();

    let define_field2 = SQLBuilderFactory::define().field()
        .name("locked")
        .on("user")
        .value(ValueConstructor::new(ValueType::Bool, Some(true.into()), Some(true.into()), None, true))
        .build();
    let define_analyzer1 = SQLBuilderFactory::define().analyzer()
        .name("example_camel")
        .tokenizer("camel")
        .build();
    let define_analyzer2 = SQLBuilderFactory::define().analyzer()
        .name("code")
        .tokenizer("class")
        .tokenizer("camel")
        .filter("lowercase")
        .filter("ascii")
        .build();
    let define_index1 = SQLBuilderFactory::define().index()
        .name("userEmailIndex")
        .on("user")
        .field_column(FieldColumn::COLUMNS(vec!["email"]))
        .unique_search(UniqueSearch::Unique)
        .build();
    let define_index2 = SQLBuilderFactory::define().index()
        .name("userEmailIndex")
        .on("user")
        .field_column(FieldColumn::COLUMNS(vec!["name"]))
        .unique_search(UniqueSearch::search("ascii").push("", "").highlight().clone())
        .build();
    let define_param = SQLBuilderFactory::define().param()
        .name("endpointBase")
        .value("https://dummyjson.com".into())
        .build();
    dbg!(define_ns);
    dbg!(define_db);
    dbg!(define_user);
    dbg!(define_token);
    dbg!(define_scope);
    dbg!(define_table1);
    dbg!(define_table2);
    dbg!(define_table3);
    dbg!(define_table4);
    dbg!(define_table5);
    dbg!(define_event);
    dbg!(define_func);
    dbg!(define_field1);
    dbg!(define_field2);
    dbg!(define_analyzer1);
    dbg!(define_analyzer2);
    dbg!(define_index1);
    dbg!(define_index2);
    dbg!(define_param);
    Ok(())
}