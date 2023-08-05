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

sql_const!((NONE,"NONE") (AFTER,"AFTER") (DIFF,"DIFF") (BEFORE,"BEFORE") (RAND,"rand()") (ULID,"ulid()") (UUID,"uuid()"));

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_const() {
        assert_eq!("AFTER", AFTER);
        assert_eq!("NONE", NONE);
    }
}