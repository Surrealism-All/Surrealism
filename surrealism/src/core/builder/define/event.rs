//! # Define Event
//! ```txt
//! @author:syf20020816@Outlook.com
//! @date:2023/10/31
//! @version:0.0.1
//! @description:
//! ```

#[derive(Debug,Clone)]
pub struct DefineEvent{
    name: & 'w str,
    on: & 'w str,
    when: Condition,
    then: & 'w str,
}