use surrealism::{ParamCombine, SurrealismRes, TimeOut, TimeUnit};

#[tokio::main]
async fn main() -> SurrealismRes<()> {
    let timeout = TimeOut::new(56_usize, TimeUnit::MINUTE);
    dbg!(&timeout.timeout());
    dbg!(&timeout.unit());
    dbg!(&timeout);
    dbg!(timeout.combine());
    Ok(())
}