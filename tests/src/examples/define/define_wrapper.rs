use surrealism::{Condition, ConditionSign, Criteria, CriteriaSign, SurrealismRes, SurrealValue, TimeOut, TimeUnit, ValueConstructor, ValueType};
use surrealism::builder::*;
use surrealism::builder::define::{FieldColumn, OnType, Permissions, PwdType, Schema, TokenType};

// [tests\src\main.rs:51] define1 = "DEFINE NAMESPACE abcum;"
// [tests\src\main.rs:52] define2 = "DEFINE DATABASE app_vitalsense;"
// [tests\src\main.rs:53] define3 = "DEFINE LOGIN username ON DATABASE PASSWORD 123456;"
// [tests\src\main.rs:54] define4 = "DEFINE TOKEN token_name ON DATABASE TYPE HS512 VALUE sNSYneezcr8kqphfOC6NwwraUHJCVAt0XjsRSNmssBaBRh3WyMa9TRfq8ST7fsU2H2kGiOpU4GbAF1bCiXmM1b3JGgleBzz7rsrz6VvYEM4q3CLkcO8CMBIlhwhzWmy8;"
// [tests\src\main.rs:55] define5 = "DEFINE TOKEN token_name ON SCOPE account TYPE HS512 VALUE sNSYneezcr8kqphfOC6NwwraUHJCVAt0XjsRSNmssBaBRh3WyMa9TRfq8ST7fsU2H2kGiOpU4GbAF1bCiXmM1b3JGgleBzz7rsrz6VvYEM4q3CLkcO8CMBIlhwhzWmy8;"
// [tests\src\main.rs:56] define6 = "DEFINE SCOPE account TIMEOUT 24h SIGNUP ( CREATE user SET email = $email, pass = crypto::argon2::generate($pass) ) SIGNIN ( SELECT * FROM user WHERE email = $email AND crypto::argon2::compare(pass,
// $pass) );"
// [tests\src\main.rs:57] define7 = "DEFINE TABLE post SCHEMALESS FOR WHERE published = true OR user = '$auth.id' FOR WHERE user = '$auth.id' FOR WHERE user = '$auth.id' FOR WHERE user = '$auth.id' OR $auth.admin = true;"
// [tests\src\main.rs:58] define8 = "DEFINE TABLE temperatures_by_month SELECT count() AS total,time::month(recorded_at) AS month,math::mean(temperature) AS average_temp FROM reading GROUP BY city;"
// [tests\src\main.rs:59] define9 = "DEFINE EVENT publish_post ON TABLE publish_post WHEN $event = 'CREATE' THEN UPDATE post SET status = \"PUBLISHED\" WHERE id = $after.post_id;"
// [tests\src\main.rs:60] define10 = "DEFINE FUNCTION fn::greet($name:string) {  RETURN \"Hello, \" + $name + \"!\"; };"
// [tests\src\main.rs:61] define11 = "DEFINE FIELD email ON TABLE user TYPE string;"
// [tests\src\main.rs:62] define12 = "DEFINE FIELD countrycode ON TABLE user TYPE string VALUE $value OR 'GBR' ASSERT $value != 'NONE' AND $value = '/[A-Z]{3}/';"
// [tests\src\main.rs:63] define13 = "DEFINE INDEX userEmailIndex ON TABLE user COLUMNS email UNIQUE;"
// [tests\src\main.rs:64] define14 = "DEFINE PARAM $endpointBase VALUE 'https://dummyjson.com';"
#[tokio::main]
async fn main() -> SurrealismRes<()> {
    let define1 = SQLBuilderFactory::define().namespace("abcum").build();
    let define2 = SQLBuilderFactory::define().db("app_vitalsense").build();
    let define3 = SQLBuilderFactory::define()
        .login("username", OnType::DB, PwdType::Pwd("123456")).build();
    let define4 = SQLBuilderFactory::define()
        .token("token_name", OnType::DB, TokenType::HS512, "sNSYneezcr8kqphfOC6NwwraUHJCVAt0XjsRSNmssBaBRh3WyMa9TRfq8ST7fsU2H2kGiOpU4GbAF1bCiXmM1b3JGgleBzz7rsrz6VvYEM4q3CLkcO8CMBIlhwhzWmy8").build();
    let define5 = SQLBuilderFactory::define()
        .token("token_name", OnType::SCOPE("account"), TokenType::HS512, "sNSYneezcr8kqphfOC6NwwraUHJCVAt0XjsRSNmssBaBRh3WyMa9TRfq8ST7fsU2H2kGiOpU4GbAF1bCiXmM1b3JGgleBzz7rsrz6VvYEM4q3CLkcO8CMBIlhwhzWmy8").build();
    let define6 = SQLBuilderFactory::define().scope("account", TimeOut::new(24, TimeUnit::HOUR), "CREATE user SET email = $email, pass = crypto::argon2::generate($pass)", "SELECT * FROM user WHERE email = $email AND crypto::argon2::compare(pass, $pass)").build();
    let define7 = SQLBuilderFactory::define()
        .table("post", false, Some(Schema::SCHEMALESS), None, Some(
            Permissions::FOR {
                select: Condition::new().push(Criteria::new("published", true, CriteriaSign::Eq), ConditionSign::Or).push(Criteria::new("user", "$auth.id", CriteriaSign::Eq), ConditionSign::None).deref_mut(),
                create: Condition::new().push(Criteria::new("user", "$auth.id", CriteriaSign::Eq), ConditionSign::None).deref_mut(),
                update: Condition::new().push(Criteria::new("user", "$auth.id", CriteriaSign::Eq), ConditionSign::None).deref_mut(),
                delete: Condition::new().push(Criteria::new("user", "$auth.id", CriteriaSign::Eq), ConditionSign::Or).push(Criteria::new("$auth.admin", true, CriteriaSign::Eq), ConditionSign::None).deref_mut(),
            }
        )).build();
    let define8 = SQLBuilderFactory::define()
        .table("temperatures_by_month", false, None, Some("SELECT count() AS total,time::month(recorded_at) AS month,math::mean(temperature) AS average_temp FROM reading GROUP BY city"), None).build();


    let define9 = SQLBuilderFactory::define()
        .event("publish_post", "publish_post", Condition::new().push(Criteria::new_event("CREATE", CriteriaSign::Eq), ConditionSign::None).deref_mut(), r#"UPDATE post SET status = "PUBLISHED" WHERE id = $after.post_id"#).build();

    let define10 = SQLBuilderFactory::define()
        .function("greet", vec!["$name:string"], "", r#""Hello, " + $name + "!""#).build();

    let define11 = SQLBuilderFactory::define()
        .field("email", "user", ValueConstructor::new(ValueType::String, None, None), None).build();
    let define12 = SQLBuilderFactory::define()
        .field("countrycode", "user",
               ValueConstructor::new(ValueType::String,
                                     Some(SurrealValue::from("GBR")),
                                     Some(Condition::new()
                                         .push(Criteria::new_field("NONE", CriteriaSign::Neq), ConditionSign::And)
                                         .push(Criteria::new_field("/[A-Z]{3}/", CriteriaSign::Eq), ConditionSign::None).deref_mut())),
               None)
        .build();

    let define13 = SQLBuilderFactory::define().index("userEmailIndex", "user", FieldColumn::COLUMNS(vec!["email"]), true).build();
    let define14 = SQLBuilderFactory::define().param("endpointBase", SurrealValue::from("https://dummyjson.com")).build();

    dbg!(define1);
    dbg!(define2);
    dbg!(define3);
    dbg!(define4);
    dbg!(define5);
    dbg!(define6);
    dbg!(define7);
    dbg!(define8);
    dbg!(define9);
    dbg!(define10);
    dbg!(define11);
    dbg!(define12);
    dbg!(define13);
    dbg!(define14);
    Ok(())
}

