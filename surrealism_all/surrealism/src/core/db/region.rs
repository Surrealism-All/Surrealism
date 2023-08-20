//!
//! ```txt
//! @author:syf20020816@Outlook.com
//! @date:2023/8/6
//! @version:0.0.1
//! @description:
//! ```
use super::{TIMEOUT};
use crate::{TimeUnit};

pub trait RegionImpl {
    fn new() -> Self;
    fn build(&self) -> String;
}
/// 通用形式的Region
pub struct SqlRegion{
    keyword:String,
    field:RegionField
}

impl RegionImpl for SqlRegion {
    fn new() -> Self {
        SqlRegion{
            keyword: String::new(),
            field: RegionField::Single(String::new())
        }
    }

    fn build(&self) -> String {
        todo!()
    }
}

/// 区域参数
/// Multi:多参数key-value形式。如: SET age = 16 , name = 'Joker'
/// Single:单参数形式。如: CONTENT { name : 'Joker' , age : 16 }
pub enum RegionField{
    Multi(Vec<(String,String)>),
    Single(String)
}



/// 设计TimeOut 使用的 Region
pub struct TimeOutRegion {
    keyword: String,
    timeout: u32,
    unit: TimeUnit,
}

impl TimeOutRegion {
    pub fn from(timeout:u32,unit:TimeUnit)->Self{
        TimeOutRegion{
            keyword: String::from(TIMEOUT),
            timeout,
            unit
        }
    }
    pub fn set_timeout(&mut self,timeout:u32)->&mut Self{
        self.timeout = timeout;
        self
    }
    pub fn set_unit(&mut self,unit:TimeUnit)->&mut Self{
        self.unit = unit;
        self
    }
}

impl RegionImpl for TimeOutRegion {
    fn new() -> Self {
        TimeOutRegion {
            keyword: String::from(TIMEOUT),
            timeout: 0,
            unit: TimeUnit::MILLISECOND,
        }
    }

    fn build(&self) -> String {
        //judge timeout
        // 0 -> "", > 0 -> format
        return if 0_u32.lt(&self.timeout) {
            format!("{} {} {}", &self.keyword, &self.timeout, &self.unit.to_str())
        } else {
            String::new()
        };
    }
}
