use crate::db::SurrealValue;
// use crate::{crypto_impl};
use super::generate_easy;

/// # Crypto Functions
/// These functions can be used when hashing data, encrypting data, and for securely authenticating users into the database.
/// > note! : you can use service crypto instead of use this directly
/// ## example
/// ```rust
/// use surrealism::db::{ UseNSDB, DefaultInitService, InitService, SurrealValue, parse_response};
/// use surrealism::db::functions::{GenerateCompare, CryptoFunc};
/// use surrealism::surreal::{SurrealismRes,parse_response};
///
/// #[tokio::main]
/// async fn main() -> SurrealismRes<()> {
///     // init service
///     let mut service = DefaultInitService::new().init();
///     // use ns:test and db:test
///     let _ = service.use_commit("test", "test").await?;
///
///     // let c1 = CryptoFunc::md5(SurrealValue::from("surrealdb")).build();
///     // dbg!(c1);
///     // let c2 = CryptoFunc::argon().generate(SurrealValue::from("surrealism")).build();
///     // dbg!(c2);
///
///     // use service to get crypto directly
///     // let crypto1 = service.crypto(CryptoFunc::sha1(SurrealValue::from("surrealism"))).await?;
///     // [tests\src\main.rs:18] res = "91b1fb195acbfea882053f58784354748ee91834"
///     // let res:String = parse_response(crypto1);
///     // dbg!(res);
///
///     // use BCrypt to generate
///     // let crypto2 = service.crypto(CryptoFunc::bcrypt().generate(SurrealValue::from("surreal")).up()).await?;
///     // [tests\src\main.rs:24] res = "$2b$12$kRCndYbUi.N1XHjWcDfNue95k/EvJgAnmHRiVG6oPp9Cg05YOtyDq"
///     // let res: String = parse_response(crypto2);
///     // dbg!(res);
///
///     // compare
///     let compare = service
///         .crypto(CryptoFunc::bcrypt().compare("$2b$12$kRCndYbUi.N1XHjWcDfNue95k/EvJgAnmHRiVG6oPp9Cg05YOtyDq", SurrealValue::from("surreal")).up())
///         .await?;
///     let res: bool = parse_response(compare);
///     dbg!(res);
///
///     Ok(())
/// }
/// ```
#[derive(Debug, Clone)]
pub enum CryptoFunc<'c> {
    Default,
    Md5(SurrealValue),
    Sha1(SurrealValue),
    Sha256(SurrealValue),
    Sha512(SurrealValue),
    Argon2(Argon2<'c>),
    Pbkdf2(Pbkdf2<'c>),
    BCrypt(BCrypt<'c>),
    Scrypt(Scrypt<'c>),
}


pub trait GenerateCompare<'a> {
    fn compare(&self, hash: &'a str, target: SurrealValue) -> Self;
    fn generate(&self, target: SurrealValue) -> Self;
    fn build(&self) -> String;
    /// up lower level struct to CryptoFunc , this function will consume self
    fn up(self) -> CryptoFunc<'a>;
}

impl<'c> Default for CryptoFunc<'c> {
    fn default() -> Self {
        CryptoFunc::Default
    }
}


impl<'c> CryptoFunc<'c> {
    /// crypto::md5() 	Returns the md5 hash of a value
    pub fn md5(value: SurrealValue) -> Self {
        CryptoFunc::Md5(value)
    }
    /// crypto::sha1() 	Returns the sha1 hash of a value
    pub fn sha1(value: SurrealValue) -> Self {
        CryptoFunc::Sha1(value)
    }
    /// crypto::sha256() 	Returns the sha256 hash of a value
    pub fn sha256(value: SurrealValue) -> Self {
        CryptoFunc::Sha256(value)
    }
    /// crypto::sha512() 	Returns the sha512 hash of a value
    pub fn sha512(value: SurrealValue) -> Self {
        CryptoFunc::Sha512(value)
    }
    /// crypto::argon2::compare() 	Compares an argon2 hash to a password
    /// crypto::argon2::generate() 	Generates a new argon2 hashed password
    pub fn argon() -> Argon2<'c> {
        Argon2 {
            compare: None,
            value: SurrealValue::None,
        }
    }
    /// crypto::bcrypt::compare() 	Compares an bcrypt hash to a password
    /// crypto::bcrypt::generate() 	Generates a new bcrypt hashed password
    pub fn bcrypt() -> BCrypt<'c> {
        BCrypt {
            compare: None,
            value: SurrealValue::None,
        }
    }
    /// crypto::pbkdf2::compare() 	Compares an pbkdf2 hash to a password
    /// crypto::pbkdf2::generate() 	Generates a new pbkdf2 hashed password
    pub fn pbkdf2() -> Pbkdf2<'c> {
        Pbkdf2 {
            compare: None,
            value: SurrealValue::None,
        }
    }
    /// crypto::scrypt::compare() 	Compares an scrypt hash to a password
    /// crypto::scrypt::generate() 	Generates a new scrypt hashed password
    pub fn scrypt() -> Scrypt<'c> {
        Scrypt {
            compare: None,
            value: SurrealValue::None,
        }
    }
    pub fn build(&self) -> String {
        match self {
            CryptoFunc::Default => panic!("you should use any kind of Crypto not defalut"),
            CryptoFunc::Md5(md5) => generate_easy("crypto", "md5", md5),
            CryptoFunc::Sha1(sha1) => generate_easy("crypto", "sha1", sha1),
            CryptoFunc::Sha256(sha256) => generate_easy("crypto", "sha256", sha256),
            CryptoFunc::Sha512(sha512) => generate_easy("crypto", "sha512", sha512),
            CryptoFunc::Argon2(argon) => argon.build(),
            CryptoFunc::Pbkdf2(pbkdf2) => pbkdf2.build(),
            CryptoFunc::BCrypt(bcrypt) => bcrypt.build(),
            CryptoFunc::Scrypt(scrypt) => scrypt.build()
        }
    }
    pub fn return_crypto(&self) -> String {
        format!("RETURN {};", self.build())
    }
}


#[derive(Debug, Clone)]
pub struct Argon2<'a> {
    compare: Option<&'a str>,
    value: SurrealValue,
}

impl<'a> GenerateCompare<'a> for Argon2<'a> {
    fn compare(&self, hash: &'a str, target: SurrealValue) -> Self {
        Argon2 {
            compare: Some(hash),
            value: target,
        }
    }

    fn generate(&self, target: SurrealValue) -> Self {
        Argon2 {
            compare: None,
            value: target,
        }
    }

    fn build(&self) -> String {
        match self.compare {
            None => format!("crypto::argon2::generate({})", &self.value.to_str()),
            Some(hash) => format!("crypto::argon2::compare('{}', {})", hash, &self.value.to_str())
        }
    }

    fn up(self) -> CryptoFunc<'a> {
        CryptoFunc::Argon2(self)
    }
}

#[derive(Debug, Clone)]
pub struct Pbkdf2<'a> {
    compare: Option<&'a str>,
    value: SurrealValue,
}

impl<'a> GenerateCompare<'a> for Pbkdf2<'a> {
    fn compare(&self, hash: &'a str, target: SurrealValue) -> Self {
        Pbkdf2 {
            compare: Some(hash),
            value: target,
        }
    }

    fn generate(&self, target: SurrealValue) -> Self {
        Pbkdf2 {
            compare: None,
            value: target,
        }
    }

    fn build(&self) -> String {
        match self.compare {
            None => format!("crypto::pbkdf2::generate({})", &self.value.to_str()),
            Some(hash) => format!("crypto::pbkdf2::compare('{}', {})", hash, &self.value.to_str())
        }
    }

    fn up(self) -> CryptoFunc<'a> {
        CryptoFunc::Pbkdf2(self)
    }
}

#[derive(Debug, Clone)]
pub struct BCrypt<'a> {
    compare: Option<&'a str>,
    value: SurrealValue,
}

impl<'a> GenerateCompare<'a> for BCrypt<'a> {
    fn compare(&self, hash: &'a str, target: SurrealValue) -> Self {
        BCrypt {
            compare: Some(hash),
            value: target,
        }
    }

    fn generate(&self, target: SurrealValue) -> Self {
        BCrypt {
            compare: None,
            value: target,
        }
    }

    fn build(&self) -> String {
        match self.compare {
            None => format!("crypto::bcrypt::generate({})", &self.value.to_str()),
            Some(hash) => format!("crypto::bcrypt::compare('{}', {})", hash, &self.value.to_str())
        }
    }

    fn up(self) -> CryptoFunc<'a> {
        CryptoFunc::BCrypt(self)
    }
}

#[derive(Debug, Clone)]
pub struct Scrypt<'a> {
    compare: Option<&'a str>,
    value: SurrealValue,
}

impl<'a> GenerateCompare<'a> for Scrypt<'a> {
    fn compare(&self, hash: &'a str, target: SurrealValue) -> Self {
        Scrypt {
            compare: Some(hash),
            value: target,
        }
    }

    fn generate(&self, target: SurrealValue) -> Self {
        Scrypt {
            compare: None,
            value: target,
        }
    }

    fn build(&self) -> String {
        match self.compare {
            None => format!("crypto::scrypt::generate({})", &self.value.to_str()),
            Some(hash) => format!("crypto::scrypt::compare('{}', {})", hash, &self.value.to_str())
        }
    }

    fn up(self) -> CryptoFunc<'a> {
        CryptoFunc::Scrypt(self)
    }
}

// crypto_impl!( Scrypt,"crypto::scrypt::generate({})","crypto::scrypt::compare({}, {})");

/// next Version may use to make code clear
#[macro_export]
macro_rules! crypto_impl {
    ($Crypto:tt,$Gen:expr,$Compare:expr) => {
        impl<'a> GenerateCompare<'a> for $Crypto<'a> {

        }
    };
}