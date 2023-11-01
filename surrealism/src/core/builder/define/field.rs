//! # Define Field
//! ```txt
//! @author:syf20020816@Outlook.com
//! @date:2023/10/31
//! @version:0.0.1
//! @description:
//! ```



#[derive(Debug, Clone)]
pub struct DefineField<'a> {
    name: &'a str,
    on:OnType<'a>,
    value: ValueConstructor,
    permissions: Option<Permissions>,

}

impl<'a> Default for DefineField<'a> {
    fn default() -> Self {
        DefineField {
            name: "",
            on: OnType::TABLE(""),
            when: None,
            then: "",
        }
    }
}

impl<'a> DefineField<'a> {
    pub fn new(
        name: &'a str,
        on: OnType<'a>,
        when: Option<Condition>,
        then: &'a str,
    )->Self{
        DefineField{}
    }
}

impl<'a> Display for DefineField<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}