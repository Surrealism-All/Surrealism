use std::str::FromStr;
use surrealdb::sql::Datetime;
use crate::core::db::SurrealValue;
use super::AdapterToValue;

pub struct DatetimeAdapter;

impl DatetimeAdapter {
    pub fn default(&self)->Datetime{
        Datetime::default()
    }
    pub fn from_str(&self,s:&str)->Datetime{
        Datetime::from_str(s).unwrap()
    }
}

impl AdapterToValue for Datetime {
    fn to_value(self) -> SurrealValue {
        SurrealValue::DateTime(self)
    }
}

