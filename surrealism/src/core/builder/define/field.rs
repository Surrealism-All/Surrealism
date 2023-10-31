//! # Define Field
//! ```txt
//! @author:syf20020816@Outlook.com
//! @date:2023/10/31
//! @version:0.0.1
//! @description:
//! ```

#[derive(Debug,Clone)]
pub struct DefineField<'a>{
    name: & 'a str,
    on: & 'a str,
    value: ValueConstructor,
    permissions: Option<Permissions>,
}