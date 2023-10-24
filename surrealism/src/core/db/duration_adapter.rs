use surrealdb::sql::Duration;
use crate::core::db::SurrealValue;
use super::AdapterToValue;

pub struct DurationAdapter;

impl DurationAdapter {
    /// Create a duration from nanoseconds
    pub fn from_nanos(&self,nanos: u64) -> Duration {
        Duration::from_nanos(nanos)
    }
    /// Create a duration from microseconds
    pub fn from_micros(&self,micros: u64) -> Duration {
        Duration::from_micros(micros)
    }
    /// Create a duration from milliseconds
    pub fn from_millis(&self,millis: u64) -> Duration {
        Duration::from_millis(millis)
    }
    /// Create a duration from seconds
    pub fn from_secs(&self,secs: u64) -> Duration {
        Duration::from_secs(secs)
    }
    /// Create a duration from minutes
    pub fn from_mins(&self,mins: u64) -> Duration {
       Duration::from_mins(mins)
    }
    /// Create a duration from hours
    pub fn from_hours(&self,hours: u64) -> Duration {
       Duration::from_hours(hours)
    }
    /// Create a duration from days
    pub fn from_days(&self,days: u64) -> Duration {
        Duration::from_days(days)
    }
    /// Create a duration from weeks
    pub fn from_weeks(&self,days: u64) -> Duration {
        Duration::from_weeks(days)
    }
}

impl AdapterToValue for Duration{
    fn to_value(self) -> SurrealValue {
        SurrealValue::Duration(self)
    }
}