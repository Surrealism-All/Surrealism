//! # Const for SurrealDB Statement
//! ```txt
//! @author:syf20020816@Outlook.com
//! @date:2023/8/5
//! @version:0.0.1
//! @description:
//! ```

/// Make Global Const Str
/// ```rust
/// pub const NONE: &str = "NONE";
/// pub const AFTER: &str = "AFTER";
/// pub const DIFF: &str = "DIFF";
/// pub const BEFORE: &str = "BEFORE";
/// ```
macro_rules! sql_const {
    ($(($Key:tt,$Value:tt))*)=>(
        $(
            pub const $Key:&str = $Value;
        )*
    )
}

sql_const!(
    (NONE,"NONE") (AFTER,"AFTER") (DIFF,"DIFF") (BEFORE,"BEFORE") (RAND,"rand()") (ULID,"ulid()") (UUID,"uuid()")
    (MILLISECOND,"MILLISECOND") (SECOND,"SECOND") (MINUTE,"MINUTE") (HOUR,"HOUR") ( DAY,"DAY") (TIMEOUT,"TIMEOUT")
    (PARALLEL,"PARALLEL") (NONE_DOWN,"none") (NULL,"NULL") (NULL_DOWN,"null") (EQ,"=") (EQUAL,"==") (LT,"<") (GT,">")
    (LT_EQ,"<=") (GT_EQ,">=") (TRUE_STR,"TRUE") (FALSE_STR,"FALSE") (LEFT_BRACE,"{ ") (RIGHT_BRACE," }") (COMMA," , ")
    (RETURN,"RETURN") (CONTENT,"CONTENT") (SET,"SET") (CREATE,"CREATE") (BLANK," ") (STMT_END,";") (NEQ,"!=") (LINK,"->")
    (GTE,">=") (LTE,"<=") (WHERE,"WHERE") (AND,"AND") (OR,"OR") (MERGE,"MERGE") (PATCH,"PATCH") (ADD_OP,"+=") (MINUS_OP,"-=")
    (PLUS_OP,"*=") (DIVIDE_OP,"/=") (UPDATE,"UPDATE") (DELETE,"DELETE") (INSERT,"INSERT") (INSERT_UPDATE,"ON DUPLICATE KEY UPDATE")
    (VALUES,"VALUES") (INTO,"INTO") (ASC,"ASC") (DESC,"DESC") (ALL,"*") (SPLIT,"SPLIT") (ORDER_BY,"ORDER BY") (GROUP_BY,"GROUP BY")
    (LIMIT,"LIMIT") (START,"START") (SELECT,"SELECT") (FETCH,"FETCH") (FROM,"FROM") (INFO,"INFO FOR") (KV,"KV") (SCOPE,"SCOPE")
    (DB,"DB") (TABLE,"TABLE") (NS,"NS") (BEGIN_TRANSACTION,"BEGIN TRANSACTION;") (CANCEL_TRANSACTION,"CANCEL TRANSACTION;")
    (COMMIT_TRANSACTION,"COMMIT TRANSACTION;") (DEFINE_NS,"DEFINE NAMESPACE") (DEFINE_DB,"DEFINE DATABASE") (DEFINE_LOGIN,"DEFINE LOGIN")
    (NAMESPACE,"NAMESPACE") (DATABASE,"DATABASE") (PASSWORD,"PASSWORD") (PASSHASH,"PASSHASH") (ON,"ON") (DEFINE_TOKEN,"DEFINE TOKEN")
    (DEFINE_SCOPE,"DEFINE SCOPE") (EDDSA,"EDDSA") (ES256,"ES256") (ES384,"ES384") (ES512,"ES512") (HS256,"HS256") (HS384,"HS384")
    (PS256,"PS256") (PS384,"PS384") (PS512,"PS512") (RS256,"RS256") (RS384,"RS384") (RS512,"RS512") (HS512,"HS512") (TYPE,"TYPE")
    (VALUE,"VALUE") (SCHEMA_FULL,"SCHEMAFULL") (SCHEMA_LESS,"SCHEMALESS") (AS,"AS") (PERMISSIONS,"PERMISSIONS") (FOR,"FOR")
    (FULL,"FULL") (SIGN_UP,"SIGNUP") (SIGN_IN,"SIGNIN") (DROP,"DROP") (DEFINE_TABLE,"DEFINE TABLE") (DEFINE_EVENT,"DEFINE EVENT")
    (ON_TABLE,"ON TABLE") (WHEN,"WHEN") (THEN,"THEN") (DEFINE_FUNCTION,"DEFINE FUNCTION") (DEFINE_FIELD,"DEFINE FIELD")
    (FIELDS,"FIELDS") (COLUMNS,"COLUMNS") (DEFINE_INDEX,"DEFINE INDEX") (DEFINE_PARAM,"DEFINE PARAM") (UNIQUE,"UNIQUE")
    (REMOVE_NS,"REMOVE NAMESPACE") (REMOVE_DB,"REMOVE DATABASE") (REMOVE_LOGIN,"REMOVE LOGIN") (REMOVE_TOKEN,"REMOVE TOKEN")
    (REMOVE_SCOPE,"REMOVE SCOPE") (REMOVE_TABLE,"REMOVE TABLE") (REMOVE_EVENT,"REMOVE EVENT") (REMOVE_FN,"REMOVE FUNCTION")
    (REMOVE_FIELD,"REMOVE FIELD") (REMOVE_INDEX,"REMOVE INDEX") (REMOVE_PARAM,"REMOVE PARAM") (RELATE,"RELATE") (LIVE_SELECT,"LIVE SELECT")
    (ANY,"any") (BOOL,"bool") (ARRAY,"array") (DATETIME,"datetime") (DURATION,"duration") (FLOAT,"float") (INT,"int") (NUMBER,"number")
    (STRING,"string") (RECORD,"record()") (GEOMETRY,"geometry") (OBJECT,"object") (DECIMAL,"decimal") (CONTAINS,"CONTAINS") (ROOT,"ROOT")
    (REMOVE_USER,"REMOVE USER") (SHOW,"SHOW CHANGES FOR") (SINCE,"SINCE") (OWNER,"OWNER") (EDITOR,"EDITOR") (VIEWER,"VIEWER") (DEFINE_USER,"DEFINE USER")
    (ROLES,"ROLES") (SET_DOWN,"set") (WITH, "WITH") (WITH_NOINDEX,"WITH NOINDEX") (WITH_INDEX,"WITH INDEX") (EXPLAIN,"EXPLAIN") (YEAR,"YEAR")
    (MICROSECOND,"MICROSECOND") (NANOSECOND,"NANOSECOND")
);

pub const FALSE: bool = false;
pub const TRUE: bool = true;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_const() {
        assert_eq!("AFTER", AFTER);
        assert_eq!("NONE", NONE);
    }
}