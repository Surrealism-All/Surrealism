use std::fmt::{Display, Formatter};
use except_plugin::{NewFrom, UnSupportedOpException};
use std::str::FromStr;
use serde::{Serialize, Deserialize};

/// # Decimal
/// 指定数值时选择128位十进制数
/// 当Decimal被创建时需要进行检查，若含有非数字则需要panic
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Decimal(String);

impl From<&str> for Decimal {
    fn from(value: &str) -> Self {
        Decimal::from_str(value).unwrap()
    }
}

impl FromStr for Decimal {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut dot = 0;
        for c in s.chars() {
            let c_ascii = c as u8;
            if c_ascii == 46 {
                dot += 1;
                if dot > 1 {
                    return Err("Parse str to Decimal error, cause the str has `.` > 1, please check your input!");
                }
                continue;
            }
            if (c_ascii < 48) || (c_ascii > 57) {
                return Err("Parse str to Decimal error, cause the char is not number, please check your input!");
            }
        }
        Ok(Decimal(s.to_string()))
    }
}


impl Decimal {
    pub fn new(value: &str) -> Self {
        Decimal::from(value)
    }
}

impl Display for Decimal {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}dec", self.0)
    }
}

#[cfg(test)]
mod tests {
    use super::Decimal;

    #[test]
    fn test_decimal1() {
        let d = Decimal::new("889.08");
        assert_eq!(d.to_string(), "889.08dec");
    }

    #[should_panic]
    #[test]
    fn test_decimal2(){
        let d = Decimal::new("88.9.08");
    }

    #[should_panic]
    #[test]
    fn test_decimal3(){
        let d = Decimal::new("88sk");
    }
}