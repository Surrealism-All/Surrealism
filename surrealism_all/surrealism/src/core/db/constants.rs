//!
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
    (RETURN,"RETURN") (CONTENT,"CONTENT") (SET,"SET") (CREATE,"CREATE") (BLANK," ") (STMT_END,";")
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