use std::path::PathBuf;
use std::time::Duration;
use except_plugin::{EasyException, easy_e, ExceptionFactory, EasyExceptionBuilder, SuperBuilderImpl, CommonParamImpl, ExceptionLevel};
use super::ErrorType;

#[derive(Debug)]
pub struct ConfigParseError(EasyException);

impl ConfigParseError {
    pub fn new(
        msg: &str,
        line: u32,
        path: PathBuf,
    ) -> Self {
        ConfigParseError(
            easy_e!(ErrorType::ConfigParseError.as_u32(),msg,ExceptionLevel::Error,line,path)
        )
    }
}