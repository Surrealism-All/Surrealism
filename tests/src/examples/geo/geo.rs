use surrealism::{DefaultInitService, InitService, SurrealismConnector, SurrealismRes, SurrealValue, UseNSDB, Point, Geometries, SurrealismCommit, ReturnType};
use surrealism::builder::{BaseWrapperImpl, ReturnImpl, SQLBuilderFactory, TableImpl};
use surrealism::builder::create::CreateWrapperImpl;

// [tests\src\main.rs:40] commit1 = Response(
//     {
//         0: Ok(
//             [
//                 Object(
//                     Object(
//         {
//             "center": Geometry(
//                 Point(
//                     Point(
//                         Coord {
//             x: -0.118092,
//             y: 51.509865,
//         },
//             ),
//             ),
//             ),
//             "id": Thing(
//                 Thing {
//             tb: "city",
//             id: String(
//             "fhao6bgiwahmo6x5tiq1",
//             ),
//         },
//             ),
//         },
//         ),
//         ),
//         ],
//         ),
//     },
// )
#[tokio::main]
async fn main() -> SurrealismRes<()> {
    let mut service = DefaultInitService::new().init();
    let _ = service.use_commit("test", "test").await?;
    let point = Geometries::Point(Point::from((-0.118092, 51.509865)));
    let create = SQLBuilderFactory::create().table("city").set()
        .add_from_value("center", SurrealValue::Geometries(point))
        .return_type(ReturnType::After).build();
    let commit1 = service.commit_sql(&create).await?;
    dbg!(commit1);
    Ok(())
}