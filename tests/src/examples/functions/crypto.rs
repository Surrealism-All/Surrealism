use surrealism::{SurrealismRes, UseNSDB, DefaultInitService, InitService, SurrealValue, parse_response};
use surrealism::functions::{GenerateCompare, CryptoFunc};

#[tokio::main]
async fn main() -> SurrealismRes<()> {
    // init service
    let mut service = DefaultInitService::new().init();
    // use ns:test and db:test
    let _ = service.use_commit("test", "test").await?;

    // let c1 = CryptoFunc::md5(SurrealValue::from("surrealdb")).build();
    // dbg!(c1);
    // let c2 = CryptoFunc::argon().generate(SurrealValue::from("surrealism")).build();
    // dbg!(c2);

    // use service to get crypto directly
    // let crypto1 = service.crypto(CryptoFunc::sha1(SurrealValue::from("surrealism"))).await?;
    // [tests\src\main.rs:18] res = "91b1fb195acbfea882053f58784354748ee91834"
    // let res:String = parse_response(crypto1);
    // dbg!(res);

    // use BCrypt to generate
    // let crypto2 = service.crypto(CryptoFunc::bcrypt().generate(SurrealValue::from("surreal")).up()).await?;
    // [tests\src\main.rs:24] res = "$2b$12$kRCndYbUi.N1XHjWcDfNue95k/EvJgAnmHRiVG6oPp9Cg05YOtyDq"
    // let res: String = parse_response(crypto2);
    // dbg!(res);

    // compare
    let compare = service
        .crypto(CryptoFunc::bcrypt().compare("$2b$12$kRCndYbUi.N1XHjWcDfNue95k/EvJgAnmHRiVG6oPp9Cg05YOtyDq", SurrealValue::from("surreal")).up())
        .await?;
    let res: bool = parse_response(compare);
    dbg!(res);

    Ok(())
}

